#![no_std]
use soroban_sdk::testutils::Events;

use conformance_core::result::{Status, TestResult};
use conformance_core::scenario::Scenario;
use soroban_sdk::testutils::Ledger;
use soroban_sdk::IntoVal;
use soroban_sdk::{token::Client as TokenClient, Address, Env, TryFromVal};

pub trait Sep41Fixture: conformance_core::fixture::Fixture {
    fn token_contract_id(&self) -> &Address;
    fn test_account_1(&self) -> &Address;
    fn test_account_2(&self) -> &Address;
    fn test_account_3(&self) -> &Address;
    fn expected_initial_balance(&self) -> i128;
}

pub mod event_helpers {
    use soroban_sdk::{Env, Map, Symbol, TryFromVal, Val, Vec as SorobanVec};

    pub fn verify_amount_data(env: &Env, data: &Val, expected_amount: i128) -> bool {
        if let Ok(amount) = i128::try_from_val(env, data) {
            return amount == expected_amount;
        }

        if let Ok(map) = Map::<Symbol, Val>::try_from_val(env, data) {
            if let Some(val) = map.get(Symbol::new(env, "amount")) {
                if let Ok(amount) = i128::try_from_val(env, &val) {
                    return amount == expected_amount;
                }
            }
        }
        false
    }

    pub fn verify_approve_data(
        env: &Env,
        data: &Val,
        expected_amount: i128,
        expected_expiration: u32,
    ) -> bool {
        if let Ok(vec) = SorobanVec::<Val>::try_from_val(env, data) {
            if vec.len() >= 2 {
                let mut iter = vec.into_iter();
                let v0 = iter.next().unwrap();
                let v1 = iter.next().unwrap();
                if let Ok(amount) = i128::try_from_val(env, &v0) {
                    if let Ok(expiration) = u32::try_from_val(env, &v1) {
                        if amount == expected_amount && expiration == expected_expiration {
                            return true;
                        }
                    }
                }
            }
        }

        if let Ok(map) = Map::<Symbol, Val>::try_from_val(env, data) {
            if let Some(val_amount) = map.get(Symbol::new(env, "amount")) {
                if let Some(val_exp) = map.get(Symbol::new(env, "live_until_ledger")) {
                    if let Ok(amount) = i128::try_from_val(env, &val_amount) {
                        if let Ok(expiration) = u32::try_from_val(env, &val_exp) {
                            return amount == expected_amount && expiration == expected_expiration;
                        }
                    }
                }
            }
        }
        false
    }
}

// -----------------------------------------------------------------------------
// METADATA CONFORMANCE
// -----------------------------------------------------------------------------

pub struct MetaNameScenario;
impl<F: Sep41Fixture> Scenario<F> for MetaNameScenario {
    fn id(&self) -> &'static str {
        "SEP41-META-001"
    }
    fn description(&self) -> &'static str {
        "Token name"
    }
    fn run(&self, env: &Env, fixture: &F) -> TestResult {
        let client = TokenClient::new(env, fixture.token_contract_id());
        let _name = client.name();
        TestResult {
            test_id: <Self as Scenario<F>>::id(self),
            description: <Self as Scenario<F>>::description(self),
            status: Status::Pass,
            expected_behavior: "Token name function executes successfully",
            observed_behavior: "Token name returned without error",
        }
    }
}

pub struct MetaSymbolScenario;
impl<F: Sep41Fixture> Scenario<F> for MetaSymbolScenario {
    fn id(&self) -> &'static str {
        "SEP41-META-002"
    }
    fn description(&self) -> &'static str {
        "Token symbol"
    }
    fn run(&self, env: &Env, fixture: &F) -> TestResult {
        let client = TokenClient::new(env, fixture.token_contract_id());
        let _symbol = client.symbol();
        TestResult {
            test_id: <Self as Scenario<F>>::id(self),
            description: <Self as Scenario<F>>::description(self),
            status: Status::Pass,
            expected_behavior: "Token symbol function executes successfully",
            observed_behavior: "Token symbol returned without error",
        }
    }
}

pub struct MetaDecimalsScenario;
impl<F: Sep41Fixture> Scenario<F> for MetaDecimalsScenario {
    fn id(&self) -> &'static str {
        "SEP41-META-003"
    }
    fn description(&self) -> &'static str {
        "Token decimals"
    }
    fn run(&self, env: &Env, fixture: &F) -> TestResult {
        let client = TokenClient::new(env, fixture.token_contract_id());
        let _decimals = client.decimals();
        TestResult {
            test_id: <Self as Scenario<F>>::id(self),
            description: <Self as Scenario<F>>::description(self),
            status: Status::Pass,
            expected_behavior: "Token decimals function executes successfully",
            observed_behavior: "Token decimals returned without error",
        }
    }
}

// -----------------------------------------------------------------------------
// BALANCE CONFORMANCE
// -----------------------------------------------------------------------------

pub struct BalInitialScenario;
impl<F: Sep41Fixture> Scenario<F> for BalInitialScenario {
    fn id(&self) -> &'static str {
        "SEP41-BAL-001"
    }
    fn description(&self) -> &'static str {
        "Initial balance"
    }
    fn run(&self, env: &Env, fixture: &F) -> TestResult {
        let client = TokenClient::new(env, fixture.token_contract_id());
        let account = fixture.test_account_1();
        let expected = fixture.expected_initial_balance();
        let balance = client.balance(account);
        if balance == expected {
            TestResult {
                test_id: <Self as Scenario<F>>::id(self),
                description: <Self as Scenario<F>>::description(self),
                status: Status::Pass,
                expected_behavior: "Account has expected initial balance",
                observed_behavior: "Balance matched expected value",
            }
        } else {
            TestResult {
                test_id: <Self as Scenario<F>>::id(self),
                description: <Self as Scenario<F>>::description(self),
                status: Status::Fail,
                expected_behavior: "Account has expected initial balance",
                observed_behavior: "Balance did not match expected value",
            }
        }
    }
}

