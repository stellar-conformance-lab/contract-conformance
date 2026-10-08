# Stellar Contract Conformance — Development Roadmap

## Project

**Organization:** `stellar-conformance-lab`
**Repository:** `contract-conformance`
**Project:** Stellar Contract Conformance

## Mission

Build an independent, reusable behavioral conformance testing framework for Soroban smart-contract standards.

The first supported standard is **SEP-41**, the Stellar token interface.

The project must verify that a contract behaves according to the selected standard, not merely that the contract exposes the expected functions.

---

# 1. Product Vision

The final system should allow a Soroban developer to take a contract implementation and verify it against a versioned Stellar standard.

Conceptually:

```text
                 Stellar Standard
                       │
                       ▼
              Conformance Profile
                       │
                       ▼
              Conformance Engine
                       │
                Setup / Fixture
                       │
                       ▼
               Contract Under Test
                       │
          ┌────────────┼────────────┐
          ▼            ▼            ▼
       Behavior      Events       State
          │            │            │
          └────────────┼────────────┘
                       ▼
              Conformance Report
```

The project must distinguish between:

* interface compatibility
* behavioral conformance
* state-transition correctness
* event correctness
* authorization behavior
* edge-case behavior

---

# 2. Core Principles

The agent MUST follow these principles throughout development.

## 2.1 Test behavior, not just interfaces

Do not stop at checking whether required functions exist.

The framework must execute scenarios and verify observable behavior.

## 2.2 Use Rust and Soroban-native testing

The core testing framework should integrate naturally with the Soroban/Rust development ecosystem.

Do not introduce a custom YAML testing language for the core engine.

## 2.3 Deterministic local testing first

The MVP must run against a deterministic local Soroban test environment.

Do not depend on live Stellar networks or RPC for the core conformance engine.

## 2.4 Setup is explicit

The framework must NOT attempt to magically discover how an arbitrary contract mints tokens or initializes state.

Contract-specific setup must be supplied by the test author through an explicit fixture/setup mechanism.

## 2.5 Standards are versioned

A conformance result must identify the exact standard/profile version against which the contract was tested.

Example:

```text
SEP-41 v0.5.2
```

Do not report simply:

```text
SEP-41 compliant
```

without identifying the tested specification version.

## 2.6 Negative testing is mandatory

The project must include deliberately non-conforming fixtures.

A conformance suite is not credible unless it demonstrates that it can detect known violations.

## 2.7 Evidence matters

Tests should produce enough structured information to explain:

* what was tested
* expected result
* actual result
* pass/fail status
* relevant evidence

## 2.8 Keep the MVP narrow

The first implementation is SEP-41 only.

Do not implement SEP-50, SEP-56, generic smart-contract verification, live-network scanning, security auditing, or arbitrary contract analysis during the MVP.

---

# 3. Scope

## 3.1 MVP IN SCOPE

The MVP must include:

* Rust workspace
* reusable conformance engine
* SEP-41 conformance profile
* Soroban test-environment integration
* explicit setup/fixture mechanism
* metadata tests
* balance tests
* transfer tests
* allowance tests
* burn tests
* event tests
* authorization/error-path tests
* important edge cases
* structured pass/fail results
* intentionally broken fixtures
* reference implementation fixture
* documentation
* automated CI

## 3.2 MVP OUT OF SCOPE

Do NOT implement:

* live-network conformance testing
* RPC-based scanning
* blockchain explorer integration
* automatic mint-function discovery
* automatic contract auditing
* security certification
* vulnerability scanning
* YAML DSL
* GUI/web dashboard
* hosted SaaS
* SEP-50
* SEP-56
* arbitrary smart-contract formal verification
* AI-generated tests
* automatic production deployment
* token discovery/indexing

These may be future projects, but they are not part of v0.1.

---

# 4. Development Phases

The agent MUST execute phases in order.

Do not skip ahead unless a phase's acceptance criteria are satisfied.

---

