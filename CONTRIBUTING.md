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
