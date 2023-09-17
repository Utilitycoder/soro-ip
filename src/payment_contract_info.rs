//! Module PaymentContractInfo
//!
//! Module responsible of managing `PaymentContractInfo` and defining its corresponding struct.
use crate::storage_types::DataKey;
use soroban_sdk::{contracttype, Address, Bytes, BytesN, Env};

const CONTRACT_INFO_KEY: DataKey = DataKey::PaymentContractInfo;
const AUTH_PARTNER_KEY: DataKey = DataKey::AuthorizedPartner;

#[derive(Clone, PartialEq, Eq, Debug)]
#[contracttype]
///Struct that stores the necessary information for the contract
pub struct PaymentContractInfo {
    /// The contract manager of the contract
    pub contract_manager: ContractManager,
    /// The identification of the company in an off chain storage
    pub company_id: Bytes,
    /// The identification of the project in an off chain storage
    pub project_id: Bytes,
    /// The identification of the contract in an off chain storage
    pub contract_name: Bytes,
    /// The way the payment will be executed (only native (xlm) for now)
    pub payment_method: PaymentMethod,
    /// The payment amount for each approved asset
    pub asset_payment_amount: i128,
    /// Contract creation date
    pub creation_date: u64,
    /// The date agreed upon for starting the execution of the contract
    pub start_date: u64,
    /// The last day on which assets could be uploaded
    pub deadline: u64,
    /// Conditions of the contract
    pub scope_of_work: Bytes,
    pub rights_royalties: Bytes,
    pub payment_time: u64,
    pub contract_type: ContractType,
}

#[contracttype]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum PaymentMethod {
    /// XLM
    Native(BytesN<32>),
}

#[contracttype]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ContractType {
    FixedPrice,
    Milestones,
    Licensing,
}

#[derive(Clone, PartialEq, Eq, Debug)]
#[contracttype]
pub struct ContractManager {
    pub address: Address,
    pub name: Bytes,
    pub job_position: Bytes,
    pub physical_address: Bytes,
}

pub(crate) fn has_contact_info(env: &Env) -> bool {
    env.storage().has(&CONTRACT_INFO_KEY)
}

pub(crate) fn write_contract_info(env: &Env, contract_info: &PaymentContractInfo) {
    env.storage().set(&CONTRACT_INFO_KEY, contract_info);
}

pub(crate) fn get_contract_info(env: &Env) -> PaymentContractInfo {
    env.storage().get_unchecked(&CONTRACT_INFO_KEY).unwrap()
}

pub(crate) fn write_creator(env: &Env, partner: &Address) {
    env.storage().set(&AUTH_PARTNER_KEY, partner)
}

pub(crate) fn get_contract_manager_address(env: &Env) -> Address {
    let contract_info: PaymentContractInfo =
        env.storage().get_unchecked(&CONTRACT_INFO_KEY).unwrap();
    contract_info.contract_manager.address
}

pub(crate) fn get_creator(env: &Env) -> Address {
    env.storage().get_unchecked(&AUTH_PARTNER_KEY).unwrap()
}

pub(crate) fn get_payment_date(env: &Env) -> u64 {
    let contract_info: PaymentContractInfo =
        env.storage().get_unchecked(&CONTRACT_INFO_KEY).unwrap();
    contract_info.deadline + contract_info.payment_time
}

pub(crate) fn get_payment_time(env: &Env) -> u64 {
    let contract_info: PaymentContractInfo =
        env.storage().get_unchecked(&CONTRACT_INFO_KEY).unwrap();
    contract_info.payment_time
}

pub(crate) fn get_payment_method(env: &Env) -> PaymentMethod {
    let contract_info: PaymentContractInfo =
        env.storage().get_unchecked(&CONTRACT_INFO_KEY).unwrap();
    contract_info.payment_method
}

pub(crate) fn get_asset_payment_amount(env: &Env) -> i128 {
    let contract_info: PaymentContractInfo =
        env.storage().get_unchecked(&CONTRACT_INFO_KEY).unwrap();
    contract_info.asset_payment_amount
}