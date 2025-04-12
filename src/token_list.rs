use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, Env, FromVal, Map, Symbol, Val,
    Vec,
};

use crate::data::DataKey;

pub fn write_add_token(env: Env, token: Address) {
    let default_tokens: Map<Address, ()> = Map::new(&env);
    let key = DataKey::TokenList;
    let mut tokens: Map<Address, ()> = env
        .storage()
        .persistent()
        .get(&key)
        .unwrap_or(default_tokens);

    tokens.set(token, ());
    env.storage().persistent().set(&key, &tokens);
}

pub fn write_remove_token(env: Env, token: Address) {
    let default_tokens: Map<Address, ()> = Map::new(&env);
    let key = DataKey::TokenList;
    let mut tokens: Map<Address, ()> = env
        .storage()
        .persistent()
        .get(&key)
        .unwrap_or(default_tokens);
    tokens.remove(token);
    env.storage().persistent().set(&key, &tokens);
}

pub fn read_token_list(env: Env) -> Vec<Address> {
    let default_tokens: Map<Address, ()> = Map::new(&env);
    env.storage()
        .persistent()
        .get(&DataKey::TokenList)
        .unwrap_or(default_tokens)
        .keys()
}
