use soroban_sdk::{Address, Env};

use crate::{
    events,
    methods::token::token_transfer, storage::{
        campaign::{get_campaign, set_campaign}, contribution::{
            get_contribution, has_contribution, remove_contribution
        }, types::error::Error
    }
};

pub fn refund(env: &Env, contributor: Address, campaign_address: Address) -> Result<(), Error> {
    contributor.require_auth();

    let mut campaign = get_campaign(env, &campaign_address)?;

    if !has_contribution(env, &campaign_address, &contributor) {
        return Err(Error::ContributionNotFound);
    }

    let amount = get_contribution(env, &campaign_address, &contributor);
    token_transfer(&env, &env.current_contract_address(), &contributor, &amount)?;

    campaign.total_raised -= amount;
    campaign.supporters -= 1;

    remove_contribution(env, &campaign_address, &contributor);
    set_campaign(env, &campaign_address, &campaign);
    events::refund::refund(&env, &contributor, &campaign_address, &amount);

    Ok(())
}