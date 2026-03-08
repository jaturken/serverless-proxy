## ADDED Requirements

### Requirement: Shared-secret authentication
Every incoming request to the Lambda Function URL SHALL be authenticated by comparing the value of the `X-Proxy-Auth` request header to the `PROXY_AUTH_SECRET` environment variable using a constant-time comparison. Requests with a missing or non-matching header MUST be rejected with HTTP 401.

#### Scenario: Valid auth header accepted
- **WHEN** a request arrives with `X-Proxy-Auth` matching `PROXY_AUTH_SECRET`
- **THEN** the request proceeds to the relay logic

#### Scenario: Missing auth header rejected
- **WHEN** a request arrives without an `X-Proxy-Auth` header
- **THEN** the handler MUST return HTTP 401 with body `Unauthorized`

#### Scenario: Wrong secret rejected
- **WHEN** a request arrives with an `X-Proxy-Auth` value that does not match `PROXY_AUTH_SECRET`
- **THEN** the handler MUST return HTTP 401 with body `Unauthorized`

#### Scenario: PROXY_AUTH_SECRET not configured
- **WHEN** the `PROXY_AUTH_SECRET` environment variable is not set or empty at startup
- **THEN** the Lambda MUST panic/fail to start with a descriptive error message
