use soroban_sdk::{contract, Address, BytesN, Env, String, Vec};

use crate::{
    access::{
        write_aggregated_pk_bytes, write_dapp_router_contract_id, write_master_contract_id,
        write_web_keys_bytes,
    },
    account_token::write_allowance_expiration,
    bls_account_auth::{write_dst_bytes, write_nonce},
    error::ContractError,
};

pub fn init_constructor(
    env: Env,
    bls_pubkeys: Vec<BytesN<96>>,
    platform: String,
    social_username: String,
    web_pubkey: BytesN<77>,
    master_contract_id: Address,
    dapp_router_contract_id: Address,
) -> Result<(), ContractError> {
    write_aggregated_pk_bytes(&env, bls_pubkeys);
    write_web_keys_bytes(&env, platform, social_username, web_pubkey);
    write_dst_bytes(&env);
    write_master_contract_id(&env, &master_contract_id);
    write_dapp_router_contract_id(&env, &dapp_router_contract_id);
    write_allowance_expiration(&env, 17000);
    write_nonce(&env);
    Ok(())
}