pub struct BalZeroScenario;
impl<F: Sep41Fixture> Scenario<F> for BalZeroScenario {
    fn id(&self) -> &'static str {
        "SEP41-BAL-002"
    }
    fn description(&self) -> &'static str {
        "Zero balance"
    }
    fn run(&self, env: &Env, fixture: &F) -> TestResult {
        let client = TokenClient::new(env, fixture.token_contract_id());
        let account = fixture.test_account_2();
        let balance = client.balance(account);
        if balance == 0 {
            TestResult {
                test_id: <Self as Scenario<F>>::id(self),
                description: <Self as Scenario<F>>::description(self),
                status: Status::Pass,
                expected_behavior: "New account balance is zero",
                observed_behavior: "Balance is exactly zero",
            }
        } else {
            TestResult {
                test_id: <Self as Scenario<F>>::id(self),
                description: <Self as Scenario<F>>::description(self),
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

pub struct TransferSuccessScenario;
impl<F: Sep41Fixture> Scenario<F> for TransferSuccessScenario {
    fn id(&self) -> &'static str {
        "SEP41-TRANSFER-001"
    }
    fn description(&self) -> &'static str {
        "Successful transfer"
    }
    fn run(&self, env: &Env, fixture: &F) -> TestResult {
        let client = TokenClient::new(env, fixture.token_contract_id());
        let alice = fixture.test_account_1();
        let bob = fixture.test_account_2();
        let initial_alice = client.balance(alice);
        let initial_bob = client.balance(bob);
        let transfer_amount = 50_i128;

        if initial_alice < transfer_amount {
            return TestResult {
                test_id: <Self as Scenario<F>>::id(self),
                description: <Self as Scenario<F>>::description(self),
                status: Status::Error,
                expected_behavior: "Fixture must have enough balance",
                observed_behavior: "Insufficient balance to perform test",
            };
        }

        env.mock_auths(&[soroban_sdk::testutils::MockAuth {
            address: alice,
            invoke: &soroban_sdk::testutils::MockAuthInvoke {
                contract: fixture.token_contract_id(),
                fn_name: "transfer",
                args: (alice, bob, &transfer_amount).into_val(env),
                sub_invokes: &[],
            },
        }]);
        client.transfer(alice, bob, &transfer_amount);

        let final_alice = client.balance(alice);
        let final_bob = client.balance(bob);

        if final_alice == initial_alice - transfer_amount
            && final_bob == initial_bob + transfer_amount
        {
            TestResult {
                test_id: <Self as Scenario<F>>::id(self),
                description: <Self as Scenario<F>>::description(self),
                status: Status::Pass,
                expected_behavior: "Balances update correctly based on transfer",
                observed_behavior: "Balances updated correctly",
            }
        } else {
            TestResult {
                test_id: <Self as Scenario<F>>::id(self),
                description: <Self as Scenario<F>>::description(self),
                status: Status::Fail,
                expected_behavior: "Balances update correctly based on transfer",
                observed_behavior: "Balances did not update correctly",
            }
        }
    }
}

pub struct TransferEventScenario;
impl<F: Sep41Fixture> Scenario<F> for TransferEventScenario {
    fn id(&self) -> &'static str {
        "SEP41-TRANSFER-002"
    }
    fn description(&self) -> &'static str {
        "Transfer event emission"
    }
    fn run(&self, env: &Env, fixture: &F) -> TestResult {
        let client = TokenClient::new(env, fixture.token_contract_id());
        let alice = fixture.test_account_1();
        let bob = fixture.test_account_2();
        let transfer_amount = 10_i128;

        env.mock_auths(&[soroban_sdk::testutils::MockAuth {
            address: alice,
            invoke: &soroban_sdk::testutils::MockAuthInvoke {
                contract: fixture.token_contract_id(),
                fn_name: "transfer",
                args: (alice, bob, &transfer_amount).into_val(env),
                sub_invokes: &[],
            },
        }]);
        client.transfer(alice, bob, &transfer_amount);

        let events = env.events().all();
        let mut found = false;
        let transfer_symbol = soroban_sdk::Symbol::new(env, "transfer");

        let filtered = events.filter_by_contract(fixture.token_contract_id());
        for event in filtered.events() {
            let soroban_sdk::xdr::ContractEventBody::V0(body) = &event.body;
            let Ok(topics) = soroban_sdk::Vec::<soroban_sdk::Val>::try_from_val(env, &body.topics)
            else {
                continue;
            };
            let Ok(data) = soroban_sdk::Val::try_from_val(env, &body.data) else {
                continue;
            };
            let contract_id = fixture.token_contract_id().clone();
            if contract_id == *fixture.token_contract_id() && topics.len() >= 3 {
                let mut iter = topics.into_iter();
                let t0 = iter.next();
                let t1 = iter.next();
                let t2 = iter.next();
                if let (Some(t0), Some(t1), Some(t2)) = (t0, t1, t2) {
                    if let (Ok(sym), Ok(from), Ok(to)) = (
                        soroban_sdk::Symbol::try_from_val(env, &t0),
                        soroban_sdk::Address::try_from_val(env, &t1),
                        soroban_sdk::Address::try_from_val(env, &t2),
                    ) {
                        if sym == transfer_symbol
                            && &from == alice
                            && &to == bob
                            && crate::event_helpers::verify_amount_data(env, &data, transfer_amount)
                        {
                            found = true;
                            break;
                        }
                    }
                }
            }
        }

        if found {
            TestResult {
                test_id: <Self as Scenario<F>>::id(self),
                description: <Self as Scenario<F>>::description(self),
                status: Status::Pass,
                expected_behavior: "Emits transfer event with correct topics and data",
                observed_behavior: "Transfer event found matching specification",
            }
        } else {
            TestResult {
                test_id: <Self as Scenario<F>>::id(self),
                description: <Self as Scenario<F>>::description(self),
                status: Status::Fail,
                expected_behavior: "Emits transfer event with correct topics and data",
                observed_behavior: "Transfer event missing or lacked required data",
            }
        }
    }
}

pub struct TransferInsufficientBalanceScenario;
impl<F: Sep41Fixture> Scenario<F> for TransferInsufficientBalanceScenario {
    fn id(&self) -> &'static str {
        "SEP41-TRANSFER-004"
    }
    fn description(&self) -> &'static str {
        "Insufficient balance"
    }
    fn run(&self, env: &Env, fixture: &F) -> TestResult {
        let client = TokenClient::new(env, fixture.token_contract_id());
        let alice = fixture.test_account_1();
        let bob = fixture.test_account_2();
        let initial_alice = client.balance(alice);
        let initial_bob = client.balance(bob);
        let transfer_amount = initial_alice + 1;

        env.mock_auths(&[soroban_sdk::testutils::MockAuth {
            address: alice,
            invoke: &soroban_sdk::testutils::MockAuthInvoke {
                contract: fixture.token_contract_id(),
                fn_name: "transfer",
                args: (alice, bob, &transfer_amount).into_val(env),
                sub_invokes: &[],
            },
        }]);
        let result = client.try_transfer(alice, bob, &transfer_amount);

        if result.is_ok() {
            return TestResult {
                test_id: <Self as Scenario<F>>::id(self),
                description: <Self as Scenario<F>>::description(self),
                status: Status::Fail,
                expected_behavior: "Transfer fails when balance is insufficient",
                observed_behavior: "Transfer succeeded unexpectedly",
            };
        }

        let final_alice = client.balance(alice);
        let final_bob = client.balance(bob);

        if final_alice != initial_alice || final_bob != initial_bob {
            return TestResult {
                test_id: <Self as Scenario<F>>::id(self),
                description: <Self as Scenario<F>>::description(self),
                status: Status::Fail,
                expected_behavior: "Balances remain unchanged after failure",
                observed_behavior: "Balances were mutated despite failed transfer",
            };
        }

        TestResult {
            test_id: <Self as Scenario<F>>::id(self),
            description: <Self as Scenario<F>>::description(self),
            status: Status::Pass,
            expected_behavior: "Transfer fails and balances are preserved",
            observed_behavior: "Transfer failed as expected without mutating state",
        }
    }
}

pub struct TransferAuthorizationScenario;
impl<F: Sep41Fixture> Scenario<F> for TransferAuthorizationScenario {
    fn id(&self) -> &'static str {
        "SEP41-TRANSFER-005"
    }
    fn description(&self) -> &'static str {
        "Transfer authorization"
    }
    fn run(&self, env: &Env, fixture: &F) -> TestResult {
        let client = TokenClient::new(env, fixture.token_contract_id());
        let alice = fixture.test_account_1();
        let bob = fixture.test_account_2();
        let transfer_amount = 10_i128;

        let initial_alice = client.balance(alice);
        let initial_bob = client.balance(bob);

        let result = client.try_transfer(alice, bob, &transfer_amount);
        if result.is_ok() {
            return TestResult {
                test_id: <Self as Scenario<F>>::id(self),
                description: <Self as Scenario<F>>::description(self),
                status: Status::Fail,
                expected_behavior: "Transfer requires Soroban authorization from sender",
                observed_behavior: "Transfer succeeded without authorization",
            };
        }

        let final_alice = client.balance(alice);
        let final_bob = client.balance(bob);

        if final_alice != initial_alice || final_bob != initial_bob {
            return TestResult {
                test_id: <Self as Scenario<F>>::id(self),
                description: <Self as Scenario<F>>::description(self),
                status: Status::Fail,
                expected_behavior: "Balances unchanged on failed auth",
                observed_behavior: "Balances mutated without authorization",
            };
        }

        TestResult {
            test_id: <Self as Scenario<F>>::id(self),
            description: <Self as Scenario<F>>::description(self),
            status: Status::Pass,
            expected_behavior: "Transfer requires authorization",
            observed_behavior: "Transfer failed when unauthorized",
        }
    }
}

