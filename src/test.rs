#![cfg(test)]

extern crate std;

use crate::{
    asset::{Asset, AssetState},
    payment_contract_info::{ContractManager, ContractType, PaymentContractInfo, PaymentMethod},
    storage_types::ContractState,
    PaymentContract, PaymentContractClient,
};
use soroban_sdk::{map, testutils::Address as _, Address, Bytes, BytesN, Env, IntoVal, Map};

mod token_contract {
    soroban_sdk::contractimport!(file = "soroban_token_spec.wasm");
}

fn create_and_init_token_contract(
    env: &Env,
    admin_id: &Address,
) -> (BytesN<32>, token_contract::Client) {
    let id = env.register_stellar_asset_contract(admin_id.clone());
    let token = token_contract::Client::new(env, &id);
    (id, token)
}

fn create_payment_contract(
    e: &Env,
    payment_contract_info: &PaymentContractInfo,
    creator_address: &Address,
) -> PaymentContractClient {
    let payment_contract =
        PaymentContractClient::new(e, &e.register_contract(None, PaymentContract {}));
    payment_contract.initialize(payment_contract_info, creator_address);
    payment_contract
}

struct PaymentContractTest {
    env: Env,
    payment_contract_info: PaymentContractInfo,
    creator_address: Address,
    assets: Map<Bytes, Bytes>,
    token_client: token_contract::Client,
}

impl PaymentContractTest {
    fn setup() -> Self {
        let env: Env = Default::default();
        let token_admin = Address::random(&env);
        let contract_manager_address = Address::random(&env);
        let creator_address = Address::random(&env);
        let company_id: Bytes = "ID-001".into_val(&env);
        let project_id: Bytes = "ID-001".into_val(&env);
        let contract_name: Bytes = "Test Contract Name".into_val(&env);
        let contract_manager: ContractManager = ContractManager {
            address: contract_manager_address.clone(),
            name: "John Doe".into_val(&env),
            job_position: "Product owner".into_val(&env),
            physical_address: "Some address".into_val(&env),
        };
        let (token_id, token_client) = create_and_init_token_contract(&env, &token_admin);
        token_client.mint(&token_admin, &contract_manager_address, &1000_i128);
        let payment_contract_info = PaymentContractInfo {
            contract_manager,
            company_id,
            project_id,
            contract_name,
            payment_method: PaymentMethod::Native(token_id),
            asset_payment_amount: 5,
            creation_date: 1681917160,
            deadline: 1684546903,
            payment_time: 0,
            contract_type: ContractType::Milestones,
            start_date: 1682003560,
            scope_of_work: "scope_of_work text".into_val(&env),
            rights_royalties: "rights_royalties text".into_val(&env),
        };
        let assets: Map<Bytes, Bytes> = map![
            &env,
            ("ASSET-ID-1".into_val(&env), "asset-1-url".into_val(&env)),
            ("ASSET-ID-2".into_val(&env), "asset-2-url".into_val(&env)),
        ];
        PaymentContractTest {
            env,
            payment_contract_info,
            creator_address,
            assets,
            token_client,
        }
    }
}

#[test]
fn test_successful_execution_of_wallet_capabilities_upon_approval() {
    let test = PaymentContractTest::setup();

    let payment_contract = create_payment_contract(
        &test.env,
        &test.payment_contract_info,
        &test.creator_address,
    );
    assert_eq!(
        payment_contract.get_payment_contract_info(),
        test.payment_contract_info
    );

    payment_contract.sign_contract(&1681977600);

    payment_contract.submit_asset(&test.assets, &1683158399);
    assert_eq!(payment_contract.get_submitted_assets().len(), 2);

    payment_contract.approve_asset(&test.assets.keys(), &1677953357);
    let asset: Asset = payment_contract
        .get_submitted_assets()
        .values()
        .get(0)
        .unwrap()
        .unwrap();

    assert_eq!(asset.state, AssetState::Paid);
    assert_eq!(payment_contract.get_fee_profit(), 0);
    assert_eq!(payment_contract.get_contract_state(), ContractState::Active);
    assert_eq!(test.token_client.balance(&test.creator_address), 10);
}

#[test]
fn test_successful_execution_of_wallet_capabilities_with_payment_time() {
    let test = PaymentContractTest::setup();
    let mut payment_contract_info = test.payment_contract_info.clone();
    payment_contract_info.payment_time = 2629743_u64;
    let contract_manager_address = test.payment_contract_info.contract_manager.address.clone();
    let payment_date = payment_contract_info.deadline + payment_contract_info.payment_time;
    let payment_contract =
        create_payment_contract(&test.env, &payment_contract_info, &test.creator_address);

    payment_contract.sign_contract(&1681977600);

    payment_contract.submit_asset(&test.assets, &1683158399);
    assert_eq!(payment_contract.get_submitted_assets().len(), 2);

    payment_contract.approve_asset(&test.assets.keys(), &1677953357);
    let mut asset: Asset = payment_contract
        .get_submitted_assets()
        .values()
        .get(0)
        .unwrap()
        .unwrap();

    assert_eq!(asset.state, AssetState::Approved);

    payment_contract.execute_payment(&payment_date, &Option::Some(contract_manager_address));
    asset = payment_contract
        .get_submitted_assets()
        .values()
        .get(0)
        .unwrap()
        .unwrap();

    assert_eq!(asset.state, AssetState::Paid);
    assert_eq!(payment_contract.get_fee_profit(), 0);
    assert_eq!(test.token_client.balance(&test.creator_address), 10);
}

