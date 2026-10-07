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

    pub struct ConformanceEngine;

    impl ConformanceEngine {
        /// Executes a scenario in a completely isolated, fresh environment.
        /// The caller provides a factory to generate a fresh fixture for the new environment,
        /// ensuring no state leaks between scenarios.
        pub fn run_isolated_scenario<F, S, Factory>(
            scenario: &S,
            fixture_factory: Factory,
        ) -> TestResult
        where
            F: Fixture,
            S: Scenario<F>,
            Factory: FnOnce(&Env) -> F,
        {
            // Enforce isolation by creating a fresh Env for this scenario run.
            // Requires the `testutils` feature in soroban-sdk.
            let env = Env::default();
            
            // The environment is prepared, so we can now construct the fixture which
            // likely depends on this specific Env instance (e.g., for Address types).
            let fixture = fixture_factory(&env);

            if let Err(_e) = fixture.setup(&env) {
                return TestResult {
                    test_id: scenario.id(),
                    description: scenario.description(),
                    status: Status::Error,
                    expected_behavior: "Setup completes successfully",
                    observed_behavior: "Setup failed",
                };
            }
            scenario.run(&env, &fixture)
        }
    }
}