pub struct TransferNegativeAmountScenario;
impl<F: Sep41Fixture> Scenario<F> for TransferNegativeAmountScenario {
    fn id(&self) -> &'static str {
        "SEP41-TRANSFER-007"
    }
    fn description(&self) -> &'static str {
        "Negative amount transfer"
    }
    fn run(&self, env: &Env, fixture: &F) -> TestResult {
        let client = TokenClient::new(env, fixture.token_contract_id());
        let alice = fixture.test_account_1();
        let bob = fixture.test_account_2();
        let initial_alice = client.balance(alice);
        let transfer_amount = -10_i128;

        env.mock_auths(&[soroban_sdk::testutils::MockAuth {
            address: alice,
            invoke: &soroban_sdk::testutils::MockAuthInvoke {
                contract: fixture.token_contract_id(),
                fn_name: "transfer",
                args: (alice, bob, &transfer_amount).into_val(env),
                sub_invokes: &[],
            },
        }]);
        let result = client.try_transfer(alice, bob, &transfer_amount);

        if result.is_ok() {
            return TestResult {
                test_id: <Self as Scenario<F>>::id(self),
                description: <Self as Scenario<F>>::description(self),
                status: Status::Fail,
                expected_behavior: "Transfer fails with negative amount",
                observed_behavior: "Transfer succeeded with negative amount",
            };
        }

        let final_alice = client.balance(alice);
        if initial_alice != final_alice {
            return TestResult {
                test_id: <Self as Scenario<F>>::id(self),
                description: <Self as Scenario<F>>::description(self),
                status: Status::Fail,
                expected_behavior: "Balances remain unchanged",
                observed_behavior: "Balances mutated on negative transfer",
            };
        }

        TestResult {
            test_id: <Self as Scenario<F>>::id(self),
            description: <Self as Scenario<F>>::description(self),
            status: Status::Pass,
            expected_behavior: "Transfer fails and state is preserved",
            observed_behavior: "Transfer failed as expected",
        }
    }
}

// -----------------------------------------------------------------------------
// ALLOWANCE CONFORMANCE
// -----------------------------------------------------------------------------

pub struct AllowanceQueryScenario;
impl<F: Sep41Fixture> Scenario<F> for AllowanceQueryScenario {
    fn id(&self) -> &'static str {
        "SEP41-ALLOWANCE-001"
    }
    fn description(&self) -> &'static str {
        "Allowance query"
    }
    fn run(&self, env: &Env, fixture: &F) -> TestResult {
        let client = TokenClient::new(env, fixture.token_contract_id());
        let alice = fixture.test_account_1();
        let carol = fixture.test_account_3();

        let initial_allowance = client.allowance(alice, carol);
        if initial_allowance != 0 {
            return TestResult {
                test_id: <Self as Scenario<F>>::id(self),
                description: <Self as Scenario<F>>::description(self),
                status: Status::Fail,
                expected_behavior: "Initial allowance is zero",
                observed_behavior: "Initial allowance is non-zero",
            };
        }

        let amount = 100_i128;
        let expiration = env.ledger().sequence() + 100;
        env.mock_auths(&[soroban_sdk::testutils::MockAuth {
            address: alice,
            invoke: &soroban_sdk::testutils::MockAuthInvoke {
                contract: fixture.token_contract_id(),
                fn_name: "approve",
                args: (alice.clone(), carol.clone(), amount, expiration).into_val(env),
                sub_invokes: &[],
            },
        }]);
        client.approve(alice, carol, &amount, &expiration);

        let final_allowance = client.allowance(alice, carol);
        if final_allowance == amount {
            TestResult {
                test_id: <Self as Scenario<F>>::id(self),
                description: <Self as Scenario<F>>::description(self),
                status: Status::Pass,
                expected_behavior: "Allowance reflects approved amount",
                observed_behavior: "Allowance matched approved amount",
            }
        } else {
            TestResult {
                test_id: <Self as Scenario<F>>::id(self),
                description: <Self as Scenario<F>>::description(self),
                status: Status::Fail,
                expected_behavior: "Allowance reflects approved amount",
                observed_behavior: "Allowance did not match approved amount",
            }
        }
    }
}

pub struct AllowanceApproveScenario;
impl<F: Sep41Fixture> Scenario<F> for AllowanceApproveScenario {
    fn id(&self) -> &'static str {
        "SEP41-ALLOWANCE-002"
    }
    fn description(&self) -> &'static str {
        "Approve behavior"
    }
    fn run(&self, env: &Env, fixture: &F) -> TestResult {
        let client = TokenClient::new(env, fixture.token_contract_id());
        let alice = fixture.test_account_1();
        let carol = fixture.test_account_3();
        let amount = 150_i128;
        let expiration = env.ledger().sequence() + 100;

        env.mock_auths(&[soroban_sdk::testutils::MockAuth {
            address: alice,
            invoke: &soroban_sdk::testutils::MockAuthInvoke {
                contract: fixture.token_contract_id(),
                fn_name: "approve",
                args: (alice.clone(), carol.clone(), amount, expiration).into_val(env),
                sub_invokes: &[],
            },
        }]);
        let result = client.try_approve(alice, carol, &amount, &expiration);

        if result.is_err() {
            return TestResult {
                test_id: <Self as Scenario<F>>::id(self),
                description: <Self as Scenario<F>>::description(self),
                status: Status::Fail,
                expected_behavior: "Approve executes successfully",
                observed_behavior: "Approve failed unexpectedly",
            };
        }

        let allowance = client.allowance(alice, carol);
        if allowance != amount {
            return TestResult {
                test_id: <Self as Scenario<F>>::id(self),
                description: <Self as Scenario<F>>::description(self),
                status: Status::Fail,
                expected_behavior: "allowance(owner, spender) equals approved amount",
                observed_behavior: "Allowance did not accurately match approval",
            };
        }

        TestResult {
            test_id: <Self as Scenario<F>>::id(self),
            description: <Self as Scenario<F>>::description(self),
            status: Status::Pass,
            expected_behavior: "Approve successfully commits exact allowance to state",
            observed_behavior: "Approve verified successfully",
        }
    }
}

pub struct AllowanceTransferFromScenario;
impl<F: Sep41Fixture> Scenario<F> for AllowanceTransferFromScenario {
    fn id(&self) -> &'static str {
        "SEP41-ALLOWANCE-003"
    }
    fn description(&self) -> &'static str {
        "transfer_from behavior and reduction"
    }
    fn run(&self, env: &Env, fixture: &F) -> TestResult {
        let client = TokenClient::new(env, fixture.token_contract_id());
        let alice = fixture.test_account_1();
        let bob = fixture.test_account_2();
        let carol = fixture.test_account_3();

        let initial_alice = client.balance(alice);
        let initial_bob = client.balance(bob);
        let amount = 100_i128;
        let transfer_amount = 30_i128;
        let expiration = env.ledger().sequence() + 100;

        env.mock_auths(&[soroban_sdk::testutils::MockAuth {
            address: alice,
            invoke: &soroban_sdk::testutils::MockAuthInvoke {
                contract: fixture.token_contract_id(),
                fn_name: "approve",
                args: (alice, carol, &amount, &expiration).into_val(env),
                sub_invokes: &[],
            },
        }]);
        client.approve(alice, carol, &amount, &expiration);
        env.mock_auths(&[soroban_sdk::testutils::MockAuth {
            address: carol,
            invoke: &soroban_sdk::testutils::MockAuthInvoke {
                contract: fixture.token_contract_id(),
                fn_name: "transfer_from",
                args: (carol, alice, bob, &transfer_amount).into_val(env),
                sub_invokes: &[],
            },
        }]);
        client.transfer_from(carol, alice, bob, &transfer_amount);

        let final_alice = client.balance(alice);
        let final_bob = client.balance(bob);
        let final_allowance = client.allowance(alice, carol);

        // Verify expiration preservation: still valid at exactly 'expiration'
        env.ledger().with_mut(|li| li.sequence_number = expiration);
        let at_exp_allowance = client.allowance(alice, carol);

        // Verify it expires properly past 'expiration'
        env.ledger()
            .with_mut(|li| li.sequence_number = expiration + 1);
        let expired_allowance = client.allowance(alice, carol);

        if final_alice == initial_alice - transfer_amount
            && final_bob == initial_bob + transfer_amount
            && final_allowance == amount - transfer_amount
            && at_exp_allowance == amount - transfer_amount
            && expired_allowance == 0
        {
            TestResult { test_id: <Self as Scenario<F>>::id(self), description: <Self as Scenario<F>>::description(self), status: Status::Pass, expected_behavior: "transfer_from moves tokens exactly, reduces allowance, and preserves expiration", observed_behavior: "State transitions correctly processed" }
        } else {
            TestResult { test_id: <Self as Scenario<F>>::id(self), description: <Self as Scenario<F>>::description(self), status: Status::Fail, expected_behavior: "transfer_from moves tokens exactly, reduces allowance, and preserves expiration", observed_behavior: "State mutations failed to match specification requirements" }
        }
    }
}

