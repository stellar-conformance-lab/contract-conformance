# Conformance Model

This document defines what "conformance" means within the context of the Stellar Contract Conformance project.

> Passing the suite is evidence of behavioral conformance against a particular profile and version. It is not a security audit, formal verification, certification, or guarantee that a contract contains no vulnerabilities.

## Scope of Testing

### What the Suite Tests
- **Behavioral Conformance:** The suite verifies that the contract behaves exactly as mandated by the standard under various scenarios.
- **State-transition Verification:** Ensures that operations modify the contract's state (e.g., balances, allowances) correctly.
- **Authorization Verification:** Ensures that operations requiring authorization enforce it, and unauthorized operations fail without causing side effects.
- **Event Verification:** Ensures that correct events are emitted with the right topics and arguments during state transitions.
- **Edge-case Behavior:** Tests limits, zero-values, and boundary conditions explicitly mentioned in or implied by the specification.
- **Negative Testing:** Ensures that deliberately broken implementations or invalid operations are caught and handled correctly.

### What the Suite Does NOT Test
- **Interface Compatibility Only:** Merely possessing a function with the correct signature is insufficient; the function must perform the correct logic.
- **Security Vulnerabilities:** We do not scan for reentrancy, integer overflow (beyond standard Rust/Soroban protections), or business-logic flaws outside the standard's scope.
- **Arbitrary Functionality:** Any custom extensions to a contract are ignored and not evaluated.
- **Live Network State:** All tests are deterministic and local. We do not evaluate contracts deployed on Testnet or Mainnet.

## Deterministic Execution
Tests execute locally using the Soroban test environment (`soroban_sdk::Env`). There are no network calls, RPC usage, or external dependencies. 

## Versioned Conformance Profiles
Standards evolve. A conformance result is meaningless without a version. The model enforces that every test suite corresponds to a specific version of a specification (e.g., SEP-41 v0.5.2).

## Test Lifecycle

Every conformance test follows this strict lifecycle:

```text
Load contract
→ Create test environment
→ Apply setup fixture
→ Execute scenario
→ Observe state/events/results
→ Evaluate assertions
→ Record result
→ Produce report
```

## Result Definitions

A test execution results in one of the following statuses:

- **PASS:** The contract exhibited the exact expected behavior, state changes, and events for the scenario.
- **FAIL:** The contract violated the expected behavior (e.g., incorrect balance, missing event, unexpected success on invalid input).
- **ERROR:** The test itself encountered an unexpected panic or setup failure that prevented the scenario from running properly (e.g., failure to mint initial tokens via the setup fixture).
- **SKIPPED:** The test was bypassed (useful for optional features of a standard, if applicable).

## Stable Test Identifiers

To ensure results can be tracked reliably across versions and tools, tests use stable, categorized identifiers.

Examples:
- `SEP41-META-001`
- `SEP41-BAL-001`
- `SEP41-TRANSFER-001`
- `SEP41-ALLOWANCE-001`
- `SEP41-AUTH-001`
