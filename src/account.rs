use soroban_sdk::{contract, contractimpl, token, Address, BytesN, Env, Map, String, Vec};

use crate::{
    access::{
        is_initialized, read_dapp_router_contract_id, read_master_contract_id, read_user_account,
        read_web_keys_bytes, write_aggregated_pk_bytes, write_dapp_router_contract_id,
        write_master_contract_id, write_user_account, write_web_keys_bytes,
    },
    account_token::{
        read_allowance, read_allowance_expiration, read_balance, send_token, spend_token,
        take_token, write_allowance_expiration, write_approve,
    },
    bls_account_auth::{owner_require_auth, read_nonce, write_dst_bytes, write_nonce},
    constructor::init_constructor,
    dap_adapter::{self, deep_auth_aqua_amm, deep_auth_soroswap},
    error::ContractError,
    token_list::{read_token_list, write_add_token, write_remove_token},
    types::{AllowanceDetails, TokenDetails, WebKeyDetails},
    user_account_trait::AccountTrait,
};

#[contract]
pub struct Account;

#[contractimpl]
impl AccountTrait for Account {
    //Account initialization, called when the account is created. Set public keys, master contract id and dapp router contract id
    ///Initialize Wallet
    fn init(
        env: Env,
        bls_pubkeys: Vec<BytesN<96>>,
        platform: String,
        social_username: String,
        web_pubkey: BytesN<77>,
        master_contract_id: Address,
        dapp_router_contract_id: Address,
    ) -> Result<(), ContractError> {
        if is_initialized(&env) {
            return Err(ContractError::AlreadyInitialized);
        }
        init_constructor(
            env,
            bls_pubkeys,
            platform,
            social_username,
            web_pubkey,
            master_contract_id,
            dapp_router_contract_id,
        )?;
        Ok(())
    }
    ///Set Allowance Expiration
    fn update_allowance_expiration(
        env: Env,
        expiration_ledger: u32,
        tx_signature: Option<BytesN<192>>,
    ) -> Result<(), ContractError> {
        owner_require_auth(env.clone(), tx_signature)?;

        write_allowance_expiration(&env, expiration_ledger);
        Ok(())
    }
    ///Set User's External Wallet
    fn set_user_account(
        env: Env,
        user_account_id: Address,
        tx_signature: Option<BytesN<192>>,
    ) -> Result<(), ContractError> {
        owner_require_auth(env.clone(), tx_signature)?;

        write_user_account(&env, &user_account_id);
        Ok(())
    }
    ///Update Master Wallet Contract
    fn update_master_contract(
        env: Env,
        master_contract_id: Address,
        tx_signature: Option<BytesN<192>>,
    ) -> Result<(), ContractError> {
        owner_require_auth(env.clone(), tx_signature)?;
        write_master_contract_id(&env, &master_contract_id);
        Ok(())
    }
    ///Update dApp Router Contract
    fn update_dapp_router_contract(
        env: Env,
        dapp_router_contract_id: Address,
        tx_signature: Option<BytesN<192>>,
    ) -> Result<(), ContractError> {
        owner_require_auth(env.clone(), tx_signature)?;
        write_dapp_router_contract_id(&env, &dapp_router_contract_id);
        Ok(())
    }
    ///Deposit Tokens
    fn deposit(
        e: Env,
        from: Address,
        token_id: Address,
        amount: i128,
    ) -> Result<(), ContractError> {
        from.require_auth();

        take_token(&e, &from, &token_id, amount);
        write_add_token(e, token_id);

        Ok(())
    }
    ///Withdraw Tokens
    fn withdraw(
        env: Env,
        to: Address,
        token_id: Address,
        amount: i128,
        tx_signature: Option<BytesN<192>>,
    ) -> Result<(), ContractError> {
        owner_require_auth(env.clone(), tx_signature)?;

        send_token(&env, &to, &token_id, amount);
        Ok(())
    }

    ///Swap on Soroswap
    fn swap_tokens_soroswap(
        env: Env,
        amount_in: i128,
        amount_out_min: i128,
        path: Vec<Address>,
        deadline: u64,
        tx_signature: Option<BytesN<192>>,
    ) -> Result<(), ContractError> {
        owner_require_auth(env.clone(), tx_signature)?;

        let to = env.current_contract_address();

        let dapp_adapter_id = read_master_contract_id(&env).unwrap();
        let dapp_adapter_contract = dap_adapter::Client::new(&env, &dapp_adapter_id);

        let soroswap_id = dapp_adapter_contract.get_soroswap_id();

        let pair_id = dapp_adapter_contract
            .get_pair_router_soroswap(&path.get_unchecked(0), &path.get_unchecked(1));

        deep_auth_soroswap(
            env.clone(),
            soroswap_id,
            pair_id,
            amount_in.clone(),
            amount_out_min.clone(),
            deadline.clone(),
            path.clone(),
            to.clone(),
        );

        dapp_adapter_contract.swap_exact_soroswap(
            &amount_in,
            &amount_out_min,
            &path,
            &to,
            &deadline,
        );

        write_add_token(env, path.get_unchecked(1));

        Ok(())
    }