pub struct AllowanceExpirationScenario;
impl<F: Sep41Fixture> Scenario<F> for AllowanceExpirationScenario {
    fn id(&self) -> &'static str {
        "SEP41-ALLOWANCE-004"
    }
    fn description(&self) -> &'static str {
        "Expiration bounds"
    }
    fn run(&self, env: &Env, fixture: &F) -> TestResult {
        let client = TokenClient::new(env, fixture.token_contract_id());
        let alice = fixture.test_account_1();
        let carol = fixture.test_account_3();
        let amount = 100_i128;
        let expiration = env.ledger().sequence() + 10;

        env.mock_auths(&[soroban_sdk::testutils::MockAuth {
            address: alice,
            invoke: &soroban_sdk::testutils::MockAuthInvoke {
                contract: fixture.token_contract_id(),
                fn_name: "approve",
                args: (alice, carol, &amount, &expiration).into_val(env),
                sub_invokes: &[],
            },
        }]);
        client.approve(alice, carol, &amount, &expiration);

        env.ledger().with_mut(|li| li.sequence_number = expiration);
        let exact_allowance = client.allowance(alice, carol);

        env.ledger()
            .with_mut(|li| li.sequence_number = expiration + 1);
        let expired_allowance = client.allowance(alice, carol);

        if exact_allowance == amount && expired_allowance == 0 {
            TestResult {
                test_id: <Self as Scenario<F>>::id(self),
                description: <Self as Scenario<F>>::description(self),
                status: Status::Pass,
                expected_behavior: "Allowance is zero when current > live_until_ledger",
                observed_behavior: "Expiration semantics correctly adhered to",
            }
        } else {
            TestResult {
                test_id: <Self as Scenario<F>>::id(self),
                description: <Self as Scenario<F>>::description(self),
                status: Status::Fail,
                expected_behavior: "Allowance is zero when current > live_until_ledger",
                observed_behavior: "Expiration semantics violated",
            }
        }
    }
}

pub struct AllowanceInsufficientScenario;
impl<F: Sep41Fixture> Scenario<F> for AllowanceInsufficientScenario {
    fn id(&self) -> &'static str {
        "SEP41-ALLOWANCE-005"
    }
    fn description(&self) -> &'static str {
        "Insufficient allowance"
    }
    fn run(&self, env: &Env, fixture: &F) -> TestResult {
        let client = TokenClient::new(env, fixture.token_contract_id());
        let alice = fixture.test_account_1();
        let bob = fixture.test_account_2();
        let carol = fixture.test_account_3();

        let initial_alice = client.balance(alice);
        let initial_bob = client.balance(bob);
        let initial_allowance = 50_i128;
        let expiration = env.ledger().sequence() + 100;

        env.mock_auths(&[soroban_sdk::testutils::MockAuth {
            address: alice,
            invoke: &soroban_sdk::testutils::MockAuthInvoke {
                contract: fixture.token_contract_id(),
                fn_name: "approve",
                args: (alice, carol, &initial_allowance, &expiration).into_val(env),
                sub_invokes: &[],
            },
        }]);
        client.approve(alice, carol, &initial_allowance, &expiration);

        let request_amount = initial_allowance + 1;
        env.mock_auths(&[soroban_sdk::testutils::MockAuth {
            address: carol,
            invoke: &soroban_sdk::testutils::MockAuthInvoke {
                contract: fixture.token_contract_id(),
                fn_name: "transfer_from",
                args: (carol, alice, bob, &request_amount).into_val(env),
                sub_invokes: &[],
            },
        }]);
        let result = client.try_transfer_from(carol, alice, bob, &request_amount);

        let final_alice = client.balance(alice);
        let final_bob = client.balance(bob);
        let final_allowance = client.allowance(alice, carol);

        if result.is_ok() {
            return TestResult {
                test_id: <Self as Scenario<F>>::id(self),
                description: <Self as Scenario<F>>::description(self),
                status: Status::Fail,
                expected_behavior: "transfer_from fails on insufficient allowance",
                observed_behavior: "transfer_from succeeded unexpectedly",
            };
        }

        if final_alice != initial_alice
            || final_bob != initial_bob
            || final_allowance != initial_allowance
        {
            return TestResult {
                test_id: <Self as Scenario<F>>::id(self),
                description: <Self as Scenario<F>>::description(self),
                status: Status::Fail,
                expected_behavior: "State unchanged on failure",
                observed_behavior: "State mutated despite failed transfer_from",
            };
        }

        TestResult {
            test_id: <Self as Scenario<F>>::id(self),
            description: <Self as Scenario<F>>::description(self),
            status: Status::Pass,
            expected_behavior: "Transfer fails cleanly without mutating state",
            observed_behavior: "Transfer rejected correctly",
        }
    }
}

pub struct AllowanceUnauthorizedApproveScenario;
impl<F: Sep41Fixture> Scenario<F> for AllowanceUnauthorizedApproveScenario {
    fn id(&self) -> &'static str {
        "SEP41-ALLOWANCE-006"
    }
    fn description(&self) -> &'static str {
        "Unauthorized approve"
    }
    fn run(&self, env: &Env, fixture: &F) -> TestResult {
        let client = TokenClient::new(env, fixture.token_contract_id());
        let alice = fixture.test_account_1();
        let carol = fixture.test_account_3();
        let amount = 100_i128;
        let expiration = env.ledger().sequence() + 100;

        // Setup initial allowance
        env.mock_auths(&[soroban_sdk::testutils::MockAuth {
            address: alice,
            invoke: &soroban_sdk::testutils::MockAuthInvoke {
                contract: fixture.token_contract_id(),
                fn_name: "approve",
                args: (alice, carol, &amount, &expiration).into_val(env),
                sub_invokes: &[],
            },
        }]);
        client.approve(alice, carol, &amount, &expiration);

        // Clear mock auths
        env.mock_auths(&[]);

        let new_amount = 200_i128;
        let result = client.try_approve(alice, carol, &new_amount, &expiration);

        if result.is_ok() {
            return TestResult {
                test_id: <Self as Scenario<F>>::id(self),
                description: <Self as Scenario<F>>::description(self),
                status: Status::Fail,
                expected_behavior: "Approve requires from authorization",
                observed_behavior: "Approve succeeded without authorization",
            };
        }

        let final_allowance = client.allowance(alice, carol);
        if final_allowance != amount {
            return TestResult {
                test_id: <Self as Scenario<F>>::id(self),
                description: <Self as Scenario<F>>::description(self),
                status: Status::Fail,
                expected_behavior: "Allowance unchanged after failed auth",
                observed_behavior: "Allowance mutated without authorization",
            };
        }

        TestResult {
            test_id: <Self as Scenario<F>>::id(self),
            description: <Self as Scenario<F>>::description(self),
            status: Status::Pass,
            expected_behavior: "Unauthorized approve fails cleanly",
            observed_behavior: "Failed cleanly without state mutation",
        }
    }
}

