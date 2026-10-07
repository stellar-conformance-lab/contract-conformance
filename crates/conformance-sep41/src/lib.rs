#![no_std]

use conformance_core::scenario::Scenario;
use conformance_core::result::{TestResult, Status};
use soroban_sdk::{token::Client as TokenClient, Address, Env};
use soroban_sdk::testutils::Ledger;

pub trait Sep41Fixture: conformance_core::fixture::Fixture {
    fn token_contract_id(&self) -> &Address;
    fn test_account_1(&self) -> &Address;
    fn test_account_2(&self) -> &Address;
    fn test_account_3(&self) -> &Address;
    fn expected_initial_balance(&self) -> i128;
}

// -----------------------------------------------------------------------------
// METADATA CONFORMANCE
// -----------------------------------------------------------------------------

pub struct MetaNameScenario;
impl<F: Sep41Fixture> Scenario<F> for MetaNameScenario {
    fn id(&self) -> &'static str { "SEP41-META-001" }
    fn description(&self) -> &'static str { "Token name" }
    fn run(&self, env: &Env, fixture: &F) -> TestResult {
        let client = TokenClient::new(env, fixture.token_contract_id());
        let _name = client.name();
        TestResult {
            test_id: self.id(), description: self.description(), status: Status::Pass,
            expected_behavior: "Token name function executes successfully", observed_behavior: "Token name returned without error",
        }
    }
}

pub struct MetaSymbolScenario;
impl<F: Sep41Fixture> Scenario<F> for MetaSymbolScenario {
    fn id(&self) -> &'static str { "SEP41-META-002" }
    fn description(&self) -> &'static str { "Token symbol" }
    fn run(&self, env: &Env, fixture: &F) -> TestResult {
        let client = TokenClient::new(env, fixture.token_contract_id());
        let _symbol = client.symbol();
        TestResult {
            test_id: self.id(), description: self.description(), status: Status::Pass,
            expected_behavior: "Token symbol function executes successfully", observed_behavior: "Token symbol returned without error",
        }
    }
}

pub struct MetaDecimalsScenario;
impl<F: Sep41Fixture> Scenario<F> for MetaDecimalsScenario {
    fn id(&self) -> &'static str { "SEP41-META-003" }
    fn description(&self) -> &'static str { "Token decimals" }
    fn run(&self, env: &Env, fixture: &F) -> TestResult {
        let client = TokenClient::new(env, fixture.token_contract_id());
        let _decimals = client.decimals();
        TestResult {
            test_id: self.id(), description: self.description(), status: Status::Pass,
            expected_behavior: "Token decimals function executes successfully", observed_behavior: "Token decimals returned without error",
        }
    }
}

// -----------------------------------------------------------------------------
// BALANCE CONFORMANCE
// -----------------------------------------------------------------------------

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
            TestResult { test_id: self.id(), description: self.description(), status: Status::Pass, expected_behavior: "Account has expected initial balance", observed_behavior: "Balance matched expected value" }
        } else {
            TestResult { test_id: self.id(), description: self.description(), status: Status::Fail, expected_behavior: "Account has expected initial balance", observed_behavior: "Balance did not match expected value" }
        }
    }
}

pub struct BalZeroScenario;
impl<F: Sep41Fixture> Scenario<F> for BalZeroScenario {
    fn id(&self) -> &'static str { "SEP41-BAL-002" }
    fn description(&self) -> &'static str { "Zero balance" }
    fn run(&self, env: &Env, fixture: &F) -> TestResult {
        let client = TokenClient::new(env, fixture.token_contract_id());
        let account = fixture.test_account_2();
        let balance = client.balance(account);
        if balance == 0 {
            TestResult { test_id: self.id(), description: self.description(), status: Status::Pass, expected_behavior: "New account balance is zero", observed_behavior: "Balance is exactly zero" }
        } else {
            TestResult { test_id: self.id(), description: self.description(), status: Status::Fail, expected_behavior: "New account balance is zero", observed_behavior: "Balance is non-zero" }
        }
    }
}

