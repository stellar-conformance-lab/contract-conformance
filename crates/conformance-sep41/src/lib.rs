#![no_std]

use conformance_core::scenario::Scenario;
use conformance_core::result::{TestResult, Status};
use soroban_sdk::{token::Client as TokenClient, Address, Env};

pub trait Sep41Fixture: conformance_core::fixture::Fixture {
    fn token_contract_id(&self) -> &Address;
    fn test_account_1(&self) -> &Address;
    fn test_account_2(&self) -> &Address;
    fn expected_initial_balance(&self) -> i128;
}

// -----------------------------------------------------------------------------
// METADATA CONFORMANCE
// -----------------------------------------------------------------------------

// SEP-41 Requirement: `name() -> String`
// The standard dictates the interface but imposes no constraint on the returned value.
// This test is intentionally limited to verifying that the method exists and returns without panic.
pub struct MetaNameScenario;
impl<F: Sep41Fixture> Scenario<F> for MetaNameScenario {
    fn id(&self) -> &'static str { "SEP41-META-001" }
    fn description(&self) -> &'static str { "Token name" }
    fn run(&self, env: &Env, fixture: &F) -> TestResult {
        let client = TokenClient::new(env, fixture.token_contract_id());
        let _name = client.name(); // Verify no panic and valid type
        TestResult {
            test_id: self.id(),
            description: self.description(),
            status: Status::Pass,
            expected_behavior: "Token name function executes successfully",
            observed_behavior: "Token name returned without error",
        }
    }
}

// SEP-41 Requirement: `symbol() -> String`
// The standard dictates the interface but imposes no constraint on the returned value.
// This test is intentionally limited to verifying that the method exists and returns without panic.
pub struct MetaSymbolScenario;
impl<F: Sep41Fixture> Scenario<F> for MetaSymbolScenario {
    fn id(&self) -> &'static str { "SEP41-META-002" }
    fn description(&self) -> &'static str { "Token symbol" }
    fn run(&self, env: &Env, fixture: &F) -> TestResult {
        let client = TokenClient::new(env, fixture.token_contract_id());
        let _symbol = client.symbol();
        TestResult {
            test_id: self.id(),
            description: self.description(),
            status: Status::Pass,
            expected_behavior: "Token symbol function executes successfully",
            observed_behavior: "Token symbol returned without error",
        }
    }
}

// SEP-41 Requirement: `decimals() -> u32`
// The standard dictates the interface but imposes no constraint on the returned value.
// This test is intentionally limited to verifying that the method exists and returns without panic.
pub struct MetaDecimalsScenario;
impl<F: Sep41Fixture> Scenario<F> for MetaDecimalsScenario {
    fn id(&self) -> &'static str { "SEP41-META-003" }
    fn description(&self) -> &'static str { "Token decimals" }
    fn run(&self, env: &Env, fixture: &F) -> TestResult {
        let client = TokenClient::new(env, fixture.token_contract_id());
        let _decimals = client.decimals();
        TestResult {
            test_id: self.id(),
            description: self.description(),
            status: Status::Pass,
            expected_behavior: "Token decimals function executes successfully",
            observed_behavior: "Token decimals returned without error",
        }
    }
}

// -----------------------------------------------------------------------------
// BALANCE CONFORMANCE
// -----------------------------------------------------------------------------

// SEP-41 Requirement: `balance(id: Address) -> i128`
// The standard requires the balance method to accurately return the current balance.
pub struct BalInitialScenario;
impl<F: Sep41Fixture> Scenario<F> for BalInitialScenario {
    fn id(&self) -> &'static str { "SEP41-BAL-001" }
    fn description(&self) -> &'static str { "Initial balance" }
    fn run(&self, env: &Env, fixture: &F) -> TestResult {
        let client = TokenClient::new(env, fixture.token_contract_id());
        let account = fixture.test_account_1();
        let expected = fixture.expected_initial_balance();
        
        let balance = client.balance(account);
        
        if balance == expected {
            TestResult {
                test_id: self.id(),
                description: self.description(),
                status: Status::Pass,
                expected_behavior: "Account has expected initial balance",
                observed_behavior: "Balance matched expected value",
            }
        } else {
            TestResult {
                test_id: self.id(),
                description: self.description(),
                status: Status::Fail,
                expected_behavior: "Account has expected initial balance",
                observed_behavior: "Balance did not match expected value",
            }
        }
    }
}