pub struct AllowanceUnauthorizedTransferFromScenario;
impl<F: Sep41Fixture> Scenario<F> for AllowanceUnauthorizedTransferFromScenario {
    fn id(&self) -> &'static str {
        "SEP41-ALLOWANCE-007"
    }
    fn description(&self) -> &'static str {
        "Unauthorized transfer_from"
    }
    fn run(&self, env: &Env, fixture: &F) -> TestResult {
        let client = TokenClient::new(env, fixture.token_contract_id());
        let alice = fixture.test_account_1();
        let bob = fixture.test_account_2();
        let carol = fixture.test_account_3();
        let amount = 50_i128;
        let expiration = env.ledger().sequence() + 100;

        // Setup initial allowance
        env.mock_auths(&[soroban_sdk::testutils::MockAuth {
            address: alice,
            invoke: &soroban_sdk::testutils::MockAuthInvoke {
                contract: fixture.token_contract_id(),
                fn_name: "approve",
                args: (alice, carol, &amount, &expiration).into_val(env),
                sub_invokes: &[],
            },
        }]);
        client.approve(alice, carol, &amount, &expiration);

        // Clear mock auths
        env.mock_auths(&[]);

        let initial_alice = client.balance(alice);
        let initial_bob = client.balance(bob);
        let initial_allowance = client.allowance(alice, carol);

        let result = client.try_transfer_from(carol, alice, bob, &amount);

        if result.is_ok() {
            return TestResult {
                test_id: <Self as Scenario<F>>::id(self),
                description: <Self as Scenario<F>>::description(self),
                status: Status::Fail,
                expected_behavior: "transfer_from requires spender authorization",
                observed_behavior: "transfer_from succeeded without authorization",
            };
        }

        let final_alice = client.balance(alice);
        let final_bob = client.balance(bob);
        let final_allowance = client.allowance(alice, carol);

        if final_alice != initial_alice
            || final_bob != initial_bob
            || final_allowance != initial_allowance
        {
            return TestResult {
                test_id: <Self as Scenario<F>>::id(self),
                description: <Self as Scenario<F>>::description(self),
                status: Status::Fail,
                expected_behavior: "State unchanged on failed auth",
                observed_behavior: "State mutated without authorization",
            };
        }

        TestResult {
            test_id: <Self as Scenario<F>>::id(self),
            description: <Self as Scenario<F>>::description(self),
            status: Status::Pass,
            expected_behavior: "Unauthorized transfer_from fails cleanly",
            observed_behavior: "Failed cleanly without authorization",
        }
    }
}

pub struct AllowanceEventScenario;
impl<F: Sep41Fixture> Scenario<F> for AllowanceEventScenario {
    fn id(&self) -> &'static str {
        "SEP41-ALLOWANCE-008"
    }
    fn description(&self) -> &'static str {
        "Approve event emission"
    }
    fn run(&self, env: &Env, fixture: &F) -> TestResult {
        let client = TokenClient::new(env, fixture.token_contract_id());
        let alice = fixture.test_account_1();
        let carol = fixture.test_account_3();
        let amount = 10_i128;
        let expiration = env.ledger().sequence() + 100;

        env.mock_auths(&[soroban_sdk::testutils::MockAuth {
            address: alice,
            invoke: &soroban_sdk::testutils::MockAuthInvoke {
                contract: fixture.token_contract_id(),
                fn_name: "approve",
                args: (alice, carol, &amount, &expiration).into_val(env),
                sub_invokes: &[],
            },
        }]);
        client.approve(alice, carol, &amount, &expiration);

        let events = env.events().all();
        let mut found_topic = false;
        let approve_symbol = soroban_sdk::Symbol::new(env, "approve");

        let filtered = events.filter_by_contract(fixture.token_contract_id());
        for event in filtered.events() {
            let soroban_sdk::xdr::ContractEventBody::V0(body) = &event.body;
            let Ok(topics) = soroban_sdk::Vec::<soroban_sdk::Val>::try_from_val(env, &body.topics)
            else {
                continue;
            };
            let Ok(data) = soroban_sdk::Val::try_from_val(env, &body.data) else {
                continue;
            };
            let contract_id = fixture.token_contract_id().clone();
            if contract_id == *fixture.token_contract_id() && topics.len() >= 3 {
                let mut iter = topics.into_iter();
                let t0 = iter.next();
                let t1 = iter.next();
                let t2 = iter.next();
                if let (Some(t0), Some(t1), Some(t2)) = (t0, t1, t2) {
                    if let (Ok(sym), Ok(from), Ok(spender)) = (
                        soroban_sdk::Symbol::try_from_val(env, &t0),
                        soroban_sdk::Address::try_from_val(env, &t1),
                        soroban_sdk::Address::try_from_val(env, &t2),
                    ) {
                        if sym == approve_symbol
                            && &from == alice
                            && &spender == carol
                            && crate::event_helpers::verify_approve_data(
                                env, &data, amount, expiration,
                            )
                        {
                            found_topic = true;
                            break;
                        }
                    }
                }
            }
        }

        if found_topic {
            TestResult {
                test_id: <Self as Scenario<F>>::id(self),
                description: <Self as Scenario<F>>::description(self),
                status: Status::Pass,
                expected_behavior: "Emits approve event with required topics and data",
                observed_behavior:
                    "Approve event found with required semantic topics and exact data",
            }
        } else {
            TestResult {
                test_id: <Self as Scenario<F>>::id(self),
                description: <Self as Scenario<F>>::description(self),
                status: Status::Fail,
                expected_behavior: "Emits approve event with required topics and data",
                observed_behavior: "Approve event missing or lacked required semantic data",
            }
        }
    }
}

pub struct AllowanceZeroRevocationScenario;
impl<F: Sep41Fixture> Scenario<F> for AllowanceZeroRevocationScenario {
    fn id(&self) -> &'static str {
        "SEP41-ALLOWANCE-009"
    }
    fn description(&self) -> &'static str {
        "Zero-amount revocation"
    }
    fn run(&self, env: &Env, fixture: &F) -> TestResult {
        let client = TokenClient::new(env, fixture.token_contract_id());
        let alice = fixture.test_account_1();
        let carol = fixture.test_account_3();
        let amount = 100_i128;
        let expiration = env.ledger().sequence() + 100;

        env.mock_auths(&[soroban_sdk::testutils::MockAuth {
            address: alice,
            invoke: &soroban_sdk::testutils::MockAuthInvoke {
                contract: fixture.token_contract_id(),
                fn_name: "approve",
                args: (alice, carol, &amount, &expiration).into_val(env),
                sub_invokes: &[],
            },
        }]);
        client.approve(alice, carol, &amount, &expiration);
        let initial_allowance = client.allowance(alice, carol);

        let zero_amount = 0_i128;
        env.mock_auths(&[soroban_sdk::testutils::MockAuth {
            address: alice,
            invoke: &soroban_sdk::testutils::MockAuthInvoke {
                contract: fixture.token_contract_id(),
                fn_name: "approve",
                args: (alice, carol, &zero_amount, &expiration).into_val(env),
                sub_invokes: &[],
            },
        }]);
        client.approve(alice, carol, &zero_amount, &expiration);
        let revoked_allowance = client.allowance(alice, carol);

        if initial_allowance == amount && revoked_allowance == 0 {
            TestResult {
                test_id: <Self as Scenario<F>>::id(self),
                description: <Self as Scenario<F>>::description(self),
                status: Status::Pass,
                expected_behavior: "approve with amount=0 revokes allowance completely",
                observed_behavior: "Allowance successfully revoked to 0",
            }
        } else {
            TestResult {
                test_id: <Self as Scenario<F>>::id(self),
                description: <Self as Scenario<F>>::description(self),
                status: Status::Fail,
                expected_behavior: "approve with amount=0 revokes allowance completely",
                observed_behavior: "Allowance was not revoked accurately",
            }
        }
    }
}

