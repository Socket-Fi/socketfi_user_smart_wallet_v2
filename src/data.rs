use soroban_sdk::{contracttype, Address};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    UserAccountId,
    MasterContractId,
    DappRouterId,
    BlsPublicKey,
    WebKey,
    Allowance(Address, Address),
    PrimarySocialProfile,
    AllowanceExpiration,
    SmartAllowance(Address),
    Balance(Address),
    TokenList,
    Nonce,
    Dst,
    TransactionNonce,
    SoroswapContract,
    PairContract,
}
