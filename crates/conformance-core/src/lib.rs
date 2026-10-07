#![no_std]

pub mod result {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum Status {
        Pass,
        Fail,
        Error,
        Skipped,
    }

    #[derive(Debug, Clone)]
    pub struct TestResult {
        pub test_id: &'static str,
        pub description: &'static str,
        pub status: Status,
        pub expected_behavior: &'static str,
        pub observed_behavior: &'static str,
    }
}

pub mod fixture {
    use soroban_sdk::Env;
    pub trait Fixture {
        type Error: core::fmt::Debug;
        fn setup(&self, env: &Env) -> Result<(), Self::Error>;
    }
}

pub mod scenario {
    use soroban_sdk::Env;
    use crate::result::TestResult;
    pub trait Scenario<F> {
        fn id(&self) -> &'static str;
        fn description(&self) -> &'static str;
        fn run(&self, env: &Env, fixture: &F) -> TestResult;
    }
}

pub mod engine {
    use soroban_sdk::Env;
    use crate::fixture::Fixture;
    use crate::scenario::Scenario;
    use crate::result::{TestResult, Status};

    pub struct ConformanceEngine<'a, F> {
        env: &'a Env,
        fixture: F,
    }

    impl<'a, F: Fixture> ConformanceEngine<'a, F> {
        pub fn new(env: &'a Env, fixture: F) -> Self {
            Self { env, fixture }
        }

        pub fn run_scenario<S: Scenario<F>>(&self, scenario: &S) -> TestResult {
            if let Err(_e) = self.fixture.setup(self.env) {
                return TestResult {
                    test_id: scenario.id(),
                    description: scenario.description(),
                    status: Status::Error,
                    expected_behavior: "Setup completes successfully",
                    observed_behavior: "Setup failed",
                };
            }
            scenario.run(self.env, &self.fixture)
        }
    }
}
