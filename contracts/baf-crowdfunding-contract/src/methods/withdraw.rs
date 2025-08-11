use soroban_sdk::{Address, Env};

use crate::{
    events,
    methods::token::token_transfer,
    storage::{
        campaign::{get_campaign, remove_campaign},
        types::error::Error
    }
};

pub fn withdraw(env: &Env, creator: Address) -> Result<(), Error> {
    creator.require_auth();

    let campaign = get_campaign(env, &creator)?;

    if campaign.total_raised != campaign.goal {
        return Err(Error::CampaignGoalNotReached);
    }

    token_transfer(
        &env,
        &env.current_contract_address(),
        &creator,
        &campaign.total_raised
    )?;

    remove_campaign(env, &creator);
    events::campaign::withdraw(&env, &creator, campaign.total_raised);
    
    Ok(())
}