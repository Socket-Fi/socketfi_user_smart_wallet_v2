use soroban_sdk::{token, Address, Env, Vec};

use crate::data::DataKey;

pub fn take_token(env: &Env, from: &Address, token_id: &Address, amount: i128) {
    let token = token::Client::new(env, token_id);
    let contract_address = env.current_contract_address();
    token.transfer(from, &contract_address, &amount);
}

pub fn send_token(env: &Env, to: &Address, token_id: &Address, amount: i128) {
    let token = token::Client::new(env, token_id);
    let contract_address = env.current_contract_address();
    token.transfer(&contract_address, to, &amount);
}

pub fn spend_token(env: &Env, spender: &Address, token_id: &Address, amount: i128, to: &Address) {
    let token = token::Client::new(env, token_id);
    let contract_address = env.current_contract_address();
    token.transfer_from(&spender, &contract_address, to, &amount);
}

pub fn read_balance(env: &Env, token_id: &Address) -> i128 {
    let token = token::Client::new(env, token_id);
    let contract_address = env.current_contract_address();
    token.balance(&contract_address)
}

pub fn read_allowance(env: &Env, token_id: &Address, spender: &Address) -> i128 {
    let token = token::Client::new(env, token_id);
    let contract_address = env.current_contract_address();
    token.allowance(&contract_address, spender)
}

pub fn write_approve(env: &Env, token_id: &Address, spender: &Address, amount: &i128) {
    let token = token::Client::new(env, token_id);
    let contract_address = env.current_contract_address();
    let expiration = read_allowance_expiration(env) + env.ledger().sequence();
    token.approve(&contract_address, spender, amount, &expiration)
}

pub fn write_allowance_expiration(env: &Env, expiration_ledger: u32) {
    env.storage()
        .persistent()
        .set(&DataKey::AllowanceExpiration, &expiration_ledger);
}

pub fn read_allowance_expiration(env: &Env) -> u32 {
    env.storage()
        .persistent()
        .get(&DataKey::AllowanceExpiration)
        .unwrap_or(0)
}
