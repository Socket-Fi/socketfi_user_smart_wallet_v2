use soroban_sdk::{
    auth::{ContractContext, InvokerContractAuthEntry, SubContractInvocation},
    vec, Address, BytesN, Env, IntoVal, Symbol, Vec,
};

soroban_sdk::contractimport!(
    file = "../socketfi_smart_account_dapp_adapter/target/wasm32-unknown-unknown/release/socketfi_dapp_adapter.wasm"
);

pub fn deep_auth_soroswap(
    env: Env,
    soroswap_id: Address,
    pair_id: Address,
    amount_in: i128,
    amount_out_min: i128,
    deadline: u64,
    path: Vec<Address>,
    to: Address,
) {
    env.authorize_as_current_contract(vec![
        &env,
        InvokerContractAuthEntry::Contract(SubContractInvocation {
            context: ContractContext {
                contract: soroswap_id,
                fn_name: Symbol::new(&env, "swap_exact_tokens_for_tokens"),
                args: (
                    amount_in.clone(),
                    amount_out_min.clone(),
                    path.clone(),
                    to.clone(),
                    deadline.clone(),
                )
                    .into_val(&env),
            },
            sub_invocations: vec![
                &env,
                InvokerContractAuthEntry::Contract(SubContractInvocation {
                    context: ContractContext {
                        contract: path.get_unchecked(0),
                        fn_name: Symbol::new(&env, "transfer"),
                        args: (to.clone(), pair_id, amount_in.clone()).into_val(&env),
                    },
                    sub_invocations: vec![&env], // No further nested calls needed
                }),
            ],
        }),
    ]);
}

pub fn deep_auth_aqua_amm(
    env: Env,
    dex_router_id: Address,
    to: Address,
    swaps_chain: Vec<(Vec<Address>, BytesN<32>, Address)>,
    token_in: Address,
    in_amount: u128,
    out_min: u128,
) {
    env.authorize_as_current_contract(vec![
        &env,
        InvokerContractAuthEntry::Contract(SubContractInvocation {
            context: ContractContext {
                contract: dex_router_id.clone(),
                fn_name: Symbol::new(&env, "swap_chained"),
                args: (
                    to.clone(),
                    swaps_chain.clone(),
                    token_in.clone(),
                    in_amount.clone(),
                    out_min.clone(),
                )
                    .into_val(&env),
            },
            sub_invocations: vec![
                &env,
                InvokerContractAuthEntry::Contract(SubContractInvocation {
                    context: ContractContext {
                        contract: token_in.clone(),
                        fn_name: Symbol::new(&env, "transfer"),
                        args: (to.clone(), dex_router_id.clone(), in_amount.clone() as i128)
                            .into_val(&env),
                    },
                    sub_invocations: vec![&env], // No further nested calls needed
                }),
            ],
        }),
    ]);
}