#[test]
fn test_successful_execution_of_wallet_capabilities_on_prepayment() {
    let test = PaymentContractTest::setup();
    let mut payment_contract_info = test.payment_contract_info.clone();
    payment_contract_info.payment_time = 2629743_u64;
    let contract_manager_address = test.payment_contract_info.contract_manager.address.clone();
    let payment_date = payment_contract_info.deadline + 604800_u64;
    let payment_contract =
        create_payment_contract(&test.env, &payment_contract_info, &test.creator_address);

    payment_contract.sign_contract(&1681977600);
    payment_contract.submit_asset(&test.assets, &1683158399);

    payment_contract.approve_asset(&test.assets.keys(), &1677953357);
    let mut asset: Asset = payment_contract
        .get_submitted_assets()
        .values()
        .get(0)
        .unwrap()
        .unwrap();

    assert_eq!(asset.state, AssetState::Approved);

    payment_contract.execute_payment(&payment_date, &Option::Some(contract_manager_address));
    asset = payment_contract
        .get_submitted_assets()
        .values()
        .get(0)
        .unwrap()
        .unwrap();

    assert_eq!(asset.state, AssetState::Paid);
    assert_eq!(payment_contract.get_fee_profit(), 1);
    assert_eq!(test.token_client.balance(&test.creator_address), 9);
}

#[test]
#[should_panic(expected = "Status(ContractError(1))")]
fn test_initialize_an_already_initialized_payment_contract() {
    let test = PaymentContractTest::setup();
    let payment_contract = create_payment_contract(
        &test.env,
        &test.payment_contract_info,
        &test.creator_address,
    );
    payment_contract.initialize(&test.payment_contract_info, &test.creator_address);
}

#[test]
#[should_panic(expected = "Status(ContractError(2))")]
fn test_accepting_and_already_accepted_contract() {
    let test = PaymentContractTest::setup();

    let payment_contract = create_payment_contract(
        &test.env,
        &test.payment_contract_info,
        &test.creator_address,
    );

    payment_contract.sign_contract(&1681977600);
    payment_contract.sign_contract(&1681999200);
}

#[test]
#[should_panic(expected = "Status(ContractError(3))")]
fn test_submit_assets_when_contract_not_active() {
    let test = PaymentContractTest::setup();

    let payment_contract = create_payment_contract(
        &test.env,
        &test.payment_contract_info,
        &test.creator_address,
    );

    payment_contract.submit_asset(&test.assets, &1683158399);
}

#[test]
#[should_panic(expected = "Status(ContractError(4))")]
fn test_approve_assets_when_no_assets_in_contract() {
    let test = PaymentContractTest::setup();

    let payment_contract = create_payment_contract(
        &test.env,
        &test.payment_contract_info,
        &test.creator_address,
    );

    payment_contract.sign_contract(&1681977600);
    payment_contract.approve_asset(&test.assets.keys(), &1677953357);
}

#[test]
#[should_panic(expected = "Status(ContractError(5))")]
fn test_execute_payment_when_no_approved_assets() {
    let test = PaymentContractTest::setup();
    let mut payment_contract_info = test.payment_contract_info.clone();
    payment_contract_info.payment_time = 2629743_u64;
    let contract_manager_address = test.payment_contract_info.contract_manager.address.clone();
    let payment_date = payment_contract_info.deadline + payment_contract_info.payment_time;
    let payment_contract =
        create_payment_contract(&test.env, &payment_contract_info, &test.creator_address);

    payment_contract.sign_contract(&1681977600);
    payment_contract.submit_asset(&test.assets, &1683158399);
    payment_contract.execute_payment(&payment_date, &Option::Some(contract_manager_address));
}

#[test]
#[should_panic(expected = "Status(ContractError(3))")]
fn test_get_contract_state_when_contract_not_active() {
    let test = PaymentContractTest::setup();
    let payment_contract_info = test.payment_contract_info.clone();
    let payment_contract =
        create_payment_contract(&test.env, &payment_contract_info, &test.creator_address);

    payment_contract.get_contract_state();
}

#[test]
#[should_panic(expected = "Status(ContractError(4))")]
fn test_get_submitted_assets_when_no_submitted_assets() {
    let test = PaymentContractTest::setup();
    let payment_contract_info = test.payment_contract_info.clone();
    let payment_contract =
        create_payment_contract(&test.env, &payment_contract_info, &test.creator_address);

    payment_contract.sign_contract(&1681977600);
    payment_contract.get_submitted_assets();
}