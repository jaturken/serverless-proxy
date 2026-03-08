## ADDED Requirements

### Requirement: Target URL and method extraction
The handler SHALL read the target URL from the `X-Target-URL` request header and the HTTP method from the `X-Target-Method` request header. Both headers are required. If either is missing or malformed, the handler MUST return HTTP 400 with a descriptive error body.

#### Scenario: Both headers present
- **WHEN** a request includes valid `X-Target-URL` and `X-Target-Method` headers
- **THEN** the handler proceeds to forward the request using those values

#### Scenario: X-Target-URL missing
- **WHEN** a request is missing the `X-Target-URL` header
- **THEN** the handler MUST return HTTP 400 with body `Missing X-Target-URL header`

#### Scenario: X-Target-Method missing
- **WHEN** a request is missing the `X-Target-Method` header
- **THEN** the handler MUST return HTTP 400 with body `Missing X-Target-Method header`

---

### Requirement: Request forwarding
The handler SHALL forward the incoming request body and headers to the `X-Target-URL` using the method specified in `X-Target-Method`. The proxy-specific headers (`X-Proxy-Auth`, `X-Target-URL`, `X-Target-Method`, `host`) SHALL be stripped before forwarding. All other incoming headers SHALL be forwarded to the target.

#### Scenario: Successful relay
- **WHEN** a valid authenticated request specifies a reachable target URL
- **THEN** the handler MUST make an HTTP request to `X-Target-URL` with the given method, body, and filtered headers, and return the target's status code, headers, and body to the original caller

#### Scenario: Proxy headers stripped
- **WHEN** the request is forwarded to the target
- **THEN** the target request MUST NOT contain `X-Proxy-Auth`, `X-Target-URL`, `X-Target-Method`, or `host` headers

#### Scenario: Target unreachable
- **WHEN** the outbound HTTP call to the target fails (connection refused, DNS failure, timeout)
- **THEN** the handler MUST return HTTP 502 with a descriptive error body

#### Scenario: Non-2xx target response
- **WHEN** the target returns any HTTP status code (including 4xx or 5xx)
- **THEN** the handler MUST return that status code and body verbatim to the caller without modification
