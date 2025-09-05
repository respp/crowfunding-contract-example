use soroban_sdk::{Address, Env, Vec};

use super::types::{error::Error, storage::DataKey};

pub(crate) fn has_contribution(env: &Env, campaign_address: &Address, contributor: &Address) -> bool {
    let key = DataKey::Contribution(campaign_address.clone(), contributor.clone());

    env.storage().instance().has(&key)
}

pub(crate) fn set_contribution(env: &Env, campaign_address: &Address, contributor: &Address, amount: i128) {
    let key = DataKey::Contribution(campaign_address.clone(), contributor.clone());

    env.storage().instance().set(&key, &amount);
}

pub(crate) fn get_contribution(env: &Env, campaign_address: &Address, contributor: &Address) -> Result<i128, Error> {
    let key = DataKey::Contribution(campaign_address.clone(), contributor.clone());

    env.storage().instance().get(&key).ok_or(Error::ContributionNotFound)
}

pub(crate) fn remove_contribution(env: &Env, campaign_address: &Address, contributor: &Address) {
    let key = DataKey::Contribution(campaign_address.clone(), contributor.clone());

    env.storage().instance().remove(&key);
}

// Función para obtener todos los contribuyentes de una campaña
pub(crate) fn get_all_contributors(env: &Env, campaign_address: &Address) -> Result<Vec<Address>, Error> {
    use crate::storage::campaign::get_campaign;
    
    // Obtener la campaña que contiene la lista de contribuyentes
    let campaign = get_campaign(env, campaign_address)?;
    
    // Retornar la lista de contribuyentes
    Ok(campaign.contributors)
}