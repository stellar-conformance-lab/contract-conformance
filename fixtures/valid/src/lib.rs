#![no_std]

use conformance_core::fixture::Fixture;
use conformance_sep41::Sep41Fixture;
use soroban_sdk::{testutils::Address as _, Address, Env};
use soroban_sdk::token::StellarAssetClient;

#[derive(Debug)]
pub enum ValidFixtureError {
    SetupFailed,
}

pub struct ValidSep41Fixture {
    pub admin: Address,
    pub alice: Address,
    pub token_id: Address,
}

impl ValidSep41Fixture {
    pub fn new(env: &Env) -> Self {
        let admin = Address::generate(env);
        let alice = Address::generate(env);
        
        // Use the official built-in Stellar Asset Contract as our reference SEP-41 implementation.
        // This is a known-good, deterministic implementation running locally.
        let token_id = env.register_stellar_asset_contract(admin.clone());
        
        Self {
            admin,
            alice,
            token_id,
        }
    }
}

impl Fixture for ValidSep41Fixture {
    type Error = ValidFixtureError;

    fn setup(&self, env: &Env) -> Result<(), Self::Error> {
        let sac_client = StellarAssetClient::new(env, &self.token_id);
        
        // Establish a deterministic initial balance for the test account.
        // This simulates whatever internal mechanism a token uses to distribute balances.
        // The conformance engine remains entirely unaware of this internal mint operation.
        sac_client.mint(&self.alice, &1000);
        
        Ok(())
    }
}

impl Sep41Fixture for ValidSep41Fixture {
    fn token_contract_id(&self) -> &Address {
        &self.token_id
    }

    fn test_account_1(&self) -> &Address {
        &self.alice
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use conformance_core::engine::ConformanceEngine;
    use conformance_sep41::{MetadataScenario, InitialBalanceScenario};
    use conformance_core::result::Status;

    #[test]
    fn demonstrate_valid_fixture() {
        // Run Metadata scenario in isolation
        let meta_result = ConformanceEngine::run_isolated_scenario(
            &MetadataScenario,
            |env| ValidSep41Fixture::new(env)
        );
        assert_eq!(meta_result.status, Status::Pass);

        // Run Balance scenario in isolation
        let bal_result = ConformanceEngine::run_isolated_scenario(
            &InitialBalanceScenario,
            |env| ValidSep41Fixture::new(env)
        );
        assert_eq!(bal_result.status, Status::Pass);
    }
}
