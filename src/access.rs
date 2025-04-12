use soroban_sdk::{crypto::bls12_381::G1Affine, Address, BytesN, Env, String, Vec};

use crate::{
    data::DataKey,
    error::ContractError,
    formatter::{convert_to_lower, convert_to_upper, to_lower_bytes},
    types::WebKeyDetails,
};

pub fn has_user_account(e: &Env) -> bool {
    let key = DataKey::UserAccountId;
    e.storage().instance().has(&key)
}

pub fn read_user_account(e: &Env) -> Result<Address, ContractError> {
    let key = DataKey::UserAccountId;
    e.storage()
        .instance()
        .get(&key)
        .ok_or(ContractError::UserAccountNotFound)
}

pub fn write_user_account(e: &Env, user_account_id: &Address) {
    let key = DataKey::UserAccountId;
    e.storage().instance().set(&key, user_account_id);
}

pub fn authenticate_user_account(e: &Env) {
    let user_account_id = read_user_account(e).unwrap();
    user_account_id.require_auth();
}

pub fn is_initialized(e: &Env) -> bool {
    let key = DataKey::BlsPublicKey;
    e.storage().persistent().has(&key)
}

pub fn write_aggregated_pk_bytes(env: &Env, keypair_pubkeys: Vec<BytesN<96>>) {
    let bls = env.crypto().bls12_381();

    let mut keypair_1_array = [0u8; 96];
    keypair_pubkeys
        .get_unchecked(0)
        .copy_into_slice(&mut keypair_1_array);

    let mut agg_pk = G1Affine::from_bytes(BytesN::from_array(env, &keypair_1_array));

    for i in 1..keypair_pubkeys.len() {
        let mut keypair_i_array = [0u8; 96];
        keypair_pubkeys
            .get_unchecked(i)
            .copy_into_slice(&mut keypair_i_array);

        let pk = G1Affine::from_bytes(BytesN::from_array(env, &keypair_i_array));
        agg_pk = bls.g1_add(&agg_pk, &pk);
    }

    env.storage()
        .persistent()
        .set(&DataKey::BlsPublicKey, &agg_pk.to_bytes());
}

pub fn read_aggregated_pk_bytes(env: &Env) -> Option<BytesN<96>> {
    env.storage().persistent().get(&DataKey::BlsPublicKey)
}

pub fn write_web_keys_bytes(
    env: &Env,
    platform: String,
    social_username: String,
    web_pubkey: BytesN<77>,
) {
    let mut web_keys = read_web_keys_bytes(env);

    web_keys.primary_social_acct = convert_to_lower(&env, social_username);
    web_keys.platform = convert_to_lower(&env, platform);
    env.storage()
        .persistent()
        .set(&DataKey::WebKey, &web_pubkey);
}

pub fn read_web_keys_bytes(env: &Env) -> WebKeyDetails {
    let default_webkeys = WebKeyDetails {
        web_public_key: BytesN::from_array(&env, &[0u8; 77]),
        primary_social_acct: String::from_str(&env, ""),
        platform: String::from_str(&env, ""),
    };
    env.storage()
        .persistent()
        .get(&DataKey::WebKey)
        .unwrap_or(default_webkeys)
}

pub fn read_master_contract_id(e: &Env) -> Option<Address> {
    let key = DataKey::MasterContractId;
    e.storage()
        .instance()
        .get(&key)
        .expect("Master Contract ID Not Found")
}

pub fn write_master_contract_id(e: &Env, master_contract_id: &Address) {
    let key = DataKey::MasterContractId;
    e.storage().instance().set(&key, master_contract_id);
}
pub fn read_dapp_router_contract_id(e: &Env) -> Option<Address> {
    let key = DataKey::DappRouterId;
    e.storage()
        .instance()
        .get(&key)
        .expect("Master Contract ID Not Found")
}

pub fn write_dapp_router_contract_id(e: &Env, dapp_router_contract_id: &Address) {
    let key = DataKey::DappRouterId;
    e.storage().instance().set(&key, dapp_router_contract_id);
}
