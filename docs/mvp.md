# Minimum Viable Product (MVP) Boundary

The MVP focuses exclusively on proving that the core conformance model works for the most fundamental Stellar standard: SEP-41. It establishes the foundational architecture without over-engineering or adding premature capabilities.

## In Scope

The following components and features are explicitly **in scope** for the MVP:

- **Rust Workspace:** A modular project structure for the framework.
- **Reusable Conformance Engine:** The core testing runner and orchestrator.
- **SEP-41 Profile:** The formalized test cases and assertions for SEP-41.
- **Explicit Setup/Fixture Mechanism:** The abstraction allowing arbitrary contracts to be tested by injecting initialization logic.
- **Test Categories:**
  - Metadata tests (name, symbol, decimals)
  - Balance tests
  - Transfer tests
  - Allowance and `transfer_from` tests
  - Burn tests
  - Event verification tests
  - Authorization tests
  - Edge cases and limit tests
- **Negative Fixtures:** Deliberately broken contract implementations used to prove the framework catches violations.
- **Deterministic Test Execution:** All tests run locally and deterministically via the Soroban SDK test environment.
- **Structured Reporting:** Both human-readable (terminal) and machine-readable (JSON) outputs.
- **Automated CI:** GitHub Actions to ensure code quality and prevent regressions.
- **Documentation:** Clear guidance on architecture, usage, and contribution.

## Out of Scope

The following features are explicitly **out of scope** for the MVP to maintain focus and prevent scope creep:

- **Live-network scanning:** The MVP does not test contracts on Testnet, Futurenet, or Mainnet. Conformance testing should be done locally during development.
- **RPC-based arbitrary contract discovery:** We do not scan the ledger to find and test contracts automatically.
- **Automatic mint-function discovery:** Contract initialization is explicitly handled by user-provided setup fixtures, not magic heuristics.
- **YAML / Custom Test DSL:** The framework uses native Rust and Soroban testing idioms to avoid the overhead of building and maintaining a custom parser and execution environment.
- **Web UI & Database:** Results are generated as files or standard output. No GUI or persistent storage infrastructure is needed.
- **Hosted Service / SaaS:** The framework is a developer tool to be run locally or in CI, not a hosted service.
- **Security Auditing & Formal Verification:** The framework checks behavioral conformance to a specific standard, not general security vulnerabilities or mathematical proofs of correctness.
- **AI-generated tests:** Tests are meticulously hand-crafted against the SEP-41 specification to ensure high fidelity and trust.
- **Additional Standards (SEP-50, SEP-56):** These will be considered only after the SEP-41 MVP is mature and battle-tested. 
