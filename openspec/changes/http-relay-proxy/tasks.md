## 1. Project Setup

- [x] 1.1 Update `Cargo.toml` with dependencies: `lambda_http`, `ureq` (with TLS), `tokio` (minimal features)
- [x] 1.2 Verify `cargo build` succeeds with the new dependencies

## 2. Authentication

- [x] 2.1 Read `PROXY_AUTH_SECRET` from environment at startup; panic with descriptive message if missing or empty
- [x] 2.2 Implement constant-time comparison of `X-Proxy-Auth` header value against the secret
- [x] 2.3 Return HTTP 401 with body `Unauthorized` when auth fails or header is missing

## 3. Request Relay

- [x] 3.1 Extract and validate `X-Target-URL` header; return HTTP 400 if missing
- [x] 3.2 Extract and validate `X-Target-Method` header; return HTTP 400 if missing
- [x] 3.3 Build header filtering logic: strip `X-Proxy-Auth`, `X-Target-URL`, `X-Target-Method`, `host` from forwarded headers
- [x] 3.4 Forward request to target using `ureq` with method, filtered headers, and body
- [x] 3.5 Return target response (status code, headers, body) verbatim to caller
- [x] 3.6 Return HTTP 502 with descriptive body on outbound connection errors

## 4. Lambda Wiring

- [x] 4.1 Wire authentication and relay logic into `lambda_http` handler in `main.rs`
- [x] 4.2 Test locally with `cargo lambda watch` (mock Lambda event) *(requires `cargo-lambda` install: `cargo install cargo-lambda`)*

## 5. Infrastructure (AWS CDK)

- [x] 5.1 Initialize a CDK app (TypeScript) in `infra/` directory
- [x] 5.2 Define Lambda function resource pointing to the `arm64` binary
- [x] 5.3 Configure Function URL with `NONE` auth mode (auth handled in-Lambda)
- [x] 5.4 Add `PROXY_AUTH_SECRET` as a Lambda environment variable placeholder (to be set via AWS Secrets or manually)
- [x] 5.5 Deploy with `cdk deploy` and smoke-test the Function URL endpoint *(run manually after building with `cargo lambda build --release --arm64`)*
