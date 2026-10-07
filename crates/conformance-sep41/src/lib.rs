#![no_std]

use conformance_core::scenario::Scenario;
use conformance_core::result::{TestResult, Status};
use soroban_sdk::{token::Client as TokenClient, Address, Env};

pub trait Sep41Fixture: conformance_core::fixture::Fixture {
    fn token_contract_id(&self) -> &Address;
    fn test_account_1(&self) -> &Address;
}

pub struct MetadataScenario;

impl<F: Sep41Fixture> Scenario<F> for MetadataScenario {
    fn id(&self) -> &'static str {
        "SEP41-META-001"
    }

    fn description(&self) -> &'static str {
        "Token name, symbol, and decimals can be observed"
    }

    fn run(&self, env: &Env, fixture: &F) -> TestResult {
        let client = TokenClient::new(env, fixture.token_contract_id());
        
        let _name = client.name();
        let _symbol = client.symbol();
        let decimals = client.decimals();
        
        if decimals != 7 {
            return TestResult {
                test_id: self.id(),
                description: self.description(),
                status: Status::Fail,
                expected_behavior: "Metadata successfully retrieved and decimals = 7",
                observed_behavior: "Decimals mismatch",
            };
        }

        TestResult {
            test_id: self.id(),
            description: self.description(),
            status: Status::Pass,
            expected_behavior: "Metadata successfully retrieved",
            observed_behavior: "Token exposed name, symbol, decimals",
        }
    }
}

pub struct InitialBalanceScenario;

impl<F: Sep41Fixture> Scenario<F> for InitialBalanceScenario {
    fn id(&self) -> &'static str {
        "SEP41-BAL-001"
    }

    fn description(&self) -> &'static str {
        "An established account has the expected initial balance"
    }

    fn run(&self, env: &Env, fixture: &F) -> TestResult {
        let client = TokenClient::new(env, fixture.token_contract_id());
        
        let alice = fixture.test_account_1();
        let balance = client.balance(alice);
        
        if balance == 1000 {
            TestResult {
                test_id: self.id(),
                description: self.description(),
                status: Status::Pass,
                expected_behavior: "Test account has initial balance of 1000",
                observed_behavior: "Balance is exactly 1000",
            }
        } else {
            TestResult {
                test_id: self.id(),
                description: self.description(),
                status: Status::Fail,
                expected_behavior: "Test account has initial balance of 1000",
                observed_behavior: "Balance is not 1000",
            }
        }
    }
}
