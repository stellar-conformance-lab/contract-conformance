#![no_std]

use conformance_core::engine::ConformanceEngine;
use conformance_core::fixture::Fixture;
use conformance_core::result::Status;
use conformance_sep41::Sep41Fixture;
use soroban_sdk::{contract, contractimpl, symbol_short, vec, Address, Env, IntoVal, Symbol};

// -----------------------------------------------------------------------------
// MACRO FOR FIXTURE BOILERPLATE
// -----------------------------------------------------------------------------
macro_rules! impl_fixture {
    ($fixture:ident, $contract:ident) => {
        pub struct $fixture {
            pub alice: Address,
            pub bob: Address,
            pub carol: Address,
            pub token_id: Address,
        }

        impl $fixture {
            pub fn new(env: &Env) -> Self {
                let alice = Address::generate(env);
                let bob = Address::generate(env);
                let carol = Address::generate(env);
                let token_id = env.register_contract(None, $contract);
                Self {
                    alice,
                    bob,
                    carol,
                    token_id,
                }
            }
        }

        impl Fixture for $fixture {
            type Error = ();
            fn setup(&self, env: &Env) -> Result<(), Self::Error> {
                env.invoke_contract::<()>(
                    &self.token_id,
                    &Symbol::new(env, "mint"),
                    vec![
                        env,
                        self.alice.into_val(env),
                        self.expected_initial_balance().into_val(env),
                    ],
                );
                Ok(())
            }
        }

        impl Sep41Fixture for $fixture {
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
    };
}

// -----------------------------------------------------------------------------
// 1. INCORRECT TRANSFER BALANCE BEHAVIOR
// -----------------------------------------------------------------------------
#[contract]
pub struct BrokenTransferContract;

#[contractimpl]
impl BrokenTransferContract {
    pub fn balance(env: Env, id: Address) -> i128 {
        env.storage().instance().get(&id).unwrap_or(0)
    }
    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();
        let to_bal: i128 = env.storage().instance().get(&to).unwrap_or(0);
        env.storage().instance().set(&to, &(to_bal + amount));
        // INTENTIONAL VIOLATION: from balance is not reduced.
    }
    pub fn mint(env: Env, to: Address, amount: i128) {
        env.storage().instance().set(&to, &amount);
    }
}
impl_fixture!(BrokenTransferFixture, BrokenTransferContract);

// -----------------------------------------------------------------------------
// 2. INCORRECT ALLOWANCE CONSUMPTION
// -----------------------------------------------------------------------------
#[contract]
pub struct BrokenAllowanceConsumptionContract;

#[contractimpl]
impl BrokenAllowanceConsumptionContract {
    pub fn balance(env: Env, id: Address) -> i128 {
        env.storage().instance().get(&id).unwrap_or(0)
    }
    pub fn allowance(env: Env, from: Address, spender: Address) -> i128 {
        env.storage().instance().get(&(from, spender)).unwrap_or(0)
    }
    pub fn approve(
        env: Env,
        from: Address,
        spender: Address,
        amount: i128,
        _expiration_ledger: u32,
    ) {
        from.require_auth();
        env.storage().instance().set(&(from, spender), &amount);
    }
    pub fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128) {
        spender.require_auth();
        let from_bal: i128 = env.storage().instance().get(&from).unwrap_or(0);
        let to_bal: i128 = env.storage().instance().get(&to).unwrap_or(0);
        env.storage().instance().set(&from, &(from_bal - amount));
        env.storage().instance().set(&to, &(to_bal + amount));
        // INTENTIONAL VIOLATION: allowance is not reduced!
    }
    pub fn mint(env: Env, to: Address, amount: i128) {
        env.storage().instance().set(&to, &amount);
    }
}
impl_fixture!(
    BrokenAllowanceConsumptionFixture,
    BrokenAllowanceConsumptionContract
);

// -----------------------------------------------------------------------------
// 3. INCORRECT BURN BALANCE BEHAVIOR
// -----------------------------------------------------------------------------
#[contract]
pub struct BrokenBurnContract;

#[contractimpl]
impl BrokenBurnContract {
    pub fn balance(env: Env, id: Address) -> i128 {
        env.storage().instance().get(&id).unwrap_or(0)
    }
    pub fn burn(env: Env, from: Address, _amount: i128) {
        from.require_auth();
        // INTENTIONAL VIOLATION: balance is not reduced!
    }
    pub fn mint(env: Env, to: Address, amount: i128) {
        env.storage().instance().set(&to, &amount);
    }
}
impl_fixture!(BrokenBurnFixture, BrokenBurnContract);

// -----------------------------------------------------------------------------
// 4. INCORRECT APPROVAL OVERWRITE BEHAVIOR
// -----------------------------------------------------------------------------
#[contract]
pub struct BrokenApprovalOverwriteContract;

#[contractimpl]
impl BrokenApprovalOverwriteContract {
    pub fn balance(env: Env, id: Address) -> i128 {
        env.storage().instance().get(&id).unwrap_or(0)
    }
    pub fn allowance(env: Env, from: Address, spender: Address) -> i128 {
        env.storage().instance().get(&(from, spender)).unwrap_or(0)
    }
    pub fn approve(
        env: Env,
        from: Address,
        spender: Address,
        amount: i128,
        _expiration_ledger: u32,
    ) {
        from.require_auth();
        // INTENTIONAL VIOLATION: adds to allowance instead of overwriting
        let current: i128 = env
            .storage()
            .instance()
            .get(&(from.clone(), spender.clone()))
            .unwrap_or(0);
        env.storage()
            .instance()
            .set(&(from, spender), &(current + amount));
    }
    pub fn mint(env: Env, to: Address, amount: i128) {
        env.storage().instance().set(&to, &amount);
    }
}
impl_fixture!(
    BrokenApprovalOverwriteFixture,
    BrokenApprovalOverwriteContract
);

