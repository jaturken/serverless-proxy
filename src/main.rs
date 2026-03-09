use lambda_http::{http::StatusCode, run, service_fn, Body, Error, Request, Response};
use std::io::Read;

/// Headers that must be stripped before forwarding to the target.
const STRIPPED_HEADERS: &[&str] = &["x-proxy-auth", "x-target-url", "x-target-method", "host"];

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Task 2.1: Read PROXY_AUTH_SECRET from environment at startup; panic if missing or empty.
    let secret = std::env::var("PROXY_AUTH_SECRET")
        .expect("PROXY_AUTH_SECRET environment variable must be set");
    if secret.is_empty() {
        panic!("PROXY_AUTH_SECRET environment variable must not be empty");
    }

    run(service_fn(move |event: Request| {
        let secret = secret.clone();
        async move { handler(event, &secret) }
    }))
    .await
}

fn handler(event: Request, secret: &str) -> Result<Response<Body>, Error> {
    // --- Task 2.2 / 2.3: Authentication ---
    let auth_header = event
        .headers()
        .get("x-proxy-auth")
        .and_then(|v| v.to_str().ok());

    match auth_header {
        Some(value) if constant_time_eq(value.as_bytes(), secret.as_bytes()) => {}
        _ => {
            return Ok(Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(Body::Text("Unauthorized".into()))
                .unwrap());
        }
    }

    // --- Task 3.1: Extract and validate X-Target-URL ---
    let target_url = match event
        .headers()
        .get("x-target-url")
        .and_then(|v| v.to_str().ok())
    {
        Some(url) => url.to_string(),
        None => {
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::Text("Missing X-Target-URL header".into()))
                .unwrap());
        }
    };

    // --- Task 3.2: Extract and validate X-Target-Method ---
    let target_method = match event
        .headers()
        .get("x-target-method")
        .and_then(|v| v.to_str().ok())
    {
        Some(method) => method.to_uppercase(),
        None => {
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::Text("Missing X-Target-Method header".into()))
                .unwrap());
        }
    };

    // --- Task 3.3: Build header filtering logic ---
    let mut filtered_headers: Vec<(String, String)> = Vec::new();
    for (name, value) in event.headers().iter() {
        let name_lower = name.as_str().to_lowercase();
        if STRIPPED_HEADERS.contains(&name_lower.as_str()) {
            continue;
        }
        if let Ok(v) = value.to_str() {
            filtered_headers.push((name.to_string(), v.to_string()));
        }
    }

    // --- Task 3.4: Forward request to target using ureq ---
    let request_body = match event.body() {
        Body::Empty => Vec::new(),
        Body::Text(text) => text.as_bytes().to_vec(),
        Body::Binary(bytes) => bytes.to_vec(),
    };

    // Determine if this method carries a body
    let has_body = matches!(target_method.as_str(), "POST" | "PUT" | "PATCH");

    // --- Task 3.5 / 3.6: Send request, return response or 502 ---
    // Configure ureq to NOT treat non-2xx status codes as errors,
    // so we can return the full response body verbatim (spec requirement).
    let agent = ureq::Agent::new_with_config(
        ureq::config::Config::builder()
            .http_status_as_error(false)
            .build(),
    );

    let ureq_result = if has_body {
        let mut req = match target_method.as_str() {
            "POST" => agent.post(&target_url),
            "PUT" => agent.put(&target_url),
            "PATCH" => agent.patch(&target_url),
            _ => unreachable!(),
        };
        for (name, value) in &filtered_headers {
            req = req.header(name, value);
        }
        req.send(&request_body[..])
    } else {
        let mut req = match target_method.as_str() {
            "GET" => agent.get(&target_url),
            "DELETE" => agent.delete(&target_url),
            "HEAD" => agent.head(&target_url),
            other => {
                return Ok(Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(Body::Text(format!("Unsupported method: {}", other).into()))
                    .unwrap());
            }
        };
        for (name, value) in &filtered_headers {
            req = req.header(name, value);
        }
        req.call()
    };

    match ureq_result {
        Ok(response) => build_proxy_response(response),
        Err(e) => {
            // Task 3.6: Return HTTP 502 on outbound connection errors
            Ok(Response::builder()
                .status(StatusCode::BAD_GATEWAY)
                .body(Body::Text(format!("Upstream request failed: {}", e).into()))
                .unwrap())
        }
    }
}

/// Build a Lambda response from a ureq http::Response<ureq::Body>, copying status, headers, and body.
fn build_proxy_response(
    mut response: ureq::http::Response<ureq::Body>,
) -> Result<Response<Body>, Error> {
    let status = response.status();
    let mut builder = Response::builder().status(status);

    // Copy response headers
    for (name, value) in response.headers().iter() {
        builder = builder.header(name, value);
    }

    // Read response body
    let mut body_bytes = Vec::new();
    response
        .body_mut()
        .as_reader()
        .read_to_end(&mut body_bytes)
        .unwrap_or(0);

    Ok(builder.body(Body::Binary(body_bytes)).unwrap())
}

/// Constant-time comparison to avoid timing attacks on the auth secret.
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut result: u8 = 0;
    for (x, y) in a.iter().zip(b.iter()) {
        result |= x ^ y;
    }
    result == 0
}