# Phase 0 — Repository Foundation

## Goal

Prepare the repository for serious development.

## Tasks

Create:

```text
.github/
├── ISSUE_TEMPLATE/
└── PULL_REQUEST_TEMPLATE/

docs/
crates/
examples/
fixtures/
tests/
```

Create foundational files:

```text
README.md
LICENSE
CONTRIBUTING.md
CODE_OF_CONDUCT.md
SECURITY.md
ROADMAP.md
```

Add appropriate GitHub issue and pull-request templates.

Configure basic repository metadata.

## Acceptance criteria

* Repository has a clean structure.
* README explains the project's purpose.
* Contribution expectations are documented.
* Security reporting process exists.
* ROADMAP.md is committed.
* No application code has been prematurely added.

## Deliverable

A clean, professional open-source repository.

---

# Phase 1 — Architecture and Specification

## Goal

Define exactly how conformance testing works before implementing the engine.

## Tasks

Create:

```text
docs/
├── architecture.md
├── conformance-model.md
├── setup-fixtures.md
├── reporting.md
└── mvp.md
```

Document:

### Conformance profile

Define:

```text
Profile
 ├── standard name
 ├── specification version
 ├── requirements
 ├── scenarios
 └── expected behavior
```

### Test lifecycle

Define:

```text
Create environment
      ↓
Deploy contract
      ↓
Run setup fixture
      ↓
Execute conformance scenario
      ↓
Capture observations
      ↓
Compare expected/actual
      ↓
Produce result
```

### Setup model

Define the API through which contract-specific initialization is supplied.

The architecture must allow:

* account creation
* token initialization
* initial balances
* administrative configuration
* allowance setup
* other standard-specific prerequisites

without making the conformance engine dependent on a specific token implementation.

### Result model

Define a structured representation for:

* profile
* specification version
* test identifier
* description
* status
* expected behavior
* observed behavior
* evidence
* error information

## Acceptance criteria

The architecture can answer:

1. How is a contract initialized?
2. How is a test executed?
3. How is expected behavior represented?
4. How is actual behavior captured?
5. How is a failure reported?
6. How are standards/version changes represented?

Do not begin the full test suite until these questions are answered.

---

# Phase 2 — Rust Workspace and Core Engine

## Goal

Build the minimal reusable conformance engine.

## Initial workspace direction

Use a Rust workspace.

Possible structure:

```text
crates/
├── conformance-core/
├── conformance-sep41/
└── conformance-report/
```

Do not create crates that have no clear responsibility.

## Core responsibilities

### `conformance-core`

Own:

* test context
* scenario execution
* assertions
* result collection
* fixture/setup abstractions
* common errors
* common types

### `conformance-report`

Own:

* result representation
* human-readable formatting
* machine-readable serialization

### `conformance-sep41`

Own:

* SEP-41-specific profile
* SEP-41 scenarios
* SEP-41 assertions
* SEP-41 specification metadata

## Acceptance criteria

The engine can execute one simple conformance test against a Soroban contract and produce a structured result.

Example:

```text
PASS: balance returns expected value
```

---

# Phase 3 — Reference SEP-41 Fixture

## Goal

Create a known-good contract implementation that the conformance suite can test.

## Tasks

Create a reference SEP-41 token fixture.

It should support the operations required by the selected SEP-41 profile.

The reference implementation is NOT the conformance engine.

It exists to demonstrate:

```text
Reference implementation
        ↓
Conformance suite
        ↓
All applicable tests pass
```

## Acceptance criteria

The reference contract can be deployed into the local test environment.

The conformance suite can initialize it using the setup mechanism.

At least the initial metadata, balance, and transfer tests pass.

---

# Phase 4 — SEP-41 Metadata and Balance Conformance

## Goal

Implement the first meaningful SEP-41 test group.

## Metadata

Test applicable SEP-41 metadata behavior:

* name
* symbol
* decimals

## Balance

Test:

* known initial balance
* zero balance
* balance changes after valid operations

