use crate::Sep41Fixture;
use conformance_core::fixture::Fixture;
use proptest::prelude::*;
use soroban_sdk::{token::Client as TokenClient, Env};

pub fn check_transfer_conservation<F: Sep41Fixture + Fixture>(
    fixture_factory: impl Fn(&Env) -> F,
    amount: i128,
) -> Result<(), TestCaseError> {
    let env = Env::default();
    env.mock_all_auths();
    let fixture = fixture_factory(&env);
    if fixture.setup(&env).is_err() {
        return Err(TestCaseError::Fail("Setup failed".into()));
    }

    let client = TokenClient::new(&env, fixture.token_contract_id());
    let sender = fixture.test_account_1();
    let receiver = fixture.test_account_2();

    let sender_before = client.balance(sender);
    let receiver_before = client.balance(receiver);

    prop_assume!(sender_before >= amount);
    prop_assume!(amount >= 0);

    client.transfer(sender, receiver, &amount);

    let sender_after = client.balance(sender);
    let receiver_after = client.balance(receiver);

    prop_assert_eq!(sender_after, sender_before - amount);
    prop_assert_eq!(receiver_after, receiver_before + amount);

    Ok(())
}

pub fn check_transfer_insufficient_balance<F: Sep41Fixture + Fixture>(
    fixture_factory: impl Fn(&Env) -> F,
    extra: i128,
) -> Result<(), TestCaseError> {
    let env = Env::default();
    env.mock_all_auths();
    let fixture = fixture_factory(&env);
    if fixture.setup(&env).is_err() {
        return Err(TestCaseError::Fail("Setup failed".into()));
    }

    let client = TokenClient::new(&env, fixture.token_contract_id());
    let sender = fixture.test_account_1();
    let receiver = fixture.test_account_2();

    let sender_before = client.balance(sender);
    let receiver_before = client.balance(receiver);

    prop_assume!(extra > 0);
    let amount = sender_before.saturating_add(extra);
    prop_assume!(amount > sender_before);

    // Soroban SDK doesn't have try_transfer generated in the standard token client interface,
    // wait! It is generated? Let's check `try_transfer` or `env.try_invoke_contract`.
    // We can use `env.try_invoke_contract` directly.
    use soroban_sdk::{vec, Symbol, IntoVal};
    
    let res = env.try_invoke_contract::<(), _>(
        fixture.token_contract_id(),
        &Symbol::new(&env, "transfer"),
        vec![&env, sender.into_val(&env), receiver.into_val(&env), amount.into_val(&env)]
    );

    prop_assert!(res.is_err(), "Transfer should fail for insufficient balance");

    // Stability: state should remain unchanged
    prop_assert_eq!(client.balance(sender), sender_before);
    prop_assert_eq!(client.balance(receiver), receiver_before);

    Ok(())
}

pub fn check_allowance_consistency<F: Sep41Fixture + Fixture>(
    fixture_factory: impl Fn(&Env) -> F,
    approve_amount: i128,
    transfer_amount: i128,
) -> Result<(), TestCaseError> {
    let env = Env::default();
    env.mock_all_auths();
    let fixture = fixture_factory(&env);
    if fixture.setup(&env).is_err() {
        return Err(TestCaseError::Fail("Setup failed".into()));
    }

    let client = TokenClient::new(&env, fixture.token_contract_id());
    let from = fixture.test_account_1();
    let spender = fixture.test_account_2();
    let to = fixture.test_account_3();

    let from_before = client.balance(from);
    let to_before = client.balance(to);

    prop_assume!(approve_amount >= 0);
    prop_assume!(transfer_amount >= 0);
    prop_assume!(from_before >= transfer_amount);
    prop_assume!(approve_amount >= transfer_amount);

    let expiration_ledger = env.ledger().sequence() + 200;
    client.approve(from, spender, &approve_amount, &expiration_ledger);

    let allowance_before = client.allowance(from, spender);
    prop_assert_eq!(allowance_before, approve_amount);

    client.transfer_from(spender, from, to, &transfer_amount);

    let from_after = client.balance(from);
    let to_after = client.balance(to);
    let allowance_after = client.allowance(from, spender);

    prop_assert_eq!(from_after, from_before - transfer_amount);
    prop_assert_eq!(to_after, to_before + transfer_amount);
    prop_assert_eq!(allowance_after, allowance_before - transfer_amount);

    // Verify expiration remains correct: advance ledger to expiration_ledger - 1, allowance should still be there
    env.ledger().set_sequence_number(expiration_ledger - 1);
    let allowance_at_exp = client.allowance(from, spender);
    prop_assert_eq!(allowance_at_exp, allowance_before - transfer_amount);

    // Advance past expiration
    env.ledger().set_sequence_number(expiration_ledger + 1);
    let allowance_expired = client.allowance(from, spender);
    prop_assert_eq!(allowance_expired, 0);

    Ok(())
}

pub fn check_allowance_insufficient<F: Sep41Fixture + Fixture>(
    fixture_factory: impl Fn(&Env) -> F,
    approve_amount: i128,
    extra: i128,
) -> Result<(), TestCaseError> {
    let env = Env::default();
    env.mock_all_auths();
    let fixture = fixture_factory(&env);
    if fixture.setup(&env).is_err() {
        return Err(TestCaseError::Fail("Setup failed".into()));
    }

    let client = TokenClient::new(&env, fixture.token_contract_id());
    let from = fixture.test_account_1();
    let spender = fixture.test_account_2();
    let to = fixture.test_account_3();

    let from_before = client.balance(from);
    let to_before = client.balance(to);

    prop_assume!(approve_amount >= 0);
    prop_assume!(extra > 0);
    let transfer_amount = approve_amount.saturating_add(extra);
    prop_assume!(transfer_amount > approve_amount);
    
    // Make sure we have enough balance, so it fails ONLY due to allowance
    if from_before < transfer_amount {
        // Mint more to `from` if needed, but since we can't mint via SEP41, we just assume it's small enough or use `prop_assume`.
        prop_assume!(from_before >= transfer_amount);
    }

    let expiration_ledger = env.ledger().sequence() + 200;
    client.approve(from, spender, &approve_amount, &expiration_ledger);

    use soroban_sdk::{vec, Symbol, IntoVal};
    let res = env.try_invoke_contract::<(), _>(
        fixture.token_contract_id(),
        &Symbol::new(&env, "transfer_from"),
        vec![&env, spender.into_val(&env), from.into_val(&env), to.into_val(&env), transfer_amount.into_val(&env)]
    );

    prop_assert!(res.is_err(), "transfer_from should fail for insufficient allowance");

    // Stability: state should remain unchanged
    prop_assert_eq!(client.balance(from), from_before);
    prop_assert_eq!(client.balance(to), to_before);
    prop_assert_eq!(client.allowance(from, spender), approve_amount);

    Ok(())
}
