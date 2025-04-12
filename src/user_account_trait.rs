use soroban_sdk::{Address, BytesN, Env, Map, String, Vec};

use crate::{
    error::ContractError,
    types::{AllowanceDetails, TokenDetails, WebKeyDetails},
};

pub trait AccountTrait {
    fn init(
        env: Env,
        bls_pubkeys: Vec<BytesN<96>>,
        platform: String,
        social_username: String,
        web_pubkey: BytesN<77>,
        master_contract_id: Address,
        dapp_router_contract_id: Address,
    ) -> Result<(), ContractError>;

    fn update_allowance_expiration(
        env: Env,
        expiration_ledger: u32,
        tx_signature: Option<BytesN<192>>,
    ) -> Result<(), ContractError>;

    fn set_user_account(
        env: Env,
        user_account_id: Address,
        tx_signature: Option<BytesN<192>>,
    ) -> Result<(), ContractError>;
    fn update_master_contract(
        env: Env,
        master_contract_id: Address,
        tx_signature: Option<BytesN<192>>,
    ) -> Result<(), ContractError>;
    fn update_dapp_router_contract(
        env: Env,
        dapp_router_contract_id: Address,
        tx_signature: Option<BytesN<192>>,
    ) -> Result<(), ContractError>;
    fn deposit(e: Env, from: Address, token_id: Address, amount: i128)
        -> Result<(), ContractError>;
    fn withdraw(
        env: Env,
        to: Address,
        token_id: Address,
        amount: i128,
        tx_signature: Option<BytesN<192>>,
    ) -> Result<(), ContractError>;

    fn swap_tokens_soroswap(
        env: Env,
        amount_in: i128,
        amount_out_min: i128,
        path: Vec<Address>,
        deadline: u64,
        tx_signature: Option<BytesN<192>>,
    ) -> Result<(), ContractError>;
    fn swap_tokens_aqua(
        env: Env,
        swaps_chain: Vec<(Vec<Address>, BytesN<32>, Address)>,
        token_in: Address,
        in_amount: u128,
        out_min: u128,
        tx_signature: Option<BytesN<192>>,
    ) -> Result<(), ContractError>;
    // fn reset_nonce(env: Env);
    fn approve(
        env: Env,
        token_id: Address,
        spender: Address,
        amount: i128,
        tx_signature: Option<BytesN<192>>,
    ) -> Result<(), ContractError>;
    fn spend(
        env: Env,
        token_id: Address,
        spender: Address,
        amount: i128,
        to: Address,
    ) -> Result<(), ContractError>;
    fn get_token_list(env: Env) -> Result<Map<Address, TokenDetails>, ContractError>;
    fn get_spender_allowances(
        env: Env,
        spender: Address,
    ) -> Result<Map<Address, AllowanceDetails>, ContractError>;
    fn get_web_keys(env: Env) -> WebKeyDetails;
    fn get_allowance(env: Env, token_id: Address, spender: Address) -> i128;
    fn get_nonce(env: Env) -> BytesN<32>;
    fn get_balance(env: Env, token_id: Address) -> i128;
    fn upgrade(
        e: Env,
        new_wasm_hash: BytesN<32>,
        tx_signature: Option<BytesN<192>>,
    ) -> Result<(), ContractError>;
}