## Requirements

Every test must have:

* stable test identifier
* description
* setup
* action
* expected observation
* actual observation
* pass/fail result

## Acceptance criteria

The reference fixture passes all implemented tests.

A deliberately broken fixture can cause at least one test to fail.

---

# Phase 5 — Transfer Conformance

## Goal

Implement comprehensive transfer behavior testing.

## Tests

At minimum:

* successful transfer
* sender balance decreases correctly
* receiver balance increases correctly
* zero-value transfer
* insufficient balance
* self-transfer
* invalid amount handling
* relevant authorization behavior

## State verification

Verify both pre-state and post-state where necessary.

Example:

```text
Before:
Alice = 100
Bob   = 20

Transfer:
Alice → Bob: 30

After:
Alice = 70
Bob   = 50
```

## Acceptance criteria

The suite detects a fixture that:

* fails to update the receiver
* deducts the wrong amount
* allows an invalid transfer
* produces incorrect state

---

# Phase 6 — Allowance and `transfer_from` Conformance

## Goal

Test delegated token spending.

## Tests

Implement applicable scenarios for:

* approve
* allowance
* transfer_from
* allowance reduction
* insufficient allowance
* expired allowance
* unauthorized spender
* boundary conditions

## Acceptance criteria

The suite verifies both:

```text
token balances
```

and:

```text
allowance state
```

after delegated transfers.

The suite must catch a deliberately broken allowance implementation.

---

# Phase 7 — Burn Conformance

## Goal

Test token destruction behavior.

## Tests

Implement applicable scenarios for:

* burn
* burn_from
* balance reduction
* authorization
* insufficient balance
* allowance interaction where applicable

## Acceptance criteria

The suite detects incorrect balance changes and invalid authorization.

---

# Phase 8 — Event Conformance

## Goal

Verify that observable events match the required behavior.

## Tests

Implement event assertions for applicable SEP-41 operations.

Verify:

* event topic/type
* relevant emitter
* event arguments
* relationship between event and state transition

## Principle

A contract must not be considered fully conformant merely because balances change correctly if required observable events are incorrect.

## Acceptance criteria

Create at least one intentionally broken fixture that performs the state transition correctly but emits an incorrect event.

The suite must detect it.

---

# Phase 9 — Authorization and Negative Testing

## Goal

Strengthen the suite against incorrect implementations.

## Test categories

Include:

* unauthorized calls
* invalid callers
* insufficient balance
* insufficient allowance
* invalid amounts
* invalid expiration
* boundary values
* repeated operations
* state consistency after failed operations

## Important requirement

Failed operations should be checked for unintended state changes.

Example:

```text
Invalid transfer attempted
        ↓
Transaction fails
        ↓
Balances must remain unchanged
```

## Acceptance criteria

The suite verifies both:

1. the invalid operation fails appropriately
2. contract state remains consistent afterward

---

# Phase 10 — Negative Fixture Suite

## Goal

Prove that the conformance framework catches real violations.

Create deliberately broken fixtures.

Suggested fixtures:

```text
fixtures/
├── valid/
├── broken-transfer/
├── broken-allowance/
├── broken-events/
├── broken-auth/
└── broken-state/
```

Each broken fixture should intentionally violate one or more known requirements.

## Acceptance criteria

The test suite must demonstrate:

```text
Valid implementation
        → PASS

Broken implementation
        → FAIL
```

The failure should identify the violated conformance requirement.

This phase is mandatory before declaring the MVP credible.

---

# Phase 11 — Reporting

## Goal

Make results useful to humans and tools.

## Human-readable output

Example:

```text
Stellar Contract Conformance
Profile: SEP-41
Version: 0.5.2

Metadata
  ✓ name
  ✓ symbol
  ✓ decimals

Transfer
  ✓ successful transfer
  ✓ insufficient balance
  ✓ zero transfer

Allowance
  ✓ approve
  ✓ transfer_from
  ✗ expired allowance

--------------------------------
12 passed
1 failed

STATUS: NON-CONFORMANT
```