// -----------------------------------------------------------------------------
// 5. INCORRECT EVENT DATA
// -----------------------------------------------------------------------------
#[contract]
pub struct BrokenEventDataContract;

#[contractimpl]
impl BrokenEventDataContract {
    pub fn balance(env: Env, id: Address) -> i128 {
        env.storage().instance().get(&id).unwrap_or(0)
    }
    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();
        let from_bal: i128 = env.storage().instance().get(&from).unwrap_or(0);
        let to_bal: i128 = env.storage().instance().get(&to).unwrap_or(0);
        env.storage().instance().set(&from, &(from_bal - amount));
        env.storage().instance().set(&to, &(to_bal + amount));

        // INTENTIONAL VIOLATION: emits wrong amount!
        let wrong_amount = amount + 1;
        env.events()
            .publish((Symbol::new(&env, "transfer"), from, to), wrong_amount);
    }
    pub fn mint(env: Env, to: Address, amount: i128) {
        env.storage().instance().set(&to, &amount);
    }
}
impl_fixture!(BrokenEventDataFixture, BrokenEventDataContract);

// -----------------------------------------------------------------------------
// 6. INCORRECT AUTHORIZATION BEHAVIOR
// -----------------------------------------------------------------------------
#[contract]
pub struct BrokenAuthorizationContract;

#[contractimpl]
impl BrokenAuthorizationContract {
    pub fn balance(env: Env, id: Address) -> i128 {
        env.storage().instance().get(&id).unwrap_or(0)
    }
    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        // INTENTIONAL VIOLATION: missing `from.require_auth();`
        let from_bal: i128 = env.storage().instance().get(&from).unwrap_or(0);
        let to_bal: i128 = env.storage().instance().get(&to).unwrap_or(0);
        env.storage().instance().set(&from, &(from_bal - amount));
        env.storage().instance().set(&to, &(to_bal + amount));
    }
    pub fn mint(env: Env, to: Address, amount: i128) {
        env.storage().instance().set(&to, &amount);
    }
}
impl_fixture!(BrokenAuthorizationFixture, BrokenAuthorizationContract);

// -----------------------------------------------------------------------------
// TESTS
// -----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use conformance_sep41::{
        AllowanceOverwriteScenario, AllowanceTransferFromScenario, BurnSuccessScenario,
        TransferAuthorizationScenario, TransferEventScenario, TransferSuccessScenario,
    };

    #[test]
    fn test_broken_transfer_balance() {
        // Expected violation: Transfer balance mismatch -> Fail
        let result = ConformanceEngine::run_isolated_scenario(&TransferSuccessScenario, |env| {
            BrokenTransferFixture::new(env)
        });
        assert_eq!(
            result.status,
            Status::Fail,
            "Expected framework to detect broken transfer balance"
        );
    }

    #[test]
    fn test_broken_allowance_consumption() {
        // Expected violation: transfer_from doesn't reduce allowance -> Fail
        let result =
            ConformanceEngine::run_isolated_scenario(&AllowanceTransferFromScenario, |env| {
                BrokenAllowanceConsumptionFixture::new(env)
            });
        assert_eq!(
            result.status,
            Status::Fail,
            "Expected framework to detect broken allowance consumption"
        );
    }

    #[test]
    fn test_broken_burn_balance() {
        // Expected violation: burn doesn't reduce balance -> Fail
        let result = ConformanceEngine::run_isolated_scenario(&BurnSuccessScenario, |env| {
            BrokenBurnFixture::new(env)
        });
        assert_eq!(
            result.status,
            Status::Fail,
            "Expected framework to detect broken burn balance"
        );
    }

    #[test]
    fn test_broken_approval_overwrite() {
        // Expected violation: approve adds instead of overwrites -> Fail
        let result = ConformanceEngine::run_isolated_scenario(&AllowanceOverwriteScenario, |env| {
            BrokenApprovalOverwriteFixture::new(env)
        });
        assert_eq!(
            result.status,
            Status::Fail,
            "Expected framework to detect broken approval overwrite"
        );
    }

    #[test]
    fn test_broken_event_data() {
        // Expected violation: transfer event emits wrong amount -> Fail
        let result = ConformanceEngine::run_isolated_scenario(&TransferEventScenario, |env| {
            BrokenEventDataFixture::new(env)
        });
        assert_eq!(
            result.status,
            Status::Fail,
            "Expected framework to detect broken event data"
        );
    }

    #[test]
    fn test_broken_authorization() {
        // Expected violation: transfer succeeds without authorization -> Fail
        let result =
            ConformanceEngine::run_isolated_scenario(&TransferAuthorizationScenario, |env| {
                BrokenAuthorizationFixture::new(env)
            });
        assert_eq!(
            result.status,
            Status::Fail,
            "Expected framework to detect broken authorization"
        );
    }
}