// -----------------------------------------------------------------------------
// TRANSFER CONFORMANCE
// -----------------------------------------------------------------------------

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
            return TestResult { test_id: self.id(), description: self.description(), status: Status::Error, expected_behavior: "Fixture must have enough balance", observed_behavior: "Insufficient balance to perform test" };
        }

        env.mock_all_auths();
        client.transfer(alice, bob, &transfer_amount);

        let final_alice = client.balance(alice);
        let final_bob = client.balance(bob);

        if final_alice == initial_alice - transfer_amount && final_bob == initial_bob + transfer_amount {
            TestResult { test_id: self.id(), description: self.description(), status: Status::Pass, expected_behavior: "Balances update correctly based on transfer", observed_behavior: "Balances updated correctly" }
        } else {
            TestResult { test_id: self.id(), description: self.description(), status: Status::Fail, expected_behavior: "Balances update correctly based on transfer", observed_behavior: "Balances did not update correctly" }
        }
    }
}

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
                for topic in topics.into_iter() {
                    if topic.to_val() == transfer_symbol.to_val() {
                        found = true;
                        break;
                    }
                }
            }
        }

        if found {
            TestResult { test_id: self.id(), description: self.description(), status: Status::Pass, expected_behavior: "Emits transfer event", observed_behavior: "Transfer event found" }
        } else {
            TestResult { test_id: self.id(), description: self.description(), status: Status::Fail, expected_behavior: "Emits transfer event", observed_behavior: "Transfer event missing" }
        }
    }
}

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
        let transfer_amount = initial_alice + 1;

        env.mock_all_auths();
        let result = client.try_transfer(alice, bob, &transfer_amount);

        if result.is_ok() {
            return TestResult { test_id: self.id(), description: self.description(), status: Status::Fail, expected_behavior: "Transfer fails when balance is insufficient", observed_behavior: "Transfer succeeded unexpectedly" };
        }

        let final_alice = client.balance(alice);
        let final_bob = client.balance(bob);

        if final_alice != initial_alice || final_bob != initial_bob {
            return TestResult { test_id: self.id(), description: self.description(), status: Status::Fail, expected_behavior: "Balances remain unchanged after failure", observed_behavior: "Balances were mutated despite failed transfer" };
        }

        TestResult { test_id: self.id(), description: self.description(), status: Status::Pass, expected_behavior: "Transfer fails and balances are preserved", observed_behavior: "Transfer failed as expected without mutating state" }
    }
}

pub struct TransferAuthorizationScenario;
impl<F: Sep41Fixture> Scenario<F> for TransferAuthorizationScenario {
    fn id(&self) -> &'static str { "SEP41-TRANSFER-005" }
    fn description(&self) -> &'static str { "Transfer authorization" }
    fn run(&self, env: &Env, fixture: &F) -> TestResult {
        let client = TokenClient::new(env, fixture.token_contract_id());
        let alice = fixture.test_account_1();
        let bob = fixture.test_account_2();
        let transfer_amount = 10_i128;

        let result = client.try_transfer(alice, bob, &transfer_amount);
        if result.is_ok() {
            return TestResult { test_id: self.id(), description: self.description(), status: Status::Fail, expected_behavior: "Transfer requires Soroban authorization from sender", observed_behavior: "Transfer succeeded without authorization" };
        }

        TestResult { test_id: self.id(), description: self.description(), status: Status::Pass, expected_behavior: "Transfer requires authorization", observed_behavior: "Transfer failed when unauthorized" }
    }
}

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

        if result.is_ok() {
            return TestResult { test_id: self.id(), description: self.description(), status: Status::Fail, expected_behavior: "Transfer fails with negative amount", observed_behavior: "Transfer succeeded with negative amount" };
        }

        let final_alice = client.balance(alice);
        if initial_alice != final_alice {
            return TestResult { test_id: self.id(), description: self.description(), status: Status::Fail, expected_behavior: "Balances remain unchanged", observed_behavior: "Balances mutated on negative transfer" };
        }

        TestResult { test_id: self.id(), description: self.description(), status: Status::Pass, expected_behavior: "Transfer fails and state is preserved", observed_behavior: "Transfer failed as expected" }
    }
}

