#![no_std]

use conformance_core::scenario::Scenario;
use conformance_core::result::{TestResult, Status};
use soroban_sdk::Env;

pub trait Sep41Fixture: conformance_core::fixture::Fixture {
    fn token_contract_id(&self) -> &soroban_sdk::Address;
}

pub struct DummyMetadataScenario;

impl<F: Sep41Fixture> Scenario<F> for DummyMetadataScenario {
    fn id(&self) -> &'static str {
        "SEP41-META-001"
    }

    fn description(&self) -> &'static str {
        "Demonstrates a passing metadata scenario"
    }

    fn run(&self, _env: &Env, _fixture: &F) -> TestResult {
        TestResult {
            test_id: self.id(),
            description: self.description(),
            status: Status::Pass,
            expected_behavior: "Token name is valid",
            observed_behavior: "Token name matched",
        }
    }
}
