## Context

This is a greenfield Rust project deployed as an AWS Lambda function exposed via a Function URL. The proxy receives HTTP requests from clients who cannot reach target servers directly (e.g. network-restricted environments). The Lambda function authenticates the caller, then forwards the request to the target URL and returns the response.

## Goals / Non-Goals

**Goals:**
- Authenticate incoming requests using a shared secret header (`X-Proxy-Auth`)
- Forward HTTP requests to arbitrary target URLs (`X-Target-URL`, `X-Target-Method`)
- Return the proxied response (status, headers, body) verbatim
- Keep the implementation simple, single-binary, no async complexity exposed to the business logic

**Non-Goals:**
- HTTPS CONNECT tunneling (WebSocket or raw TCP proxying)
- Per-client tokens or dynamic auth
- Response streaming or chunked transfer
- Rate limiting, logging beyond Lambda defaults
- API Gateway integration

## Decisions

### D1: Runtime — `lambda_http` + `tokio`

**Choice**: Use `lambda_http` (AWS official crate) as the Lambda runtime handler.

**Rationale**: `lambda_http` handles Lambda Function URL event parsing and response serialization transparently. The handler is declared `async fn` as required by the runtime, but business logic inside is fully synchronous — no `.await` appears in domain code.

**Alternative considered**: `lambda_runtime` with manual JSON deserialization — more boilerplate, no meaningful benefit.

---

### D2: Outbound HTTP Client — `ureq`

**Choice**: Use `ureq` for the outbound HTTP call to the target.

**Rationale**: `ureq` is synchronous, zero-dependency (uses `rustls` for TLS), and extremely simple API. Since we don't stream responses and have no concurrency needs inside a single Lambda invocation, async HTTP adds complexity without benefit.

**Alternative considered**: `reqwest` blocking feature — same result but heavier dependency tree. `hyper` — too low-level.

---

### D3: Auth — Shared Secret in Environment Variable

**Choice**: Compare `X-Proxy-Auth` header value against `PROXY_AUTH_SECRET` environment variable (constant-time compare).

**Rationale**: Simple to deploy, rotate (Lambda env var update), and reason about. Constant-time compare avoids timing attacks.

---

### D4: Deployment — Cargo Lambda + AWS CDK

**Choice**: Build with `cargo lambda build --release --arm64`, deploy infrastructure with AWS CDK (TypeScript).

**Rationale**: `cargo lambda` handles cross-compilation and Lambda bootstrap binary packaging. CDK gives infrastructure-as-code for the Lambda function + Function URL. `arm64` (Graviton2) is cheaper and has lower cold start than `x86_64` for compiled binaries.

---

### D5: Header Forwarding Strategy

**Choice**: Forward all headers from the original client request to the target, except `X-Proxy-Auth`, `X-Target-URL`, `X-Target-Method`, and `host`.

**Rationale**: Proxy-specific headers must not leak to the target. `host` must be stripped as it refers to the Lambda domain, not the target. All other headers (e.g. `content-type`, `authorization`, `accept`) are forwarded to faithfully relay the client's intent.

## Risks / Trade-offs

| Risk | Mitigation |
|---|---|
| Lambda outbound blocked by NAT/VPC | Default Lambda has internet access; VPC config required only for private targets (out of scope) |
| Target TLS errors (self-signed certs) | `ureq` + `rustls` verifies TLS by default; document that self-signed targets need explicit opt-out |
| Large response bodies exceeding Lambda payload limit (6MB) | Document 6MB limit; buffered response is acceptable for the stated use case |
| Secret exposure in logs | Ensure `X-Proxy-Auth` header is never logged |
| Open relay abuse | Shared secret is the only gate; operators must treat `PROXY_AUTH_SECRET` as a sensitive credential |