    ///Swap on Aqua AMM
    fn swap_tokens_aqua(
        env: Env,
        swaps_chain: Vec<(Vec<Address>, BytesN<32>, Address)>,
        token_in: Address,
        in_amount: u128,
        out_min: u128,
        tx_signature: Option<BytesN<192>>,
    ) -> Result<(), ContractError> {
        owner_require_auth(env.clone(), tx_signature)?;

        let to = env.current_contract_address();

        let dapp_adapter_id = read_master_contract_id(&env).unwrap();
        let dapp_adapter_contract = dap_adapter::Client::new(&env, &dapp_adapter_id);

        let aqua_router_id = dapp_adapter_contract.get_aqua_amm_router_id();

        deep_auth_aqua_amm(
            env,
            aqua_router_id.clone(),
            to.clone(),
            swaps_chain.clone(),
            token_in.clone(),
            in_amount.clone(),
            out_min.clone(),
        );

        dapp_adapter_contract.swap_chain_aqua(&to, &swaps_chain, &token_in, &in_amount, &out_min);

        Ok(())
    }

    ///Approve Spender Allowance
    fn approve(
        env: Env,
        token_id: Address,
        spender: Address,
        amount: i128,
        tx_signature: Option<BytesN<192>>,
    ) -> Result<(), ContractError> {
        owner_require_auth(env.clone(), tx_signature)?;

        write_approve(&env, &token_id, &spender, &amount);
        Ok(())
    }

    ///Spend
    fn spend(
        env: Env,
        token_id: Address,
        spender: Address,
        amount: i128,
        to: Address,
    ) -> Result<(), ContractError> {
        spender.require_auth();
        spend_token(&env, &spender, &token_id, amount, &to);

        Ok(())
    }

    ///Get Token Details
    fn get_token_list(env: Env) -> Result<Map<Address, TokenDetails>, ContractError> {
        let mut token_details: Map<Address, TokenDetails> = Map::new(&env);
        let token_list = read_token_list(env.clone());

        for i in 0..token_list.len() {
            let token_id = token_list.get_unchecked(i);
            let balance = read_balance(&env, &token_id);
            if balance > 0 {
                let details = TokenDetails {
                    symbol: token::Client::new(&env, &token_id).symbol(),
                    balance: balance,
                };
                token_details.set(token_id, details)
            }
        }

        Ok(token_details)
    }

    ///Get Spender Allowances
    fn get_spender_allowances(
        env: Env,
        spender: Address,
    ) -> Result<Map<Address, AllowanceDetails>, ContractError> {
        let mut allowance_details: Map<Address, AllowanceDetails> = Map::new(&env);
        let token_list = read_token_list(env.clone());

        for i in 0..token_list.len() {
            let token_id = token_list.get_unchecked(i);
            let allowance = read_allowance(&env, &token_id, &spender);
            if allowance > 0 {
                let details = AllowanceDetails {
                    symbol: token::Client::new(&env, &token_id).symbol(),
                    allowance: allowance,
                };
                allowance_details.set(token_id, details)
            }
        }

        Ok(allowance_details)
    }

    ///Get Passkey
    fn get_web_keys(env: Env) -> WebKeyDetails {
        read_web_keys_bytes(&env)
    }

    ///Get Spender Allowance
    fn get_allowance(env: Env, token_id: Address, spender: Address) -> i128 {
        read_allowance(&env, &token_id, &spender)
    }

    ///Get Current Nonce
    fn get_nonce(env: Env) -> BytesN<32> {
        read_nonce(&env)
    }

    ///Get Token Balance
    fn get_balance(env: Env, token_id: Address) -> i128 {
        read_balance(&env, &token_id)
    }

    ///Upgrade Contract
    fn upgrade(
        e: Env,
        new_wasm_hash: BytesN<32>,
        tx_signature: Option<BytesN<192>>,
    ) -> Result<(), ContractError> {
        owner_require_auth(e.clone(), tx_signature)?;
        e.deployer().update_current_contract_wasm(new_wasm_hash);
        Ok(())
    }
}
