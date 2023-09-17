use crate::{
    error::ContractError, payment::execute_payment, payment_contract_info::get_payment_time,
    storage_types::DataKey,
};

use soroban_sdk::{contracttype, map, panic_with_error, Bytes, Env, Map, Vec};

const CREATOR_ASSETS_KEY: DataKey = DataKey::CreatorAssets;

#[derive(Clone, PartialEq, Eq, Debug)]
#[contracttype]
pub struct Asset {
    pub asset_url: Bytes,
    pub submission_date: u64,
    pub state: AssetState,
}

#[derive(Clone, PartialEq, Eq, Debug)]
#[contracttype]
pub enum AssetState {
    InReview,
    Approved,
    Rejected,
    Paid,
}

impl Asset {
    fn new(asset_url: Bytes, submission_date: u64) -> Self {
        Asset {
            asset_url,
            submission_date,
            state: AssetState::InReview,
        }
    }
}

pub(crate) fn store_assets(env: &Env, asset_ids: Map<Bytes, Bytes>, submission_date: u64) {
    let mut assets: Map<Bytes, Asset> = map![env];
    for asset_url in asset_ids.iter() {
        let (id, url) = asset_url.unwrap();
        let asset = Asset::new(url, submission_date);
        assets.set(id, asset);
    }
    write_assets(env, &assets)
}

pub(crate) fn approve_asset(env: &Env, assets_ids: Vec<Bytes>, date: &u64) {
    check_if_has_assets(env);
    let mut assets: Map<Bytes, Asset> = env.storage().get_unchecked(&CREATOR_ASSETS_KEY).unwrap();
    let payment_time = get_payment_time(env);
    assets_ids
        .iter()
        .for_each(|asset_id| change_asset_state(asset_id.unwrap(), &mut assets));
    write_assets(env, &assets);
    if payment_time == 0 {
        execute_payment(env, date, &Option::None)
    }
}

pub(crate) fn read_assets(env: &Env) -> Map<Bytes, Asset> {
    env.storage().get(&CREATOR_ASSETS_KEY).unwrap().unwrap()
}

pub(crate) fn write_assets(env: &Env, assets: &Map<Bytes, Asset>) {
    env.storage().set(&CREATOR_ASSETS_KEY, assets)
}

pub(crate) fn check_if_has_assets(env: &Env) {
    if !env.storage().has(&CREATOR_ASSETS_KEY) {
        panic_with_error!(env, ContractError::AssetsNotFound);
    }
}

fn change_asset_state(asset_id: Bytes, assets: &mut Map<Bytes, Asset>) {
    if let Some(asset) = assets.get(asset_id.clone()) {
        let mut asset = asset.unwrap();
        asset.state = AssetState::Approved;
        assets.set(asset_id, asset)
    }
}