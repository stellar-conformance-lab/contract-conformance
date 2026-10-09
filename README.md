# Stellar Contract Conformance

Stellar Contract Conformance is an independent, reusable testing framework for Soroban smart-contract standards. It evaluates whether a smart contract dynamically behaves in accordance with the specified behavioral requirements of a standard.

## The Problem and Behavioral Conformance

Traditional compilation checks verify **interface compatibility**—meaning the contract exposes the expected function signatures and types. However, interface compatibility is inherently insufficient to guarantee safe interoperability. A contract may implement a `transfer` function signature perfectly but fail to properly deduct balances, handle authorization, or track allowances correctly behind the scenes. 

Stellar Contract Conformance focuses purely on **behavioral conformance**. By treating the contract as a black box and driving state changes through generic interfaces, it verifies that side effects, event emissions, state mutations, and authorizations strictly match standard requirements.

## Core Concepts

Understanding the framework relies on clearly distinguishing the following layers:

* **Specification**: The written standard describing the expected rules (e.g., SEP-41).
* **Conformance Profile**: A programmatic collection of grouped tests verifying a specification.
* **Engine**: The generic orchestration layer (`ConformanceEngine`) that executes tests inside perfectly isolated evaluation boundaries.
* **Scenario**: A single, focused assertion mapped to a strict requirement within a specification.
* **Fixture**: The boundary wrapper connecting the generic conformance tests to a specific implementation's unique setup requirements (minting, deploying, etc.).
* **Report**: The deterministic, machine-readable JSON summary of scenario outcomes.

## Current Scope and Status

The core conformance engine is stable and in active maintenance. **SEP-41 (Token Standard) v0.5.2** is the project's stable, flagship behavioral conformance profile. It evaluates metadata, balance, transfers, allowances, burns, events, and authorizations.

### MVP Milestone (Complete)
* **MVP status:** Complete against the documented MVP acceptance criteria.
* **Delivered functionality:** SEP-41 conformance execution, structured JSON report generation, historical report publishing, and dashboard visualization of current and historical results.
* **Architecture boundary:** The engine is authoritative for conformance execution; the dashboard visualizes published reports and does not execute arbitrary contracts.
* **Deferred improvements:** End-to-end dashboard tests, further accessibility work, historical-run comparison, and support for additional profiles.
* **Verification caveat:** Failed-report publishing has been reviewed logically, but the failure path has not yet been demonstrated end-to-end.

Development of additional behavioral profiles is currently paused. Future profile expansion is strictly contingent upon the Stellar ecosystem maturing additional smart-contract standards into "Active" or "Final" status to guarantee stable normative requirements.

### Limitations / Non-goals
This framework does **NOT** provide:
* Certified compliance badges or official Stellar/SDF certification.
* Security auditing, vulnerability scanning, or formal verification.
* Production security guarantees.
* Live-network contract scanning or blockchain exploration.
* 100% complete edge-case test coverage.

## Architecture

The project separates testing evaluation logic completely from the target implementations:
1. **Generic Engine**: `crates/conformance-core/` and `crates/conformance-report/` handle generic evaluation and JSON aggregation.
2. **Behavioral Profile**: `crates/conformance-sep41/` contains standard-driven test assertions cleanly agnostic to any specific token contract's private logic.
3. **Reference Fixture**: `fixtures/valid/` evaluates the framework against the Stellar Asset Contract as the deterministic reference implementation under test.
4. **Deliberately Non-Conforming Fixtures**: `fixtures/invalid/` utilizes a suite of intentionally broken contracts to mathematically prove that the conformance engine correctly detects and penalizes actual standard violations.

### Continuous Integration
The repository enforces deterministic execution via strict CI quality gates utilizing stable Rust constraints (`rustfmt`, `clippy`, and `cargo test --workspace`), assuring all positive and negative fixture coverage remains functionally preserved.

## Repository Structure

