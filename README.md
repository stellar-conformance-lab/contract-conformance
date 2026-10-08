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

The project is currently under active development. **SEP-41 (Token Standard) v0.5.2** is the inaugural conformance profile. It evaluates metadata, balance, transfers, allowances, burns, events, and authorizations.

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

## How to add a new fixture

1. Implement the `Fixture` and profile-specific traits (e.g., `Sep41Fixture`) for your custom contract.
2. Contain all contract registration, initial minting, and authorization bypass setup exclusively inside the `Fixture::setup` method.
3. Hook your fixture directly into the generic scenario loop natively using `ConformanceEngine::run_isolated_scenario`.

## How to add a new conformance scenario

1. Navigate to the relevant profile crate (e.g., `crates/conformance-sep41/src/lib.rs`).
2. Define a new `Scenario<F>` struct allocating a stable scenario ID (e.g., `SEP41-NEW-001`).
3. Leverage isolated environment state checks to yield an explicit `Status::Pass`, `Status::Fail`, or `Status::Error`.
4. Register the new scenario explicitly within the downstream fixtures targeting the profile.