## Machine-readable output

Provide a stable structured format such as JSON.

The exact schema must be defined in:

```text
docs/reporting.md
```

## Acceptance criteria

CI and external tools can consume the machine-readable result without parsing human-readable terminal output.

---

# Phase 12 — CI and Quality Gates

## Goal

Ensure every contribution is automatically validated.

CI should run:

```text
cargo fmt --check
cargo check
cargo test
cargo clippy
conformance fixture tests
```

Add appropriate Rust toolchain configuration.

## Quality requirements

The repository must maintain:

* reproducible builds
* deterministic tests
* documented supported Rust version
* clear error messages
* no committed secrets
* no network dependency for core tests

## Acceptance criteria

A clean pull request automatically runs the complete test suite.

A deliberately broken implementation causes CI to fail.

---

# Phase 13 — Documentation and Developer Experience

## Goal

Make the project usable by another Soroban developer without maintainer assistance.

Documentation must explain:

1. What the project is.
2. What conformance means.
3. What SEP-41 version is supported.
4. How to integrate a contract.
5. How to provide setup.
6. How to run the suite.
7. How to interpret results.
8. How to create a new conformance test.
9. How to create a fixture.
10. How to contribute.

Create examples showing the simplest supported workflow.

## Acceptance criteria

A developer unfamiliar with the repository can follow the documentation and execute the reference conformance suite successfully.

---

# Phase 14 — MVP Release

## Goal

Produce the first credible public release.

Before release:

* all MVP tests pass against the reference fixture
* broken fixtures are detected
* CI is green
* documentation is complete
* report format is stable
* architecture documentation matches implementation
* no known critical bugs remain
* scope remains limited to SEP-41

Tag:

```text
v0.1.0
```

The release should clearly state that conformance results are testing evidence, NOT a security audit or formal certification.

---

# Phase 15 — Wave Preparation

Only begin this phase after the MVP is working.

## Goal

Prepare the repository for external contributors through Drips Wave.

Create contributor issues based on actual repository needs.

Potential issues:

1. Add additional SEP-41 metadata edge cases.
2. Expand transfer conformance coverage.
3. Expand allowance expiration tests.
4. Add additional negative fixtures.
5. Improve event assertions.
6. Improve authorization tests.
7. Add JSON report improvements.
8. Improve documentation.
9. Add property-based transfer tests.
10. Add CLI support.
11. Add GitHub Action integration.
12. Add additional SEP-41 version support.

Every Wave issue must include:

* problem
* background
* expected implementation
* acceptance criteria
* tests
* dependencies
* relevant files
* contributor notes

Do not create artificial issues simply to increase the issue count.

---

# Phase 16 — CLI

Only begin after the core library is stable.

## Goal

Provide a convenient developer interface.

Potential command:

```bash
stellar-conform test --profile sep-41
```

The CLI must call the same conformance engine used by Rust integrations.

Do NOT duplicate conformance logic inside the CLI.

Architecture:

```text
CLI
 │
 ▼
Conformance Engine
 │
 ├── SEP-41 profile
 ├── setup
 ├── tests
 └── reporting
```

---

# Phase 17 — GitHub Action

Only begin after CLI and reporting are stable.

## Goal

Allow projects to run conformance testing automatically in CI.

Potential workflow:

```text
Pull Request
     ↓
GitHub Actions
     ↓
Build contract
     ↓
Run conformance profile
     ↓
Generate report
     ↓
Pass / Fail
```

The action must not contain duplicate conformance logic.

---

# Phase 18 — Property-Based Testing

Only begin after deterministic tests are mature.

## Goal

Discover edge cases that hand-written scenarios may miss.

Potential properties:

### Transfer conservation

For valid transfers:

```text
sender_after = sender_before - amount
receiver_after = receiver_before + amount
```

### Failed-operation stability

For invalid operations:

```text
state_after = state_before
```

