# Setup and Fixtures

This is a critical component of the architecture. It answers the fundamental question:

> **How can one reusable conformance suite test different SEP-41 contract implementations when each implementation may require different initialization state?**

The answer is the **Explicit Setup/Fixture Abstraction**. The conformance framework cannot and should not magically know how to mint tokens, configure administrators, or bypass authorizations for arbitrary contracts. Instead, it delegates these implementation-specific prerequisites to an explicitly provided setup fixture.

## The Setup Mechanism

The architecture requires the user to provide a fixture implementation that satisfies a specific trait or interface defined by the Conformance Profile. 

Conceptually, testing a contract looks like this:

```rust
Sep41Suite::new(&env, contract_id)
    .with_setup(MyCustomContractSetup::new())
    .run();
```

### Addressing Initialization Requirements

The setup abstraction is responsible for executing all necessary preconditions before a test scenario runs, including:
- **Account creation:** Generating required `Address` types for Alice, Bob, etc.
- **Contract registration/installation:** Deploying the WASM to the local test environment.
- **Token initialization:** Calling the contract's unique `initialize(admin, decimals, name, symbol)` function.
- **Initial balances:** Minting tokens to specific addresses so scenarios have a starting balance to transfer or burn.
- **Administrator configuration:** Holding the admin credentials required to perform setup actions.
- **Allowances and Authorization:** Configuring predefined allowances if a scenario requires a specific pre-state.

## Ownership and Responsibilities

- **The Fixture owns:** The knowledge of *how* to deploy the contract, *how* to mint tokens, and *how* to authenticate administrative actions. It executes setup logic when prompted by the engine.
- **The Conformance Engine owns:** The `Env`, the scenarios, test isolation, assertions, and the execution flow. It tells the fixture *what* state is needed (e.g., "I need Alice to have 100 tokens"), but not *how* to achieve it.
- **The Contract Under Test owns:** Its internal state and business logic.

## Execution and Isolation

- **Deterministic Setup:** Setup must be highly deterministic. It operates entirely within the local Soroban `Env`.
- **Test Isolation:** Setup is **not** reusable across tests in the same `Env`. To prevent state leakage, every test scenario runs in a completely fresh, isolated `Env`, and the fixture is executed from scratch for each test.
- **Setup Failures:** If the setup fixture fails (e.g., it panics while trying to mint tokens), the Conformance Engine catches this and marks the test result as an `ERROR` (not a `FAIL`), clearly distinguishing a test infrastructure issue from a contract conformance violation.

## Supporting Known-Good and Broken Fixtures

Because the Setup abstraction is an injected dependency, the framework can easily test both standard implementations and deliberately broken ones. We simply pass a different contract ID and a matching Setup fixture into the `Sep41Suite`. 

This native Rust/Soroban approach completely avoids the need for a custom YAML DSL while maintaining total flexibility.