pub struct AllowanceOverwriteScenario;
impl<F: Sep41Fixture> Scenario<F> for AllowanceOverwriteScenario {
    fn id(&self) -> &'static str {
        "SEP41-ALLOWANCE-010"
    }
    fn description(&self) -> &'static str {
        "Approve overwrite behavior"
    }
    fn run(&self, env: &Env, fixture: &F) -> TestResult {
        let client = TokenClient::new(env, fixture.token_contract_id());
        let alice = fixture.test_account_1();
        let carol = fixture.test_account_3();
        let amount1 = 100_i128;
        let exp1 = env.ledger().sequence() + 50;
        let amount2 = 40_i128;
        let exp2 = env.ledger().sequence() + 100;

        env.mock_auths(&[soroban_sdk::testutils::MockAuth {
            address: alice,
            invoke: &soroban_sdk::testutils::MockAuthInvoke {
                contract: fixture.token_contract_id(),
                fn_name: "approve",
                args: (alice, carol, &amount1, &exp1).into_val(env),
                sub_invokes: &[],
            },
        }]);
        client.approve(alice, carol, &amount1, &exp1);
        env.mock_auths(&[soroban_sdk::testutils::MockAuth {
            address: alice,
            invoke: &soroban_sdk::testutils::MockAuthInvoke {
                contract: fixture.token_contract_id(),
                fn_name: "approve",
                args: (alice, carol, &amount2, &exp2).into_val(env),
                sub_invokes: &[],
            },
        }]);
        client.approve(alice, carol, &amount2, &exp2);

        let final_allowance = client.allowance(alice, carol);

        env.ledger().with_mut(|li| li.sequence_number = exp2);
        let at_exp2_allowance = client.allowance(alice, carol);

        if final_allowance == amount2 && at_exp2_allowance == amount2 {
            TestResult {
                test_id: <Self as Scenario<F>>::id(self),
                description: <Self as Scenario<F>>::description(self),
                status: Status::Pass,
                expected_behavior: "Subsequent approve fully overwrites amount and expiration",
                observed_behavior: "Allowance successfully overwritten",
            }
        } else {
            TestResult {
                test_id: <Self as Scenario<F>>::id(self),
                description: <Self as Scenario<F>>::description(self),
                status: Status::Fail,
                expected_behavior: "Subsequent approve fully overwrites amount and expiration",
                observed_behavior: "Allowance overwrite failed",
            }
        }
    }
}

// -----------------------------------------------------------------------------
// BURN CONFORMANCE
// -----------------------------------------------------------------------------

pub struct BurnSuccessScenario;
impl<F: Sep41Fixture> Scenario<F> for BurnSuccessScenario {
    fn id(&self) -> &'static str {
        "SEP41-BURN-001"
    }
    fn description(&self) -> &'static str {
        "Direct burn"
    }
    fn run(&self, env: &Env, fixture: &F) -> TestResult {
        let client = TokenClient::new(env, fixture.token_contract_id());
        let alice = fixture.test_account_1();
        let initial_alice = client.balance(alice);
        let burn_amount = 50_i128;

        if initial_alice < burn_amount {
            return TestResult {
                test_id: <Self as Scenario<F>>::id(self),
                description: <Self as Scenario<F>>::description(self),
                status: Status::Error,
                expected_behavior: "Alice has enough balance",
                observed_behavior: "Insufficient balance for setup",
            };
        }

        env.mock_auths(&[soroban_sdk::testutils::MockAuth {
            address: alice,
            invoke: &soroban_sdk::testutils::MockAuthInvoke {
                contract: fixture.token_contract_id(),
                fn_name: "burn",
                args: (alice, &burn_amount).into_val(env),
                sub_invokes: &[],
            },
        }]);
        client.burn(alice, &burn_amount);

        let final_alice = client.balance(alice);

        if final_alice == initial_alice - burn_amount {
            TestResult {
                test_id: <Self as Scenario<F>>::id(self),
                description: <Self as Scenario<F>>::description(self),
                status: Status::Pass,
                expected_behavior: "Balance decreases by burn amount",
                observed_behavior: "Balance decreased correctly",
            }
        } else {
            TestResult {
                test_id: <Self as Scenario<F>>::id(self),
                description: <Self as Scenario<F>>::description(self),
                status: Status::Fail,
                expected_behavior: "Balance decreases by burn amount",
                observed_behavior: "Balance did not decrease correctly",
            }
        }
    }
}

pub struct BurnAuthorizationScenario;
impl<F: Sep41Fixture> Scenario<F> for BurnAuthorizationScenario {
    fn id(&self) -> &'static str {
        "SEP41-BURN-002"
    }
    fn description(&self) -> &'static str {
        "Burn authorization"
    }
    fn run(&self, env: &Env, fixture: &F) -> TestResult {
        let client = TokenClient::new(env, fixture.token_contract_id());
        let alice = fixture.test_account_1();
        let initial_alice = client.balance(alice);
        let burn_amount = 10_i128;

        let result = client.try_burn(alice, &burn_amount);

        if result.is_ok() {
            return TestResult {
                test_id: <Self as Scenario<F>>::id(self),
                description: <Self as Scenario<F>>::description(self),
                status: Status::Fail,
                expected_behavior: "Burn requires from authorization",
                observed_behavior: "Burn succeeded without authorization",
            };
        }

        let final_alice = client.balance(alice);
        if final_alice != initial_alice {
            return TestResult {
                test_id: <Self as Scenario<F>>::id(self),
                description: <Self as Scenario<F>>::description(self),
                status: Status::Fail,
                expected_behavior: "State remains unchanged after failed auth",
                observed_behavior: "Balance mutated despite failed auth",
            };
        }

        TestResult {
            test_id: <Self as Scenario<F>>::id(self),
            description: <Self as Scenario<F>>::description(self),
            status: Status::Pass,
            expected_behavior: "Burn fails cleanly without authorization",
            observed_behavior: "Failed cleanly",
        }
    }
}

pub struct BurnFromSuccessScenario;
impl<F: Sep41Fixture> Scenario<F> for BurnFromSuccessScenario {
    fn id(&self) -> &'static str {
        "SEP41-BURN-003"
    }
    fn description(&self) -> &'static str {
        "burn_from behavior"
    }
    fn run(&self, env: &Env, fixture: &F) -> TestResult {
        let client = TokenClient::new(env, fixture.token_contract_id());
        let alice = fixture.test_account_1();
        let carol = fixture.test_account_3();
        let initial_alice = client.balance(alice);
        let amount = 100_i128;
        let burn_amount = 30_i128;
        let expiration = env.ledger().sequence() + 100;

        env.mock_auths(&[soroban_sdk::testutils::MockAuth {
            address: alice,
            invoke: &soroban_sdk::testutils::MockAuthInvoke {
                contract: fixture.token_contract_id(),
                fn_name: "approve",
                args: (alice, carol, &amount, &expiration).into_val(env),
                sub_invokes: &[],
            },
        }]);
        client.approve(alice, carol, &amount, &expiration);
        env.mock_auths(&[soroban_sdk::testutils::MockAuth {
            address: carol,
            invoke: &soroban_sdk::testutils::MockAuthInvoke {
                contract: fixture.token_contract_id(),
                fn_name: "burn_from",
                args: (carol, alice, &burn_amount).into_val(env),
                sub_invokes: &[],
            },
        }]);
        client.burn_from(carol, alice, &burn_amount);

        let final_alice = client.balance(alice);
        let final_allowance = client.allowance(alice, carol);

        env.ledger().with_mut(|li| li.sequence_number = expiration);
        let at_exp_allowance = client.allowance(alice, carol);

        env.ledger()
            .with_mut(|li| li.sequence_number = expiration + 1);
        let expired_allowance = client.allowance(alice, carol);

        if final_alice == initial_alice - burn_amount
            && final_allowance == amount - burn_amount
            && at_exp_allowance == amount - burn_amount
            && expired_allowance == 0
        {
            TestResult {
                test_id: <Self as Scenario<F>>::id(self),
                description: <Self as Scenario<F>>::description(self),
                status: Status::Pass,
                expected_behavior:
                    "burn_from decreases balance and allowance, preserves expiration",
                observed_behavior: "Delegated burn executed accurately",
            }
        } else {
            TestResult {
                test_id: <Self as Scenario<F>>::id(self),
                description: <Self as Scenario<F>>::description(self),
                status: Status::Fail,
                expected_behavior:
                    "burn_from decreases balance and allowance, preserves expiration",
                observed_behavior: "State mutations incorrect",
            }
        }
    }
}

