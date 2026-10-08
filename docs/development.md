# Development Guide

Welcome to the Stellar Contract Conformance testing framework! This guide provides a conceptual overview of how the repository is structured, how tests are executed, and how to author new tests or fixtures. 

## Crate Responsibilities & Dependency Direction

The workspace is organized hierarchically to enforce strict separation of concerns. Dependencies flow **unidirectionally** from the reporting/testing layers down to the core abstractions.

* **`conformance-core`**: Defines the generic traits for `Fixture` and `Scenario`, the `Status` enumerations, and the `ConformanceEngine`. It does **not** know about SEP-41.
* **`conformance-sep41`**: Contains the actual behavioral `Scenario` tests for the SEP-41 standard (e.g., Transfer, Allowance, Burn). It depends on `conformance-core`.
* **`conformance-report`**: Houses the structured `ConformanceReport` format and JSON serializers. It consumes results from `conformance-core` but knows nothing about the tests themselves.
* **`fixtures/valid`**: The reference fixture using a compliant SEP-41 contract (Stellar Asset Contract) to prove the framework accepts valid contracts. Depends on `conformance-sep41` and `conformance-core`.
* **`fixtures/invalid`**: A suite of deliberately broken mocked contracts used to mathematically prove that the conformance engine actively detects standard violations.

## Architecture and Execution Flow

1. **Fixture Injection**: A `Fixture` is a localized environment wrapper bridging the generic test scenarios with a specific contract implementation. Fixtures handle custom setup like minting tokens or funding accounts implicitly, hiding implementation-specifics.
2. **Isolated Environment Model**: The `ConformanceEngine` iterates over a list of scenarios. For **every single scenario**, the engine spawns a completely fresh `soroban_sdk::Env`. This guarantees 100% test isolation and prevents state pollution.
3. **Scenario Execution**: Inside the isolated environment, a scenario interacts with the contract *only* through the standardized SEP-41 client interfaces. 
4. **Result / Status Model**: The scenario verifies the observable state transitions. It yields a `TestResult` containing a definitive `Status`:
    * `PASS`: Valid behavior.
    * `FAIL`: Standard violation detected (behavioral non-conformance).
    * `ERROR`: The fixture setup or infrastructure crashed before meaningful evaluation could happen.
5. **Reporting Flow**: Evaluated results are aggregated and structured into a JSON-compatible `ConformanceReport` ensuring deterministic CI validations.

## Valid vs Deliberately Non-Conforming Fixtures

* **Valid Fixtures** (`fixtures/valid/`): Use completely compliant implementations. A scenario `FAIL` against this fixture means our test is flawed, or the reference implementation changed.
* **Invalid / Negative Fixtures** (`fixtures/invalid/`): Micro-contracts intentionally lacking a specific rule (e.g., ignoring authorization). When run against the conformance engine, we *expect* a `FAIL`. By verifying the framework returns `FAIL` natively, we prove the framework provides true quality-assurance capabilities.

## Making Changes: Where to go?

* **Adding a new conformance test for SEP-41**: Open `crates/conformance-sep41/src/lib.rs`. Define a new struct implementing `Scenario<F>`. Add your logic using explicit `TestResult` returns. Register it in `fixtures/valid/src/lib.rs`.
* **Testing an intentional violation**: Open `fixtures/invalid/src/lib.rs`. Define a new `#[contract]` mirroring the SEP-41 signature but mutating state improperly. Use the `impl_fixture!` macro to map it, and add a standard Rust `#[test]` asserting the engine catches a `Status::Fail`.
* **Modifying Report outputs**: Edit `crates/conformance-report/src/lib.rs`.