// SEP-41 Requirement: `balance(id: Address) -> i128`
// The standard implicitly requires that unfunded accounts report 0 balance.
pub struct BalZeroScenario;
impl<F: Sep41Fixture> Scenario<F> for BalZeroScenario {
    fn id(&self) -> &'static str { "SEP41-BAL-002" }
    fn description(&self) -> &'static str { "Zero balance" }
    fn run(&self, env: &Env, fixture: &F) -> TestResult {
        let client = TokenClient::new(env, fixture.token_contract_id());
        let account = fixture.test_account_2();
        
        let balance = client.balance(account);
        
        if balance == 0 {
            TestResult {
                test_id: self.id(),
                description: self.description(),
                status: Status::Pass,
                expected_behavior: "New account balance is zero",
                observed_behavior: "Balance is exactly zero",
            }
        } else {
            TestResult {
                test_id: self.id(),
                description: self.description(),
                status: Status::Fail,
                expected_behavior: "New account balance is zero",
                observed_behavior: "Balance is non-zero",
            }
        }
    }
}

// -----------------------------------------------------------------------------
// TRANSFER CONFORMANCE
// -----------------------------------------------------------------------------

// SEP-41 Requirement: `transfer(from: Address, to: Address, amount: i128)`
// Transfer moves amount from `from` to `to`.
pub struct TransferSuccessScenario;
impl<F: Sep41Fixture> Scenario<F> for TransferSuccessScenario {
    fn id(&self) -> &'static str { "SEP41-TRANSFER-001" }
    fn description(&self) -> &'static str { "Successful transfer" }
    fn run(&self, env: &Env, fixture: &F) -> TestResult {
        let client = TokenClient::new(env, fixture.token_contract_id());
        let alice = fixture.test_account_1();
        let bob = fixture.test_account_2();
        
        let initial_alice = client.balance(alice);
        let initial_bob = client.balance(bob);
        let transfer_amount = 50_i128;

        if initial_alice < transfer_amount {
            return TestResult {
                test_id: self.id(),
                description: self.description(),
                status: Status::Error,
                expected_behavior: "Fixture must have enough balance",
                observed_behavior: "Insufficient balance to perform test",
            };
        }

        env.mock_all_auths();
        client.transfer(alice, bob, &transfer_amount);

        let final_alice = client.balance(alice);
        let final_bob = client.balance(bob);

        if final_alice == initial_alice - transfer_amount && final_bob == initial_bob + transfer_amount {
            TestResult {
                test_id: self.id(),
                description: self.description(),
                status: Status::Pass,
                expected_behavior: "Balances update correctly based on transfer",
                observed_behavior: "Balances updated correctly",
            }
        } else {
            TestResult {
                test_id: self.id(),
                description: self.description(),
                status: Status::Fail,
                expected_behavior: "Balances update correctly based on transfer",
                observed_behavior: "Balances did not update correctly",
            }
        }
    }
}

// SEP-41 Requirement: Transfer event emission
// Topics: ["transfer", from, to]
// Data: amount
pub struct TransferEventScenario;
impl<F: Sep41Fixture> Scenario<F> for TransferEventScenario {
    fn id(&self) -> &'static str { "SEP41-TRANSFER-002" }
    fn description(&self) -> &'static str { "Transfer event emission" }
    fn run(&self, env: &Env, fixture: &F) -> TestResult {
        let client = TokenClient::new(env, fixture.token_contract_id());
        let alice = fixture.test_account_1();
        let bob = fixture.test_account_2();
        let transfer_amount = 10_i128;

        env.mock_all_auths();
        client.transfer(alice, bob, &transfer_amount);

        let events = env.events().all();
        let mut found = false;

        let transfer_symbol = soroban_sdk::Symbol::new(env, "transfer");

        for (contract_id, topics, _data) in events.into_iter() {
            if contract_id == *fixture.token_contract_id() {
                // The current specification permits additional topics.
                // We verify that "transfer" exists among the topics.
                for topic in topics.into_iter() {
                    if topic.to_val() == transfer_symbol.to_val() {
                        found = true;
                        break;
                    }
                }
            }
        }

        if found {
            TestResult {
                test_id: self.id(),
                description: self.description(),
                status: Status::Pass,
                expected_behavior: "Emits transfer event",
                observed_behavior: "Transfer event found",
            }
        } else {
            TestResult {
                test_id: self.id(),
                description: self.description(),
                status: Status::Fail,
                expected_behavior: "Emits transfer event",
                observed_behavior: "Transfer event missing",
            }
        }
    }
}

