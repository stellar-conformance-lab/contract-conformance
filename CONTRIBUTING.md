# Contributing to Stellar Contract Conformance

## Project Purpose
Build an independent, reusable behavioral conformance testing framework for Soroban smart-contract standards.

## Development Philosophy
The core testing framework should integrate naturally with the Soroban/Rust development ecosystem. We prioritize deterministic local testing and require explicit setup for fixtures.

## Roadmap-Driven Development
We follow strict roadmap-driven development. Contributors must review [ROADMAP.md](ROADMAP.md) and avoid introducing scope outside the current roadmap phase.

## How to Propose Changes
1. Review the current phase in [ROADMAP.md](ROADMAP.md).
2. Open an issue using the provided templates to discuss proposed changes.
3. Submit a pull request referencing the issue.

## Coding Expectations
* Use standard Rust practices.
* Follow the architecture established for the current phase.
* Do not introduce unnecessary dependencies.

## Local Validation Commands
Before opening a pull request, ensure your code passes the following quality gates locally. These exact commands are also run by our CI workflow for every pull request:

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

## Testing Expectations
* Provide both positive and negative tests.
* Ensure deterministic tests.

## Documentation Expectations
* Document behavior, architecture, and expected test outcomes.
* Keep documentation synchronized with the current phase.

## Pull Request Expectations
* Keep PRs focused on the current roadmap phase.
* Fill out the PR template completely.
* Ensure all tests pass.

## Issue Expectations
* Use the provided bug report or feature request templates.
* Clearly identify the relevant conformance profile/version when applicable.
