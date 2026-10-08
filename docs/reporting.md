# Reporting Model

The reporting model defines how conformance results are presented, explicitly isolating reporting logic from core conformance assertions. It produces deterministic JSON and human-readable text representations natively from an execution run.

## Result Statuses

A scenario result has one of the following explicitly defined statuses:

* **PASS**: The contract was evaluated and completely fulfilled the standard requirement.
* **FAIL**: The contract was evaluated but violated the standard behavior. This is an actual conformance failure (e.g. incorrect balance state, wrong event emitted).
* **ERROR**: An infrastructure, test harness, or fixture-setup error prevented the scenario from properly evaluating the contract. This is distinct from a conformance failure.
* **SKIPPED**: The scenario could not or was not evaluated.

**Important Note for Negative Fixtures:** A deliberately broken test fixture might result in a `FAIL`. Within the reporting model, this is still reported as a `FAIL`. It is the job of the *testing harness* (the caller) to intercept the `FAIL` and recognize it as an expected outcome. The report natively documents reality, without implicitly remapping statuses.

## Scenario Result Structure

Each evaluated scenario emits a `ScenarioResult` object capturing its identity and outcome:

* `test_id`: Stable identifier (e.g., `SEP41-TRANSFER-001`).
* `description`: Short title of the evaluated capability.
* `status`: `PASS`, `FAIL`, `ERROR`, or `SKIPPED`.
* `expected_behavior`: Describes the core requirement being validated.
* `observed_behavior`: Provides details on the observed failure when `status` is `FAIL` or `ERROR`.

## Aggregate Report

A full conformance execution run produces a `ConformanceReport` capturing all scenario outcomes, categorized functionally by standard and implementation fixture.

* `profile`: Standard profile executed (e.g. "SEP-41").
* `fixture`: The fixture implementing the test logic.
* `summary`: Contains counts for total, passed, failed, errors, and skipped scenarios.
* `status`: The deterministic overall status derived from scenario results.
* `results`: An array of sequentially ordered `ScenarioResult` items.

### Summary Counts

```text
total = passed + failed + errors + skipped
```

### Overall-Status Semantics

The `ConformanceReport.status` evaluates deterministically:

1. If any scenario is `ERROR` -> Overall is **ERROR**.
2. If any scenario is `FAIL` -> Overall is **FAIL**.
3. If total scenarios evaluated == 0 or all are `SKIPPED` -> Overall is **SKIPPED**.
4. Otherwise -> Overall is **PASS**.

## Formats

### Machine-Readable (JSON)

The core `conformance-report` crate supports generating stable JSON formatted strings mapping directly to the `ConformanceReport` struct via `serde` serialization. 

Example:

```json
{
  "profile": "SEP-41",
  "fixture": "valid",
  "status": "FAIL",
  "summary": {
    "total": 1,
    "passed": 0,
    "failed": 1,
    "errors": 0,
    "skipped": 0
  },
  "results": [
    {
      "test_id": "SEP41-TRANSFER-001",
      "description": "Successful transfer updates balances correctly",
      "status": "FAIL",
      "expected_behavior": "Sender balance decreases by amount, receiver balance increases by amount",
      "observed_behavior": "Balances mutated without authorization"
    }
  ]
}
```

### Human-Readable (Text)

Example Output:

```text
Stellar Contract Conformance Report

Profile: SEP-41
Fixture: valid

FAIL   SEP41-TRANSFER-001    Successful transfer updates balances correctly
         Requirement: Sender balance decreases by amount, receiver balance increases by amount
         Message:     Balances mutated without authorization

Summary:
  Total: 1
  Passed: 0
  Failed: 1
  Errors: 0
  Skipped: 0

Overall: FAIL
```
