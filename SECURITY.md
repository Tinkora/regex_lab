# Security Policy

## Supported Versions

| Version | Supported |
|---------|-----------|
| 0.1.x (current) | ✅ |

## Reporting a Vulnerability

If you discover a security vulnerability, please **do not** open a public issue.

Instead, email the project maintainer directly. You should receive a response
within 48 hours. We will work with you to understand the scope and coordinate
a fix and disclosure timeline.

### Scope

The following areas are within scope:

- Regex pattern injection leading to WASM sandbox escape
- Denial of service via pathological regex patterns
- WASM memory safety issues
- Input validation bypasses

### Out of Scope

- Issues already documented as known limitations
- Theoretical attacks requiring physical access
- Issues in dependencies (please report upstream)

## Security Model

The regex_lab project follows these security principles:

1. **Browser-local**: All regex processing happens in-browser via WASM. No test strings, patterns, or replacement text ever leave the browser.

2. **Input validation**: Patterns are validated before compilation. Input and pattern lengths are capped to prevent resource exhaustion.

3. **No server**: There is no backend server — everything runs as a static site. No data to breach, no secrets to leak.

4. **CSP-ready**: The static editor generates no inline scripts from user content and renders all results via DOM API (no innerHTML from user input).
