# Contributing to Stellar Contract Conformance

Welcome! We are excited to accept contributions. This document outlines the expected developer experience, quality requirements, and architecture needed to contribute effectively to the repository.

## Project Purpose
Build an independent, reusable behavioral conformance testing framework for Soroban smart-contract standards.

## Prerequisites and Toolchain
* **Supported Toolchain**: Standard stable Rust (`rust-toolchain.toml` specifies the `stable` channel).
* **Dependencies**: No excessive requirements beyond `soroban-sdk`.

## Local Validation Commands
Before opening a pull request, ensure your code passes the following quality gates locally. These exact commands are also run by our CI workflow for every pull request:

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```
CI rigorously enforces these limits. Code cannot be merged if formatting, linters, or test assertions fail.

## Roadmap-Driven Development
We follow strict roadmap-driven development. Contributors must review [ROADMAP.md](ROADMAP.md) and avoid introducing scope outside the current roadmap phase.

## Architecture at a Contributor Level

The framework operates on strict unidirectional boundaries:
1. **Generic Engine** (`conformance-core`): Runs scenarios in strict isolation.
2. **Behavioral Profile** (`conformance-sep41`): Contains the abstract testing scenarios logic mapping to SEP-41 requirements.
3. **Fixtures** (`fixtures/valid` and `fixtures/invalid`): Implementation-specific wrappers hooking custom contracts into generic engine loops.

## How to Add a Conformance Scenario

A scenario executes a single evaluation against an active contract.

1. **Locate Standard Profile**: Add a new struct inside `crates/conformance-sep41/src/lib.rs`.
2. **Stable IDs**: Ensure the `id()` method returns a stable, sequential identifier (e.g., `SEP41-NEW-001`). Do NOT generate dynamically.
3. **Execute Logic**: Implement `Scenario::run()`. Interact purely via standard generic interfaces (e.g., `TokenClient`). Do not depend on implementation-specific endpoints.
4. **Validation Distinctions**:
    * Return `Status::Fail`: If the contract successfully executed the operation but mathematically/structurally violated standard requirements (e.g., failed to decrease balance).
    * Return `Status::Error`: If the generic setup or infrastructure failed preventing test evaluation (e.g., unable to deploy).

## How to Add or Modify a Fixture

Fixtures bridge generic standard logic into specific smart contract contexts.

1. **Keep Setup Internal**: All contract deployments, administrative bypasses, initial state minting, and internal authorization setups **must** remain exclusively inside the `Fixture::setup()` method. Generic scenarios should never be forced to understand specific contract administration logic.
2. **Implement Profile Traits**: Implement the relevant trait (e.g., `Sep41Fixture`) supplying exactly the deterministic dependencies required for generic execution.
3. **Register Scenarios**: Within `fixtures/<your-fixture>/src/lib.rs`, manually invoke `ConformanceEngine::run_isolated_scenario` passing the new/modified scenario struct mapping to your fixture factory.
4. **Test Isolation Expectations**: Note that `ConformanceEngine::run_isolated_scenario` builds a completely clean `Env` for *every* execution. Fixtures must remain deterministic without relying on shared global state cascades.

## Negative Fixtures Expectations

Deliberately non-conforming fixtures exist inside `fixtures/invalid/`. 

* If you add a scenario, verify your assertion actively fails when evaluated against a negatively mocked fixture violating that exact rule.
* Map expected violations locally within `fixtures/invalid/src/lib.rs` verifying the scenario outputs `Status::Fail` to pass the local CI run. 

## Specification Traceability (Documenting Behavior)

When creating or modifying scenarios, you must provide clear traceability back to the specification. Ensure your structural implementation mirrors:

`SEP-41 Requirement → conformance scenario mapping → explicit assertion → deterministic expected outcome`

Do NOT:
* Invent phantom requirements that are not strictly present in SEP-41 specifications.
* Add unsupported edge-case derivations simply to arbitrarily increase the test count.

## How to Propose Changes
1. Review the current phase in [ROADMAP.md](ROADMAP.md).
2. Open an issue using the provided templates to discuss proposed changes.
3. Submit a pull request referencing the issue ensuring CI commands successfully compile.
