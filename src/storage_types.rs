//! Module StorageTypes
//!
//! Module that defines the set of keys that can be used to access and store data within the contract.
use soroban_sdk::contracttype;

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    /// Stores all the metadata of the contract as a `PaymentContractInfo` struct
    PaymentContractInfo,
    /// To store the partner that is requested to accept the contract
    AuthorizedPartner,
    /// To store the date that the contract was accepted by the creator
    DateOfAcceptance,
    /// To store the possible current contract states available in `ContractState` enum
    ContractState,
    /// To store the creator submitted assets as `Map<Bytes, Asset>`
    CreatorAssets,
    /// To store the fee that Mixip collected from a contract
    FeeProfit,
}

#[contracttype]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ContractState {
    Active,
    Rejected,
    Finished,
}