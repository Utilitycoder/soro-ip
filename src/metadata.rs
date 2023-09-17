use crate::storage_types::{ContractState, DataKey};
use soroban_sdk::Env;

const CONTRACT_STATE_KEY: DataKey = DataKey::ContractState;

pub fn sign_contract(env: &Env, date: &u64) {
    let acceptance_date_key = DataKey::DateOfAcceptance;
    env.storage()
        .set(&CONTRACT_STATE_KEY, &ContractState::Active);
    env.storage().set(&acceptance_date_key, date);
}

pub fn is_contract_with_state(env: &Env) -> bool {
    env.storage().has(&CONTRACT_STATE_KEY)
}

pub fn get_contract_state(env: &Env) -> ContractState {
    env.storage().get_unchecked(&CONTRACT_STATE_KEY).unwrap()
}

pub fn is_contract_active(env: &Env) -> bool {
    match env.storage().get(&CONTRACT_STATE_KEY) {
        Some(state) => matches!(state.unwrap(), ContractState::Active),
        None => false,
    }
}

pub fn get_fee_profit(env: &Env) -> i128 {
    let key = DataKey::FeeProfit;
    match env.storage().get(&key) {
        Some(claimable_fee) => claimable_fee.unwrap(),
        None => 0,
    }
}

pub fn update_fee(env: &Env, amount: &i128) {
    let fee_key = DataKey::FeeProfit;
    let fee: i128 = match env.storage().get(&fee_key) {
        Some(fee) => fee.unwrap(),
        None => 0,
    };
    env.storage().set(&fee_key, &(fee + amount))
}