// -----------------------------------------------------------------------------
// ALLOWANCE CONFORMANCE
// -----------------------------------------------------------------------------

pub struct AllowanceQueryScenario;
impl<F: Sep41Fixture> Scenario<F> for AllowanceQueryScenario {
    fn id(&self) -> &'static str { "SEP41-ALLOWANCE-001" }
    fn description(&self) -> &'static str { "Allowance query" }
    fn run(&self, env: &Env, fixture: &F) -> TestResult {
        let client = TokenClient::new(env, fixture.token_contract_id());
        let alice = fixture.test_account_1();
        let carol = fixture.test_account_3();
        
        let initial_allowance = client.allowance(alice, carol);
        if initial_allowance != 0 {
            return TestResult { test_id: self.id(), description: self.description(), status: Status::Fail, expected_behavior: "Initial allowance is zero", observed_behavior: "Initial allowance is non-zero" };
        }

        env.mock_all_auths();
        let amount = 100_i128;
        let expiration = env.ledger().sequence() + 100;
        client.approve(alice, carol, &amount, &expiration);
        
        let final_allowance = client.allowance(alice, carol);
        if final_allowance == amount {
            TestResult { test_id: self.id(), description: self.description(), status: Status::Pass, expected_behavior: "Allowance reflects approved amount", observed_behavior: "Allowance matched approved amount" }
        } else {
            TestResult { test_id: self.id(), description: self.description(), status: Status::Fail, expected_behavior: "Allowance reflects approved amount", observed_behavior: "Allowance did not match approved amount" }
        }
    }
}

pub struct AllowanceApproveScenario;
impl<F: Sep41Fixture> Scenario<F> for AllowanceApproveScenario {
    fn id(&self) -> &'static str { "SEP41-ALLOWANCE-002" }
    fn description(&self) -> &'static str { "Approve behavior" }
    fn run(&self, env: &Env, fixture: &F) -> TestResult {
        let client = TokenClient::new(env, fixture.token_contract_id());
        let alice = fixture.test_account_1();
        let carol = fixture.test_account_3();
        let amount = 150_i128;
        let expiration = env.ledger().sequence() + 100;

        env.mock_all_auths();
        let result = client.try_approve(alice, carol, &amount, &expiration);

        if result.is_err() {
            return TestResult { test_id: self.id(), description: self.description(), status: Status::Fail, expected_behavior: "Approve executes successfully", observed_behavior: "Approve failed unexpectedly" };
        }

        let allowance = client.allowance(alice, carol);
        if allowance != amount {
            return TestResult { test_id: self.id(), description: self.description(), status: Status::Fail, expected_behavior: "allowance(owner, spender) equals approved amount", observed_behavior: "Allowance did not accurately match approval" };
        }

        TestResult { test_id: self.id(), description: self.description(), status: Status::Pass, expected_behavior: "Approve successfully commits exact allowance to state", observed_behavior: "Approve verified successfully" }
    }
}

pub struct AllowanceTransferFromScenario;
impl<F: Sep41Fixture> Scenario<F> for AllowanceTransferFromScenario {
    fn id(&self) -> &'static str { "SEP41-ALLOWANCE-003" }
    fn description(&self) -> &'static str { "transfer_from behavior and reduction" }
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

        env.mock_all_auths();
        client.approve(alice, carol, &amount, &expiration);
        client.transfer_from(carol, alice, bob, &transfer_amount);

        let final_alice = client.balance(alice);
        let final_bob = client.balance(bob);
        let final_allowance = client.allowance(alice, carol);

