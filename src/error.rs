use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    /// Error that indicates the contract was already initialized
    AlreadyInitialized = 1,
    /// Error that indicates the contract is already accepted, finished or rejected for the creator
    AlreadyInProgress = 2,
    /// Error that indicates if the contract isn't active
    ContractNotActive = 3,
    /// Error that indicates if the contract doesn't have submitted assets
    AssetsNotFound = 4,
    /// Error that indicates a payment can't be executed because there are no approved assets
    NoApprovedAssets = 5,
    /// Error that indicates the contract wasn't already initialized
    NotInitialized = 6,
}