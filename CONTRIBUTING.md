# Contributing to Vito (Rust Backend)

Thanks for your interest in contributing to Vito! We welcome all kinds of contributions including bug reports, feature requests, and pull requests.

## How to Contribute

### 1. Fork & Branch
- Fork the repository
- Create a new branch from `main`, e.g. `feature/add-logging`

### 2. Please do testing
- Please add unit tests for the functionality you implement.
- Make sure all tests pass before submitting your pull request.

```bash
cargo test
```

### 3. Code Standards
- We use Rust with `cargo fmt` for formatting and `clippy` for linting.
- Please make sure to format and lint your code before submitting.
```bash
cargo fmt
cargo clippy
```