```text
contract-conformance/
├── .github/workflows/      # CI pipelines
├── crates/
│   ├── conformance-core/   # Generic evaluation engine
│   ├── conformance-report/ # Result JSON serialization
│   └── conformance-sep41/  # SEP-41 scenario traits
├── docs/                   # Architectural guides
├── fixtures/
│   ├── valid/              # Compliant reference implementations
│   └── invalid/            # Broken implementations targeting framework validation
├── ROADMAP.md              # Active phased development tracker
├── CONTRIBUTING.md         # Contribution and workflow expectations
└── rust-toolchain.toml     # Stable toolchain specifier
```

## How to run the test suite

You can execute the full suite natively leveraging the Soroban test environment locally:

```bash
cargo test --workspace
```

## Command Line Interface

You can run the conformance engine against supported profiles using the CLI:

**Installed / Release form:**
```bash
stellar-conform test --profile sep-41
```

**Development invocation:**
```bash
cargo run -p conformance-cli -- test --profile sep-41
```

## GitHub Action Integration

A consuming repository can natively run SEP-41 conformance checks via GitHub Actions. Under the hood, this action invokes the `stellar-conform` CLI.

**Basic usage:**
```yaml
name: Contract Conformance

on:
  pull_request:
  push:
    branches:
      - main

jobs:
  conformance:
    runs-on: ubuntu-latest
    steps:
      - uses: stellar-conformance-lab/contract-conformance@main
```

**Inputs:**
* `profile` (optional): The conformance profile to execute. Defaults to `sep-41`.

**Failure Behavior:**
If the selected profile encounters a failure, error, or if an unsupported profile is supplied, the action explicitly inherits the CLI's non-zero exit code and fails the CI job appropriately.

## Property-Based Testing

To supplement the deterministic scenario suite, Stellar Contract Conformance includes property-based tests using `proptest`. While handwritten scenarios cover specific fixed inputs, property testing aims to discover edge cases by evaluating thousands of dynamically generated, valid inputs against generalized contract invariants. 

Property-based testing does **not** prove complete SEP-41 correctness. It acts as an additional layer of assurance to catch implementation oversights.

### Covered Properties

The current framework tests the following SEP-41 invariants:
1. **Transfer Conservation**: Valid transfers correctly subtract from the sender and add to the receiver (`sender_after = sender_before - amount`, `receiver_after = receiver_before + amount`).
2. **Failed-Operation Stability**: Invalid operations (such as transfers exceeding balance or unapproved delegated spending) must fail natively and leave the ledger state entirely untouched (`state_after = state_before`).
3. **Allowance Consistency**: Delegated transfers properly subtract from both the sender's balance and the spender's allowance, while ensuring that the allowance expiration ledger behavior remains strictly enforced.

### Test Architecture

* **Constrained Generation**: Test cases are systematically constrained to respect valid pre-conditions (e.g., ensuring `amount >= 0` and balances satisfy the transfer request) before testing the invariant.
* **Determinism**: The `proptest` framework guarantees that generated failure cases remain fully reproducible using the test seeding functionality. No live RPC or external network dependencies are used; everything runs in completely isolated simulated environments.
* **Execution**: Run property tests naturally alongside the standard suite via `cargo test`.

## How to add a new fixture

1. Implement the `Fixture` and profile-specific traits (e.g., `Sep41Fixture`) for your custom contract.
2. Contain all contract registration, initial minting, and authorization bypass setup exclusively inside the `Fixture::setup` method.
3. Hook your fixture directly into the generic scenario loop natively using `ConformanceEngine::run_isolated_scenario`.

## How to add a new conformance scenario

1. Navigate to the relevant profile crate (e.g., `crates/conformance-sep41/src/lib.rs`).
2. Define a new `Scenario<F>` struct allocating a stable scenario ID (e.g., `SEP41-NEW-001`).
3. Leverage isolated environment state checks to yield an explicit `Status::Pass`, `Status::Fail`, or `Status::Error`.
4. Register the new scenario explicitly within the downstream fixtures targeting the profile.