unless the standard explicitly permits a state change.

### Allowance consistency

Verify that delegated spending obeys the applicable allowance rules.

Use Soroban-compatible property-based testing facilities where appropriate.

---

# Phase 19 — Additional Standards

**Status:** Formally Paused (as of Phase 19B Alternative Standard Research Audit).

Phase 19 is formally paused because no sufficiently mature second behavioral Stellar smart-contract standard currently exists.

After evaluating candidates including SEP-50, SEP-56, SEP-40, and SEP-57, it was determined that all relevant behavioral standards remain in "Draft" status. Implementing a conformance profile against Draft specifications creates a brittle, speculative test suite rather than authoritative validation infrastructure.

**Resumption Criteria:**
Phase 19 may only be reopened when a relevant Stellar smart-contract behavioral standard reaches **Active** or **Final** status and provides sufficiently stable normative requirements to support deterministic conformance testing. Do not implement against Draft SEPs.

**Flagship Profile Stability:**
SEP-41 remains the project's stable, flagship behavioral conformance profile. It should remain frozen to preserve reliability, and must not be modified unless a maintenance task identifies a concrete correctness issue.

---

# 5. Definition of Done

A feature is not complete merely because code compiles.

A feature is complete when:

* implementation exists
* tests exist
* relevant negative tests exist
* documentation exists
* behavior is deterministic
* CI passes
* acceptance criteria are satisfied
* no unrelated scope has been introduced

---

# 6. Agent Operating Rules

The coding agent MUST:

1. Read `ROADMAP.md` before starting work.
2. Identify the current phase.
3. Complete the current phase before starting the next.
4. Never silently expand scope.
5. Never introduce a new dependency without justification.
6. Prefer existing Soroban SDK functionality over custom infrastructure.
7. Never replace a specification requirement with an assumption.
8. Verify current SEP specifications before implementing standard-specific behavior.
9. Add tests with implementation changes.
10. Add negative tests where behavior can fail.
11. Keep commits logically scoped.
12. Update documentation when architecture changes.
13. Never claim conformance without executable evidence.
14. Never describe the project as an official Stellar Development Foundation project.
15. Never describe conformance testing as a security audit or certification.

---

# 7. Phase Completion Protocol

At the end of every phase, the agent must report:

```text
Phase:
Status:

Implemented:
- ...

Tests:
- ...

Documentation:
- ...

Known issues:
- ...

Files changed:
- ...

Next phase:
- ...
```

The agent must stop after the phase unless explicitly instructed to continue.

---

# 8. Current Starting Point

Current project status:

```text
Organization:
stellar-conformance-lab

Repository:
contract-conformance

Current phase:
Phase 0 — Repository Foundation

Current objective:
Prepare the repository and establish the project specification before implementing the conformance engine.
```

Do not begin Phase 1 until Phase 0 acceptance criteria are satisfied.

---

# 9. Immediate Next Action

The first implementation action is:

```text
Complete Phase 0.
```

After Phase 0 is complete, proceed to:

```text
Phase 1 — Architecture and Specification
```

Do not write the full conformance engine before the architecture and setup model have been documented and reviewed.

---

# 10. Long-Term Target

The long-term developer experience should eventually look approximately like:

```text
Developer
   │
   ▼
Build Soroban Contract
   │
   ▼
Select Conformance Profile
   │
   └── SEP-41 vX.Y
   │
   ▼
Provide Test Setup
   │
   ▼
Run Conformance Suite
   │
   ▼
┌────────────────────────────┐
│ Stellar Contract           │
│ Conformance Report         │
├────────────────────────────┤
│ Profile: SEP-41            │
│ Version: X.Y               │
│ Passed: 28                 │
│ Failed: 1                  │
│ Status: NON-CONFORMANT     │
└────────────────────────────┘
```

The project should ultimately become a reusable foundation for behavioral conformance testing across Stellar/Soroban standards while keeping the core engine independent from any single contract implementation.
