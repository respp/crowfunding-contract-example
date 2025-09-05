use soroban_sdk::{contract, contractimpl, Env, Address};

use crate::{
    methods::{
        add_campaign::add_campaign,
        cancel_campaign::cancel_campaign,
        contribute::contribute,
        get_campaign::get_campaign,
        initialize::initialize,
        refund::refund,
        withdraw::withdraw
    },
    storage::{
        structs::campaign::Campaign,
        types::error::Error
    },
};

#[contract]
pub struct CrowdfundingContract;

#[contractimpl]
impl CrowdfundingContract {
    pub fn __constructor(env: Env, admin: Address, token: Address) -> Result<(), Error> {
        initialize(&env, admin, token)
    }

    pub fn create_campaign(env: Env, creator: Address, goal: i128, min_donation: i128) -> Result<(), Error> {
        add_campaign(&env, creator, goal, min_donation)
    }

    pub fn get_campaign(env: Env, campaign_address: Address) -> Result<Campaign, Error> {
        get_campaign(&env, &campaign_address)
    }

    pub fn contribute(env: Env, contributor: Address, campaign_address: Address, amount: i128) -> Result<(), Error> {
        contribute(&env, contributor, campaign_address, amount)
    }

    pub fn withdraw(env: Env, creator: Address) -> Result<(), Error> {
        withdraw(&env, creator)
    }

    pub fn refund(env: Env, contributor: Address, campaign_address: Address) -> Result<(), Error> {
        refund(&env, contributor, campaign_address)
    }

    pub fn cancel_campaign(env: Env, creator: Address) -> Result<(), Error> {
        cancel_campaign(&env, creator)
    }
}