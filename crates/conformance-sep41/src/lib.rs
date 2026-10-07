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

pub struct BalIsolationScenario;
impl<F: Sep41Fixture> Scenario<F> for BalIsolationScenario {
    fn id(&self) -> &'static str { "SEP41-BAL-003" }
    fn description(&self) -> &'static str { "Balance isolation" }
    fn run(&self, env: &Env, fixture: &F) -> TestResult {
        let client = TokenClient::new(env, fixture.token_contract_id());
        let account1 = fixture.test_account_1();
        let account2 = fixture.test_account_2();
        let expected = fixture.expected_initial_balance();
        
        // Querying account 2 shouldn't mutate account 1
        let _bal2 = client.balance(account2);
        let bal1 = client.balance(account1);
        
        if bal1 == expected {
            TestResult {
                test_id: self.id(),
                description: self.description(),
                status: Status::Pass,
                expected_behavior: "Querying balances does not mutate them",
                observed_behavior: "Balance preserved",
            }
        } else {
            TestResult {
                test_id: self.id(),
                description: self.description(),
                status: Status::Fail,
                expected_behavior: "Querying balances does not mutate them",
                observed_behavior: "Balance was mutated unexpectedly",
            }
        }
    }
}