pub struct BurnFromAuthorizationScenario;
impl<F: Sep41Fixture> Scenario<F> for BurnFromAuthorizationScenario {
    fn id(&self) -> &'static str {
        "SEP41-BURN-004"
    }
    fn description(&self) -> &'static str {
        "burn_from authorization"
    }
    fn run(&self, env: &Env, fixture: &F) -> TestResult {
        let client = TokenClient::new(env, fixture.token_contract_id());
        let alice = fixture.test_account_1();
        let carol = fixture.test_account_3();
        let burn_amount = 30_i128;
        let amount = 100_i128;
        let expiration = env.ledger().sequence() + 100;

        env.mock_auths(&[soroban_sdk::testutils::MockAuth {
            address: alice,
            invoke: &soroban_sdk::testutils::MockAuthInvoke {
                contract: fixture.token_contract_id(),
                fn_name: "approve",
                args: (alice, carol, &amount, &expiration).into_val(env),
                sub_invokes: &[],
            },
        }]);
        client.approve(alice, carol, &amount, &expiration);

        env.mock_auths(&[]);

        let initial_alice = client.balance(alice);
        let initial_allowance = client.allowance(alice, carol);

        let result = client.try_burn_from(carol, alice, &burn_amount);

        if result.is_ok() {
            return TestResult {
                test_id: <Self as Scenario<F>>::id(self),
                description: <Self as Scenario<F>>::description(self),
                status: Status::Fail,
                expected_behavior: "burn_from requires spender auth",
                observed_behavior: "burn_from succeeded without auth",
            };
        }

        let final_alice = client.balance(alice);
        let final_allowance = client.allowance(alice, carol);

        if final_alice != initial_alice || final_allowance != initial_allowance {
            return TestResult {
                test_id: <Self as Scenario<F>>::id(self),
                description: <Self as Scenario<F>>::description(self),
                status: Status::Fail,
                expected_behavior: "State unchanged on failed auth",
                observed_behavior: "State mutated without authorization",
            };
        }

        TestResult {
            test_id: <Self as Scenario<F>>::id(self),
            description: <Self as Scenario<F>>::description(self),
            status: Status::Pass,
            expected_behavior: "Unauthorized burn_from fails cleanly",
            observed_behavior: "Failed cleanly",
        }
    }
}

pub struct BurnInsufficientBalanceScenario;
impl<F: Sep41Fixture> Scenario<F> for BurnInsufficientBalanceScenario {
    fn id(&self) -> &'static str {
        "SEP41-BURN-005"
    }
    fn description(&self) -> &'static str {
        "Insufficient balance"
    }
    fn run(&self, env: &Env, fixture: &F) -> TestResult {
        let client = TokenClient::new(env, fixture.token_contract_id());
        let alice = fixture.test_account_1();
        let initial_alice = client.balance(alice);
        let burn_amount = initial_alice + 1;

        env.mock_auths(&[soroban_sdk::testutils::MockAuth {
            address: alice,
            invoke: &soroban_sdk::testutils::MockAuthInvoke {
                contract: fixture.token_contract_id(),
                fn_name: "burn",
                args: (alice, &burn_amount).into_val(env),
                sub_invokes: &[],
            },
        }]);
        let result = client.try_burn(alice, &burn_amount);

        if result.is_ok() {
            return TestResult {
                test_id: <Self as Scenario<F>>::id(self),
                description: <Self as Scenario<F>>::description(self),
                status: Status::Fail,
                expected_behavior: "Burn fails when balance is insufficient",
                observed_behavior: "Burn succeeded unexpectedly",
            };
        }

        let final_alice = client.balance(alice);
        if final_alice != initial_alice {
            return TestResult {
                test_id: <Self as Scenario<F>>::id(self),
                description: <Self as Scenario<F>>::description(self),
                status: Status::Fail,
                expected_behavior: "Balance unchanged after failure",
                observed_behavior: "Balance mutated despite failed burn",
            };
        }

        TestResult {
            test_id: <Self as Scenario<F>>::id(self),
            description: <Self as Scenario<F>>::description(self),
            status: Status::Pass,
            expected_behavior: "Burn fails cleanly on insufficient balance",
            observed_behavior: "Burn rejected correctly",
        }
    }
}

pub struct BurnFromInsufficientAllowanceScenario;
impl<F: Sep41Fixture> Scenario<F> for BurnFromInsufficientAllowanceScenario {
    fn id(&self) -> &'static str {
        "SEP41-BURN-006"
    }
    fn description(&self) -> &'static str {
        "Insufficient allowance"
    }
    fn run(&self, env: &Env, fixture: &F) -> TestResult {
        let client = TokenClient::new(env, fixture.token_contract_id());
        let alice = fixture.test_account_1();
        let carol = fixture.test_account_3();
        let initial_alice = client.balance(alice);
        let initial_allowance = 50_i128;
        let expiration = env.ledger().sequence() + 100;

        env.mock_auths(&[soroban_sdk::testutils::MockAuth {
            address: alice,
            invoke: &soroban_sdk::testutils::MockAuthInvoke {
                contract: fixture.token_contract_id(),
                fn_name: "approve",
                args: (alice, carol, &initial_allowance, &expiration).into_val(env),
                sub_invokes: &[],
            },
        }]);
        client.approve(alice, carol, &initial_allowance, &expiration);

        let burn_amount = initial_allowance + 1;
        env.mock_auths(&[soroban_sdk::testutils::MockAuth {
            address: carol,
            invoke: &soroban_sdk::testutils::MockAuthInvoke {
                contract: fixture.token_contract_id(),
                fn_name: "burn_from",
                args: (carol, alice, &burn_amount).into_val(env),
                sub_invokes: &[],
            },
        }]);
        let result = client.try_burn_from(carol, alice, &burn_amount);

        if result.is_ok() {
            return TestResult {
                test_id: <Self as Scenario<F>>::id(self),
                description: <Self as Scenario<F>>::description(self),
                status: Status::Fail,
                expected_behavior: "burn_from fails on insufficient allowance",
                observed_behavior: "burn_from succeeded unexpectedly",
            };
        }

        let final_alice = client.balance(alice);
        let final_allowance = client.allowance(alice, carol);

        if final_alice != initial_alice || final_allowance != initial_allowance {
            return TestResult {
                test_id: <Self as Scenario<F>>::id(self),
                description: <Self as Scenario<F>>::description(self),
                status: Status::Fail,
                expected_behavior: "State unchanged on failure",
                observed_behavior: "State mutated despite failed burn_from",
            };
        }

        TestResult {
            test_id: <Self as Scenario<F>>::id(self),
            description: <Self as Scenario<F>>::description(self),
            status: Status::Pass,
            expected_behavior: "burn_from fails cleanly without mutating state",
            observed_behavior: "Delegated burn rejected correctly",
        }
    }
}

pub struct BurnEventScenario;
impl<F: Sep41Fixture> Scenario<F> for BurnEventScenario {
    fn id(&self) -> &'static str {
        "SEP41-BURN-007"
    }
    fn description(&self) -> &'static str {
        "Burn event verification"
    }
    fn run(&self, env: &Env, fixture: &F) -> TestResult {
        let client = TokenClient::new(env, fixture.token_contract_id());
        let alice = fixture.test_account_1();
        let burn_amount = 10_i128;

        env.mock_auths(&[soroban_sdk::testutils::MockAuth {
            address: alice,
            invoke: &soroban_sdk::testutils::MockAuthInvoke {
                contract: fixture.token_contract_id(),
                fn_name: "burn",
                args: (alice, &burn_amount).into_val(env),
                sub_invokes: &[],
            },
        }]);
        client.burn(alice, &burn_amount);

        let events = env.events().all();
        let mut found_valid_event = false;
        let burn_symbol = soroban_sdk::Symbol::new(env, "burn");

        let filtered = events.filter_by_contract(fixture.token_contract_id());
        for event in filtered.events() {
            let soroban_sdk::xdr::ContractEventBody::V0(body) = &event.body;
            let Ok(topics) = soroban_sdk::Vec::<soroban_sdk::Val>::try_from_val(env, &body.topics)
            else {
                continue;
            };
            let Ok(data) = soroban_sdk::Val::try_from_val(env, &body.data) else {
                continue;
            };
            let contract_id = fixture.token_contract_id().clone();
            if contract_id == *fixture.token_contract_id() && topics.len() >= 2 {
                let mut iter = topics.into_iter();
                let t0 = iter.next();
                let t1 = iter.next();
                if let (Some(t0), Some(t1)) = (t0, t1) {
                    if let (Ok(sym), Ok(from)) = (
                        soroban_sdk::Symbol::try_from_val(env, &t0),
                        soroban_sdk::Address::try_from_val(env, &t1),
                    ) {
                        if sym == burn_symbol
                            && &from == alice
                            && crate::event_helpers::verify_amount_data(env, &data, burn_amount)
                        {
                            found_valid_event = true;
                            break;
                        }
                    }
                }
            }
        }

        if found_valid_event {
            TestResult {
                test_id: <Self as Scenario<F>>::id(self),
                description: <Self as Scenario<F>>::description(self),
                status: Status::Pass,
                expected_behavior: "Emits burn event with exact topics and valid data format",
                observed_behavior: "Burn event found matching exact SEP-41 semantic requirements",
            }
        } else {
            TestResult {
                test_id: <Self as Scenario<F>>::id(self),
                description: <Self as Scenario<F>>::description(self),
                status: Status::Fail,
                expected_behavior: "Emits burn event with exact topics and valid data format",
                observed_behavior:
                    "Burn event missing or lacked required data matching amount burned",
            }
        }
    }
}

