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
    pub bob: Address,
    pub carol: Address,
    pub token_id: Address,
}

impl ValidSep41Fixture {
    pub fn new(env: &Env) -> Self {
        let admin = Address::generate(env);
        let alice = Address::generate(env);
        let bob = Address::generate(env);
        let carol = Address::generate(env);
        
        // Use the official Stellar Asset Contract as the deterministic reference implementation under test.
        let token_id = env.register_stellar_asset_contract(admin.clone());
        
        Self {
            admin,
            alice,
            bob,
            carol,
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
        sac_client.mint(&self.alice, &self.expected_initial_balance());
        
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

    fn test_account_2(&self) -> &Address {
        &self.bob
    }

    fn test_account_3(&self) -> &Address {
        &self.carol
    }

    fn expected_initial_balance(&self) -> i128 {
        1000
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use conformance_core::engine::ConformanceEngine;
    use conformance_core::result::Status;
    use conformance_sep41::{
        MetaNameScenario, MetaSymbolScenario, MetaDecimalsScenario,
        BalInitialScenario, BalZeroScenario,
        TransferSuccessScenario, TransferEventScenario, 
        TransferInsufficientBalanceScenario, TransferAuthorizationScenario,
        TransferNegativeAmountScenario,
        AllowanceQueryScenario, AllowanceApproveScenario,
        AllowanceTransferFromScenario, AllowanceExpirationScenario,
        AllowanceInsufficientScenario, AllowanceUnauthorizedApproveScenario,
        AllowanceUnauthorizedTransferFromScenario, AllowanceEventScenario,
        AllowanceZeroRevocationScenario, AllowanceOverwriteScenario
    };

    #[test]
    fn demonstrate_valid_fixture() {
        let results = [
            ConformanceEngine::run_isolated_scenario(&MetaNameScenario, |env| ValidSep41Fixture::new(env)),
            ConformanceEngine::run_isolated_scenario(&MetaSymbolScenario, |env| ValidSep41Fixture::new(env)),
            ConformanceEngine::run_isolated_scenario(&MetaDecimalsScenario, |env| ValidSep41Fixture::new(env)),
            ConformanceEngine::run_isolated_scenario(&BalInitialScenario, |env| ValidSep41Fixture::new(env)),
            ConformanceEngine::run_isolated_scenario(&BalZeroScenario, |env| ValidSep41Fixture::new(env)),
            ConformanceEngine::run_isolated_scenario(&TransferSuccessScenario, |env| ValidSep41Fixture::new(env)),
            ConformanceEngine::run_isolated_scenario(&TransferEventScenario, |env| ValidSep41Fixture::new(env)),
            ConformanceEngine::run_isolated_scenario(&TransferInsufficientBalanceScenario, |env| ValidSep41Fixture::new(env)),
            ConformanceEngine::run_isolated_scenario(&TransferAuthorizationScenario, |env| ValidSep41Fixture::new(env)),
            ConformanceEngine::run_isolated_scenario(&TransferNegativeAmountScenario, |env| ValidSep41Fixture::new(env)),
            ConformanceEngine::run_isolated_scenario(&AllowanceQueryScenario, |env| ValidSep41Fixture::new(env)),
            ConformanceEngine::run_isolated_scenario(&AllowanceApproveScenario, |env| ValidSep41Fixture::new(env)),
            ConformanceEngine::run_isolated_scenario(&AllowanceTransferFromScenario, |env| ValidSep41Fixture::new(env)),
            ConformanceEngine::run_isolated_scenario(&AllowanceExpirationScenario, |env| ValidSep41Fixture::new(env)),
            ConformanceEngine::run_isolated_scenario(&AllowanceInsufficientScenario, |env| ValidSep41Fixture::new(env)),
            ConformanceEngine::run_isolated_scenario(&AllowanceUnauthorizedApproveScenario, |env| ValidSep41Fixture::new(env)),
            ConformanceEngine::run_isolated_scenario(&AllowanceUnauthorizedTransferFromScenario, |env| ValidSep41Fixture::new(env)),
            ConformanceEngine::run_isolated_scenario(&AllowanceEventScenario, |env| ValidSep41Fixture::new(env)),
            ConformanceEngine::run_isolated_scenario(&AllowanceZeroRevocationScenario, |env| ValidSep41Fixture::new(env)),
            ConformanceEngine::run_isolated_scenario(&AllowanceOverwriteScenario, |env| ValidSep41Fixture::new(env)),
        ];

        for result in results {
            assert_eq!(result.status, Status::Pass);
        }
    }
}
