# Changelog

All notable changes of this package. Versions follow [Semantic Versioning](https://semver.org/).

## [1.3.1](https://github.com/unione-repo/unione-rust-sdk/releases/tag/v1.3.1) - 2026-11-05

### Fixed

- Rust 1.70: serde is pinned to 1.0.228 (newer serde_derive needs Rust 1.71) in the MSRV workflow and the README pins.

## [1.3.0](https://github.com/unione-repo/unione-rust-sdk/releases/tag/v1.3.0) - 2026-11-05

### Added

- Tests of the package (offline: the API is faked, no request leaves the machine).
- A CI workflow (`.github/workflows/test.yml`) runs the tests on every push and pull request over the supported runtimes; a release is published only when they pass.
- `ApiError` tells what failed and how to act: `method` (the API method, `email/send`), `kind` (`api`, `network`, `timeout`, `invalid_response`, `invalid_request`), `retryable()` (a repeat may succeed: a network error, a timeout, HTTP 429 or 5xx), `hint` (how to fix the API error, from the errors table of the docs) and `api_message` (the message of the API as is).
- Checks before sending (kind `invalid_request`, no request is sent): an empty API key; for `email/send` a missing or empty `message`, `message.recipients` or `email` of a recipient.
- A CI workflow checks the build with Rust 1.70; the README lists the dependency pins for 1.70.

### Changed

- **The text of the error message** names the method and, for an API error, the code, the HTTP status and the fix: `email/send: Unknown recipient (API error 204, HTTP 400). Fix: ... See ...#api-errors`. Code that parsed the message should use the error code and `api_message`.
- A text of the server in the message is cut to 500 characters (`api_message` keeps all of it).

### Fixed

- The API error code is taken only when it is a number (or a string of digits).

Earlier releases: the [tags](https://github.com/unione-repo/unione-rust-sdk/tags) of the repository.
