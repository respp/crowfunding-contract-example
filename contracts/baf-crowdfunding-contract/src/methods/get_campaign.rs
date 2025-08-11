use soroban_sdk::{Address, Env};

use crate::storage::{
    campaign::get_campaign as read_campaign, structs::campaign::Campaign, types::error::Error
};

pub fn get_campaign(env: &Env, campaign_address: &Address) ->  Result<Campaign, Error> {
    let campaign = read_campaign(env, &campaign_address)?;
    Ok(campaign)
}