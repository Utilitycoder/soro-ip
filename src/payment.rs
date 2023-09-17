mod token_contract {
    soroban_sdk::contractimport!(file = "soroban_token_spec.wasm");
}

use crate::{
    asset::{read_assets, write_assets, Asset, AssetState},
    error::ContractError,
    metadata::update_fee,
    payment_contract_info::{
        get_asset_payment_amount, get_contract_manager_address, get_creator, get_payment_date,
        get_payment_method, get_payment_time, PaymentMethod,
    },
};
use soroban_sdk::{panic_with_error, unwrap::UnwrapOptimized, vec, Address, Bytes, Env, Map, Vec};

pub(crate) fn execute_payment(env: &Env, date: &u64, prepayment_source: &Option<Address>) {
    let payment_date = get_payment_date(env);
    let payment_time = get_payment_time(env);
    let payment_method = get_payment_method(env);
    let contract_manager_address = get_contract_manager_address(env);
    let creator_address = get_creator(env);
    let asset_payment_amount = get_asset_payment_amount(env);
    let (payment_amount, assets_to_pay) = calculate_payment_amount(env, &asset_payment_amount);

    match payment_method {
        PaymentMethod::Native(contract_id) => {
            let client = token_contract::Client::new(env, &contract_id);
            if payment_date > *date && payment_time != 0 {
                prepayment_source.clone().unwrap().require_auth();
                execute_prepayment(
                    env,
                    &payment_amount,
                    &prepayment_source.clone().unwrap(),
                    &creator_address,
                    &client,
                )
            } else {
                contract_manager_address.require_auth();
                client.xfer(&contract_manager_address, &creator_address, &payment_amount);
            }
        }
    }
    set_assets_as_paid(env, assets_to_pay);
}

fn execute_prepayment(
    env: &Env,
    payment_amount: &i128,
    prepayment_source: &Address,
    creator_address: &Address,
    client: &token_contract::Client,
) {
    let payment_amount = *payment_amount as f64;
    let fee = payment_amount * 0.1_f64;
    let prepayment_amount = payment_amount - fee;
    update_fee(env, &(fee as i128));
    client.xfer(
        prepayment_source,
        creator_address,
        &(prepayment_amount as i128),
    )
}

fn calculate_payment_amount(env: &Env, asset_payment_amount: &i128) -> (i128, Vec<Bytes>) {
    let submitted_assets: Map<Bytes, Asset> = read_assets(env);
    let mut asset_ids: Vec<Bytes> = vec![env];
    for asset in submitted_assets.iter() {
        let (id, data) = asset.unwrap();
        if data.state == AssetState::Approved {
            asset_ids.push_front(id)
        }
    }
    if asset_ids.is_empty() {
        panic_with_error!(env, ContractError::NoApprovedAssets);
    }
    let total_payment_amount: i128 = asset_payment_amount
        .checked_mul(asset_ids.len() as i128)
        .unwrap_optimized();
    (total_payment_amount, asset_ids)
}

fn set_assets_as_paid(env: &Env, assets_to_pay: Vec<Bytes>) {
    let mut submitted_assets: Map<Bytes, Asset> = read_assets(env);
    assets_to_pay.iter().for_each(|asset_id| {
        let id = asset_id.unwrap();
        let mut asset = submitted_assets.get_unchecked(id.clone()).unwrap();
        asset.state = AssetState::Paid;
        submitted_assets.set(id, asset);
    });
    write_assets(env, &submitted_assets);
}