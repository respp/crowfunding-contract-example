use soroban_sdk::{Address, Env};

use super::types::storage::DataKey;

pub(crate) fn has_contribution(env: &Env, campaign_address: &Address, contributor: &Address) -> bool {
    let key = DataKey::Contribution(campaign_address.clone(), contributor.clone());

    env.storage().instance().has(&key)
}

pub(crate) fn set_contribution(env: &Env, campaign_address: &Address, contributor: &Address, amount: i128) {
    let key = DataKey::Contribution(campaign_address.clone(), contributor.clone());

    env.storage().instance().set(&key, &amount);
}

pub(crate) fn get_contribution(env: &Env, campaign_address: &Address, contributor: &Address) -> i128 {
    let key = DataKey::Contribution(campaign_address.clone(), contributor.clone());

    env.storage().instance().get(&key).unwrap()
}

pub(crate) fn remove_contribution(env: &Env, campaign_address: &Address, contributor: &Address) {
    let key = DataKey::Contribution(campaign_address.clone(), contributor.clone());

    env.storage().instance().remove(&key);
}