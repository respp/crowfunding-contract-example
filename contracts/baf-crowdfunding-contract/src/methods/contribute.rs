use soroban_sdk::{Address, Env};
use crate::{
    events,
    methods::token::token_transfer,
    storage::{
        campaign::{get_campaign, has_campaign, set_campaign}, contribution::set_contribution, types::error::Error
    }
};

pub fn contribute(env: &Env, contributor: Address, campaign_address: Address, amount: i128) -> Result<(), Error> {
    contributor.require_auth();

    if amount < 0 {
        return Err(Error::AmountMustBePositive);
    }

    if !has_campaign(env, &campaign_address) {
        return Err(Error::CampaignNotFound);
    }

    let mut campaign = get_campaign(env, &campaign_address)?;

    if campaign.min_donation > amount {
        return Err(Error::ContributionBelowMinimum);
    }

    if campaign.total_raised + amount > campaign.goal {
        return Err(Error::CampaignGoalExceeded);
    }

    token_transfer(&env, &contributor, &env.current_contract_address(), &amount)?;

    campaign.total_raised += amount;
    campaign.supporters += 1;
    
    // Agregar contribuyente a la lista si es la primera vez que contribuye
    if !campaign.contributors.contains(&contributor) {
        campaign.contributors.push_back(contributor.clone());
    }
        
    set_campaign(env, &campaign_address, &campaign);
    set_contribution(env, &campaign_address, &contributor, amount);
    events::contribute::add_contribute(&env, &contributor, &campaign_address, &amount);

    Ok(())
}