        // Verify expiration preservation: still valid at exactly 'expiration'
        env.ledger().with_mut(|li| li.sequence_number = expiration);
        let at_exp_allowance = client.allowance(alice, carol);

        // Verify it expires properly past 'expiration'
        env.ledger().with_mut(|li| li.sequence_number = expiration + 1);
        let expired_allowance = client.allowance(alice, carol);

        if final_alice == initial_alice - transfer_amount && 
           final_bob == initial_bob + transfer_amount &&
           final_allowance == amount - transfer_amount &&
           at_exp_allowance == amount - transfer_amount &&
           expired_allowance == 0 {
            TestResult { test_id: self.id(), description: self.description(), status: Status::Pass, expected_behavior: "transfer_from moves tokens exactly, reduces allowance, and preserves expiration", observed_behavior: "State transitions correctly processed" }
        } else {
            TestResult { test_id: self.id(), description: self.description(), status: Status::Fail, expected_behavior: "transfer_from moves tokens exactly, reduces allowance, and preserves expiration", observed_behavior: "State mutations failed to match specification requirements" }
        }
    }
}

pub struct AllowanceExpirationScenario;
impl<F: Sep41Fixture> Scenario<F> for AllowanceExpirationScenario {
    fn id(&self) -> &'static str { "SEP41-ALLOWANCE-004" }
    fn description(&self) -> &'static str { "Expiration bounds" }
    fn run(&self, env: &Env, fixture: &F) -> TestResult {
        let client = TokenClient::new(env, fixture.token_contract_id());
        let alice = fixture.test_account_1();
        let carol = fixture.test_account_3();
        let amount = 100_i128;
        let expiration = env.ledger().sequence() + 10;

        env.mock_all_auths();
        client.approve(alice, carol, &amount, &expiration);

        env.ledger().with_mut(|li| li.sequence_number = expiration);
        let exact_allowance = client.allowance(alice, carol);

        env.ledger().with_mut(|li| li.sequence_number = expiration + 1);
        let expired_allowance = client.allowance(alice, carol);

        if exact_allowance == amount && expired_allowance == 0 {
            TestResult { test_id: self.id(), description: self.description(), status: Status::Pass, expected_behavior: "Allowance is zero when current > live_until_ledger", observed_behavior: "Expiration semantics correctly adhered to" }
        } else {
            TestResult { test_id: self.id(), description: self.description(), status: Status::Fail, expected_behavior: "Allowance is zero when current > live_until_ledger", observed_behavior: "Expiration semantics violated" }
        }
    }
}

pub struct AllowanceInsufficientScenario;
impl<F: Sep41Fixture> Scenario<F> for AllowanceInsufficientScenario {
    fn id(&self) -> &'static str { "SEP41-ALLOWANCE-005" }
    fn description(&self) -> &'static str { "Insufficient allowance" }
    fn run(&self, env: &Env, fixture: &F) -> TestResult {
        let client = TokenClient::new(env, fixture.token_contract_id());
        let alice = fixture.test_account_1();
        let bob = fixture.test_account_2();
        let carol = fixture.test_account_3();
        
        let initial_alice = client.balance(alice);
        let initial_bob = client.balance(bob);
        let initial_allowance = 50_i128;
        let expiration = env.ledger().sequence() + 100;

        env.mock_all_auths();
        client.approve(alice, carol, &initial_allowance, &expiration);
        
        let request_amount = initial_allowance + 1;
        let result = client.try_transfer_from(carol, alice, bob, &request_amount);

        let final_alice = client.balance(alice);
        let final_bob = client.balance(bob);
        let final_allowance = client.allowance(alice, carol);

        if result.is_ok() {
            return TestResult { test_id: self.id(), description: self.description(), status: Status::Fail, expected_behavior: "transfer_from fails on insufficient allowance", observed_behavior: "transfer_from succeeded unexpectedly" };
        }

        if final_alice != initial_alice || final_bob != initial_bob || final_allowance != initial_allowance {
            return TestResult { test_id: self.id(), description: self.description(), status: Status::Fail, expected_behavior: "State unchanged on failure", observed_behavior: "State mutated despite failed transfer_from" };
        }

        TestResult { test_id: self.id(), description: self.description(), status: Status::Pass, expected_behavior: "Transfer fails cleanly without mutating state", observed_behavior: "Transfer rejected correctly" }
    }
}

