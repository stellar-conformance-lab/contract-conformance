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
