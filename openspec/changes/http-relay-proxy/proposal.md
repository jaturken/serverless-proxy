## Why

Developers and services behind network restrictions need a way to make outbound HTTP requests to arbitrary targets without direct internet access. A lightweight serverless relay proxy on AWS Lambda provides a cost-effective, low-maintenance solution with no always-on infrastructure.

## What Changes

- New Rust binary deployed as an AWS Lambda function exposed via Function URL
- Accepts inbound HTTP requests with relay instructions encoded in headers
- Authenticates callers via a shared secret (`X-Proxy-Auth` header)
- Forwards requests to any target URL specified in `X-Target-URL` and `X-Target-Method` headers
- Returns the target's response (status, headers, body) verbatim to the caller

## Non-goals

- HTTPS CONNECT tunneling (only HTTP relay)
- Per-client tokens or dynamic secret rotation
- Response streaming
- API Gateway (Function URL only)
- Rate limiting or request logging beyond Lambda built-ins

## Capabilities

### New Capabilities

- `proxy-auth`: Header-based shared-secret authentication for incoming relay requests
- `http-relay`: Forward an HTTP request to an arbitrary target URL and return the response

### Modified Capabilities

_(none — greenfield project)_

## Impact

- New Rust crate with dependencies: `lambda_http`, `ureq`, `tokio`
- AWS infrastructure: Lambda function + Function URL (deployed via Cargo Lambda + AWS CDK)
- Single environment variable: `PROXY_AUTH_SECRET`