// SEP-41 Requirement: Insufficient balance
// Operation fails/traps; insufficient-balance condition is enforced; state is reverted.
pub struct TransferInsufficientBalanceScenario;
impl<F: Sep41Fixture> Scenario<F> for TransferInsufficientBalanceScenario {
    fn id(&self) -> &'static str { "SEP41-TRANSFER-004" }
    fn description(&self) -> &'static str { "Insufficient balance" }
    fn run(&self, env: &Env, fixture: &F) -> TestResult {
        let client = TokenClient::new(env, fixture.token_contract_id());
        let alice = fixture.test_account_1();
        let bob = fixture.test_account_2();
        
        let initial_alice = client.balance(alice);
        let initial_bob = client.balance(bob);
        let transfer_amount = initial_alice + 1; // More than available

        env.mock_all_auths();
        
        // Attempt transfer. It must fail.
        let result = client.try_transfer(alice, bob, &transfer_amount);

        let final_alice = client.balance(alice);
        let final_bob = client.balance(bob);

        if result.is_ok() {
            return TestResult {
                test_id: self.id(),
                description: self.description(),
                status: Status::Fail,
                expected_behavior: "Transfer fails when balance is insufficient",
                observed_behavior: "Transfer succeeded unexpectedly",
            };
        }

        if final_alice != initial_alice || final_bob != initial_bob {
            return TestResult {
                test_id: self.id(),
                description: self.description(),
                status: Status::Fail,
                expected_behavior: "Balances remain unchanged after failure",
                observed_behavior: "Balances were mutated despite failed transfer",
            };
        }

        TestResult {
            test_id: self.id(),
            description: self.description(),
            status: Status::Pass,
            expected_behavior: "Transfer fails and balances are preserved",
            observed_behavior: "Transfer failed as expected without mutating state",
        }
    }
}

// SEP-41 Requirement: Authorization
// The `from` address must authorize the transfer.
pub struct TransferAuthorizationScenario;
impl<F: Sep41Fixture> Scenario<F> for TransferAuthorizationScenario {
    fn id(&self) -> &'static str { "SEP41-TRANSFER-005" }
    fn description(&self) -> &'static str { "Transfer authorization" }
    fn run(&self, env: &Env, fixture: &F) -> TestResult {
        let client = TokenClient::new(env, fixture.token_contract_id());
        let alice = fixture.test_account_1();
        let bob = fixture.test_account_2();
        let transfer_amount = 10_i128;

        // We DO NOT mock auths here.
        // We attempt a transfer. It should fail due to missing authorization.
        let result = client.try_transfer(alice, bob, &transfer_amount);

        if result.is_ok() {
            return TestResult {
                test_id: self.id(),
                description: self.description(),
                status: Status::Fail,
                expected_behavior: "Transfer requires Soroban authorization from sender",
                observed_behavior: "Transfer succeeded without authorization",
            };
        }

        TestResult {
            test_id: self.id(),
            description: self.description(),
            status: Status::Pass,
            expected_behavior: "Transfer requires authorization",
            observed_behavior: "Transfer failed when unauthorized",
        }
    }
}

// SEP-41 Requirement: Negative amount
// Standard mandates failure for negative transfer amounts.
pub struct TransferNegativeAmountScenario;
impl<F: Sep41Fixture> Scenario<F> for TransferNegativeAmountScenario {
    fn id(&self) -> &'static str { "SEP41-TRANSFER-007" }
    fn description(&self) -> &'static str { "Negative amount transfer" }
    fn run(&self, env: &Env, fixture: &F) -> TestResult {
        let client = TokenClient::new(env, fixture.token_contract_id());
        let alice = fixture.test_account_1();
        let bob = fixture.test_account_2();
        
        let initial_alice = client.balance(alice);
        let transfer_amount = -10_i128;

        env.mock_all_auths();
        
        let result = client.try_transfer(alice, bob, &transfer_amount);
        let final_alice = client.balance(alice);

        if result.is_ok() {
            return TestResult {
                test_id: self.id(),
                description: self.description(),
                status: Status::Fail,
                expected_behavior: "Transfer fails with negative amount",
                observed_behavior: "Transfer succeeded with negative amount",
            };
        }

        if initial_alice != final_alice {
            return TestResult {
                test_id: self.id(),
                description: self.description(),
                status: Status::Fail,
                expected_behavior: "Balances remain unchanged",
                observed_behavior: "Balances mutated on negative transfer",
            };
        }

        TestResult {
            test_id: self.id(),
            description: self.description(),
            status: Status::Pass,
            expected_behavior: "Transfer fails and state is preserved",
            observed_behavior: "Transfer failed as expected",
        }
    }
}

// SEP41-TRANSFER-003 (zero transfer) and SEP41-TRANSFER-006 (self transfer) are intentionally omitted.
// The current authoritative SEP-41 specification does not strictly define behavioral 
// conformance constraints beyond generic immutability for these specific edge cases.