pub struct AllowanceUnauthorizedApproveScenario;
impl<F: Sep41Fixture> Scenario<F> for AllowanceUnauthorizedApproveScenario {
    fn id(&self) -> &'static str { "SEP41-ALLOWANCE-006" }
    fn description(&self) -> &'static str { "Unauthorized approve" }
    fn run(&self, env: &Env, fixture: &F) -> TestResult {
        let client = TokenClient::new(env, fixture.token_contract_id());
        let alice = fixture.test_account_1();
        let carol = fixture.test_account_3();
        let amount = 100_i128;
        let expiration = env.ledger().sequence() + 100;

        let result = client.try_approve(alice, carol, &amount, &expiration);

        if result.is_ok() {
            return TestResult { test_id: self.id(), description: self.description(), status: Status::Fail, expected_behavior: "Approve requires from authorization", observed_behavior: "Approve succeeded without authorization" };
        }

        let final_allowance = client.allowance(alice, carol);
        if final_allowance != 0 {
            return TestResult { test_id: self.id(), description: self.description(), status: Status::Fail, expected_behavior: "Allowance unchanged after failed auth", observed_behavior: "Allowance mutated without authorization" };
        }

        TestResult { test_id: self.id(), description: self.description(), status: Status::Pass, expected_behavior: "Unauthorized approve fails cleanly", observed_behavior: "Failed cleanly without state mutation" }
    }
}

pub struct AllowanceUnauthorizedTransferFromScenario;
impl<F: Sep41Fixture> Scenario<F> for AllowanceUnauthorizedTransferFromScenario {
    fn id(&self) -> &'static str { "SEP41-ALLOWANCE-007" }
    fn description(&self) -> &'static str { "Unauthorized transfer_from" }
    fn run(&self, env: &Env, fixture: &F) -> TestResult {
        let client = TokenClient::new(env, fixture.token_contract_id());
        let alice = fixture.test_account_1();
        let bob = fixture.test_account_2();
        let carol = fixture.test_account_3();
        let amount = 50_i128;
        
        let result = client.try_transfer_from(carol, alice, bob, &amount);

        if result.is_ok() {
            return TestResult { test_id: self.id(), description: self.description(), status: Status::Fail, expected_behavior: "transfer_from requires spender authorization", observed_behavior: "transfer_from succeeded without authorization" };
        }

        TestResult { test_id: self.id(), description: self.description(), status: Status::Pass, expected_behavior: "Unauthorized transfer_from fails cleanly", observed_behavior: "Failed cleanly without authorization" }
    }
}