pub struct AllowanceTransferFromEventScenario;
impl<F: Sep41Fixture> Scenario<F> for AllowanceTransferFromEventScenario {
    fn id(&self) -> &'static str {
        "SEP41-ALLOWANCE-011"
    }
    fn description(&self) -> &'static str {
        "transfer_from event emission"
    }
    fn run(&self, env: &Env, fixture: &F) -> TestResult {
        let client = TokenClient::new(env, fixture.token_contract_id());
        let alice = fixture.test_account_1();
        let bob = fixture.test_account_2();
        let carol = fixture.test_account_3();
        let transfer_amount = 10_i128;
        let expiration = env.ledger().sequence() + 100;

        env.mock_auths(&[soroban_sdk::testutils::MockAuth {
            address: alice,
            invoke: &soroban_sdk::testutils::MockAuthInvoke {
                contract: fixture.token_contract_id(),
                fn_name: "approve",
                args: (alice, carol, &transfer_amount, &expiration).into_val(env),
                sub_invokes: &[],
            },
        }]);
        client.approve(alice, carol, &transfer_amount, &expiration);

        env.mock_auths(&[soroban_sdk::testutils::MockAuth {
            address: carol,
            invoke: &soroban_sdk::testutils::MockAuthInvoke {
                contract: fixture.token_contract_id(),
                fn_name: "transfer_from",
                args: (carol, alice, bob, &transfer_amount).into_val(env),
                sub_invokes: &[],
            },
        }]);
        client.transfer_from(carol, alice, bob, &transfer_amount);

        let events = env.events().all();
        let mut found = false;
        let transfer_symbol = soroban_sdk::Symbol::new(env, "transfer");

        let filtered = events.filter_by_contract(fixture.token_contract_id());
        for event in filtered.events() {
            let soroban_sdk::xdr::ContractEventBody::V0(body) = &event.body;
            let Ok(topics) = soroban_sdk::Vec::<soroban_sdk::Val>::try_from_val(env, &body.topics)
            else {
                continue;
            };
            let Ok(data) = soroban_sdk::Val::try_from_val(env, &body.data) else {
                continue;
            };
            let contract_id = fixture.token_contract_id().clone();
            if contract_id == *fixture.token_contract_id() && topics.len() >= 3 {
                let mut iter = topics.into_iter();
                let t0 = iter.next();
                let t1 = iter.next();
                let t2 = iter.next();
                if let (Some(t0), Some(t1), Some(t2)) = (t0, t1, t2) {
                    if let (Ok(sym), Ok(from), Ok(to)) = (
                        soroban_sdk::Symbol::try_from_val(env, &t0),
                        soroban_sdk::Address::try_from_val(env, &t1),
                        soroban_sdk::Address::try_from_val(env, &t2),
                    ) {
                        if sym == transfer_symbol
                            && &from == alice
                            && &to == bob
                            && crate::event_helpers::verify_amount_data(env, &data, transfer_amount)
                        {
                            found = true;
                            break;
                        }
                    }
                }
            }
        }

        if found {
            TestResult {
                test_id: <Self as Scenario<F>>::id(self),
                description: <Self as Scenario<F>>::description(self),
                status: Status::Pass,
                expected_behavior: "Emits transfer event with correct topics and data",
                observed_behavior: "transfer_from event found matching specification",
            }
        } else {
            TestResult {
                test_id: <Self as Scenario<F>>::id(self),
                description: <Self as Scenario<F>>::description(self),
                status: Status::Fail,
                expected_behavior: "Emits transfer event with correct topics and data",
                observed_behavior: "Valid transfer_from event was not emitted",
            }
        }
    }
}

pub struct BurnFromEventScenario;
impl<F: Sep41Fixture> Scenario<F> for BurnFromEventScenario {
    fn id(&self) -> &'static str {
        "SEP41-BURN-008"
    }
    fn description(&self) -> &'static str {
        "burn_from event emission"
    }
    fn run(&self, env: &Env, fixture: &F) -> TestResult {
        let client = TokenClient::new(env, fixture.token_contract_id());
        let alice = fixture.test_account_1();
        let carol = fixture.test_account_3();
        let burn_amount = 5_i128;
        let expiration = env.ledger().sequence() + 100;

        env.mock_auths(&[soroban_sdk::testutils::MockAuth {
            address: alice,
            invoke: &soroban_sdk::testutils::MockAuthInvoke {
                contract: fixture.token_contract_id(),
                fn_name: "approve",
                args: (alice, carol, &burn_amount, &expiration).into_val(env),
                sub_invokes: &[],
            },
        }]);
        client.approve(alice, carol, &burn_amount, &expiration);

        env.mock_auths(&[soroban_sdk::testutils::MockAuth {
            address: carol,
            invoke: &soroban_sdk::testutils::MockAuthInvoke {
                contract: fixture.token_contract_id(),
                fn_name: "burn_from",
                args: (carol, alice, &burn_amount).into_val(env),
                sub_invokes: &[],
            },
        }]);
        client.burn_from(carol, alice, &burn_amount);

        let events = env.events().all();
        let mut found_valid_event = false;
        let burn_symbol = soroban_sdk::Symbol::new(env, "burn");

        let filtered = events.filter_by_contract(fixture.token_contract_id());
        for event in filtered.events() {
            let soroban_sdk::xdr::ContractEventBody::V0(body) = &event.body;
            let Ok(topics) = soroban_sdk::Vec::<soroban_sdk::Val>::try_from_val(env, &body.topics)
            else {
                continue;
            };
            let Ok(data) = soroban_sdk::Val::try_from_val(env, &body.data) else {
                continue;
            };
            let contract_id = fixture.token_contract_id().clone();
            if contract_id == *fixture.token_contract_id() && topics.len() >= 2 {
                let mut iter = topics.into_iter();
                let t0 = iter.next();
                let t1 = iter.next();
                if let (Some(t0), Some(t1)) = (t0, t1) {
                    if let (Ok(sym), Ok(from)) = (
                        soroban_sdk::Symbol::try_from_val(env, &t0),
                        soroban_sdk::Address::try_from_val(env, &t1),
                    ) {
                        if sym == burn_symbol
                            && &from == alice
                            && crate::event_helpers::verify_amount_data(env, &data, burn_amount)
                        {
                            found_valid_event = true;
                            break;
                        }
                    }
                }
            }
        }

        if found_valid_event {
            TestResult {
                test_id: <Self as Scenario<F>>::id(self),
                description: <Self as Scenario<F>>::description(self),
                status: Status::Pass,
                expected_behavior: "Emits burn event with exact topics and valid data format",
                observed_behavior: "burn_from event found matching specification",
            }
        } else {
            TestResult {
                test_id: <Self as Scenario<F>>::id(self),
                description: <Self as Scenario<F>>::description(self),
                status: Status::Fail,
                expected_behavior: "Emits burn event with exact topics and valid data format",
                observed_behavior:
                    "Burn_from event missing or lacked required data matching amount burned",
            }
        }
    }
}
