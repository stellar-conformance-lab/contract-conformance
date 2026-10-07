# Reporting Model

The reporting model defines how conformance results are presented. It must support both developers reading test output locally and automated systems (like CI/CD pipelines or GitHub Actions) consuming the results programmatically.

## Human-Readable Output

When run via a CLI or test runner, the engine should produce a concise, easy-to-read summary of the conformance status. 

Example:

```text
Stellar Contract Conformance
Profile: SEP-41
Version: 0.5.2

Metadata
  PASS  SEP41-META-001  Token name
  PASS  SEP41-META-002  Token symbol

Balance
  PASS  SEP41-BAL-001   Initial balance

Transfer
  PASS  SEP41-TRANSFER-001  Successful transfer
  FAIL  SEP41-TRANSFER-002  Zero-value transfer
  PASS  SEP41-TRANSFER-003  Insufficient balance

Allowance
  PASS  SEP41-ALLOWANCE-001  Approve allowance

Authorization
  FAIL  SEP41-AUTH-001  Unauthorized transfer

--------------------------------
6 passed
2 failed
0 errors

STATUS: NON-CONFORMANT
```

## Machine-Readable Output

For automation, the engine will output a structured JSON report. This schema must be stable and provide comprehensive metadata.

### Conceptual JSON Structure

```json
{
  "summary": {
    "profile": "SEP-41",
    "version": "0.5.2",
    "status": "NON-CONFORMANT",
    "passed": 6,
    "failed": 2,
    "errors": 0,
    "skipped": 0,
    "execution_time_ms": 142
  },
  "results": [
    {
      "test_id": "SEP41-TRANSFER-001",
      "description": "Successful transfer updates balances correctly",
      "status": "PASS",
      "expected_behavior": "Sender balance decreases by amount, receiver balance increases by amount",
      "observed_behavior": "Balances updated as expected",
      "failure_info": null,
      "execution_metadata": {
        "cpu_instructions": 45000,
        "mem_bytes": 1024
      }
    },
    {
      "test_id": "SEP41-TRANSFER-002",
      "description": "Zero-value transfer",
      "status": "FAIL",
      "expected_behavior": "Transaction succeeds with no balance changes",
      "observed_behavior": "Transaction panicked with 'Invalid amount'",
      "failure_info": {
        "reason": "Contract rejected a 0-amount transfer which is permitted by SEP-41",
        "trace": "..."
      },
      "execution_metadata": {
        "cpu_instructions": 12000,
        "mem_bytes": 512
      }
    }
  ]
}
```

### Key Elements
- **Profile and Version:** Exactly identifies the standard being tested.
- **Test ID:** The stable identifier (e.g., `SEP41-TRANSFER-001`).
- **Status:** `PASS`, `FAIL`, `ERROR`, or `SKIPPED`.
- **Failure Info:** If a test fails, this provides detailed context, trace, or differences between expected and actual state, preventing developers from having to blindly guess what went wrong. 

This model ensures the core engine does not need to be rewritten when adding a CLI tool or GitHub Action in the future.