pub struct AllowanceEventScenario;
impl<F: Sep41Fixture> Scenario<F> for AllowanceEventScenario {
    fn id(&self) -> &'static str { "SEP41-ALLOWANCE-008" }
    fn description(&self) -> &'static str { "Approve event emission" }
    fn run(&self, env: &Env, fixture: &F) -> TestResult {
        let client = TokenClient::new(env, fixture.token_contract_id());
        let alice = fixture.test_account_1();
        let carol = fixture.test_account_3();
        let amount = 10_i128;
        let expiration = env.ledger().sequence() + 100;

        env.mock_all_auths();
        client.approve(alice, carol, &amount, &expiration);

        let events = env.events().all();
        let mut found_topic = false;
        let approve_symbol = soroban_sdk::Symbol::new(env, "approve");

        for (contract_id, topics, _data) in events.into_iter() {
            if contract_id == *fixture.token_contract_id() {
                let mut has_symbol = false;
                let mut has_owner = false;
                let mut has_spender = false;
                
                for topic in topics.into_iter() {
                    let val = topic.to_val();
                    if val == approve_symbol.to_val() { has_symbol = true; }
                    if val == alice.to_val() { has_owner = true; }
                    if val == carol.to_val() { has_spender = true; }
                }

                if has_symbol && has_owner && has_spender {
                    found_topic = true;
                    break;
                }
            }
        }

        if found_topic {
            TestResult { test_id: self.id(), description: self.description(), status: Status::Pass, expected_behavior: "Emits approve event with required topics", observed_behavior: "Approve event found with required semantic topics" }
        } else {
            TestResult { test_id: self.id(), description: self.description(), status: Status::Fail, expected_behavior: "Emits approve event with required topics", observed_behavior: "Approve event missing required semantic topics" }
        }
    }
}

pub struct AllowanceZeroRevocationScenario;
impl<F: Sep41Fixture> Scenario<F> for AllowanceZeroRevocationScenario {
    fn id(&self) -> &'static str { "SEP41-ALLOWANCE-009" }
    fn description(&self) -> &'static str { "Zero-amount revocation" }
    fn run(&self, env: &Env, fixture: &F) -> TestResult {
        let client = TokenClient::new(env, fixture.token_contract_id());
        let alice = fixture.test_account_1();
        let carol = fixture.test_account_3();
        let amount = 100_i128;
        let expiration = env.ledger().sequence() + 100;

        env.mock_all_auths();
        client.approve(alice, carol, &amount, &expiration);
        let initial_allowance = client.allowance(alice, carol);
        
        let zero_amount = 0_i128;
        client.approve(alice, carol, &zero_amount, &expiration);
        let revoked_allowance = client.allowance(alice, carol);

        if initial_allowance == amount && revoked_allowance == 0 {
            TestResult { test_id: self.id(), description: self.description(), status: Status::Pass, expected_behavior: "approve with amount=0 revokes allowance completely", observed_behavior: "Allowance successfully revoked to 0" }
        } else {
            TestResult { test_id: self.id(), description: self.description(), status: Status::Fail, expected_behavior: "approve with amount=0 revokes allowance completely", observed_behavior: "Allowance was not revoked accurately" }
        }
    }
}

pub struct AllowanceOverwriteScenario;
impl<F: Sep41Fixture> Scenario<F> for AllowanceOverwriteScenario {
    fn id(&self) -> &'static str { "SEP41-ALLOWANCE-010" }
    fn description(&self) -> &'static str { "Approve overwrite behavior" }
    fn run(&self, env: &Env, fixture: &F) -> TestResult {
        let client = TokenClient::new(env, fixture.token_contract_id());
        let alice = fixture.test_account_1();
        let carol = fixture.test_account_3();
        let amount1 = 100_i128;
        let exp1 = env.ledger().sequence() + 50;
        let amount2 = 40_i128;
        let exp2 = env.ledger().sequence() + 100;

        env.mock_all_auths();
        client.approve(alice, carol, &amount1, &exp1);
        client.approve(alice, carol, &amount2, &exp2);
        
        let final_allowance = client.allowance(alice, carol);
        
        env.ledger().with_mut(|li| li.sequence_number = exp2);
        let at_exp2_allowance = client.allowance(alice, carol);

        if final_allowance == amount2 && at_exp2_allowance == amount2 {
            TestResult { test_id: self.id(), description: self.description(), status: Status::Pass, expected_behavior: "Subsequent approve fully overwrites amount and expiration", observed_behavior: "Allowance successfully overwritten" }
        } else {
            TestResult { test_id: self.id(), description: self.description(), status: Status::Fail, expected_behavior: "Subsequent approve fully overwrites amount and expiration", observed_behavior: "Allowance overwrite failed" }
        }
    }
}
