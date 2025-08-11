use soroban_sdk::{Address, Env, Symbol};


pub(crate) fn add_contribute(env: &Env, contributor: &Address, campaign_address: &Address, amount: &i128) {
    let topics = (Symbol::new(env, "add_contribute"), contributor);
    let data = (campaign_address, amount);
    env.events().publish(topics, data);
}
