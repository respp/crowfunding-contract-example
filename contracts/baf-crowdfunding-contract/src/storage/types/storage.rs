use soroban_sdk::{contracttype, Address};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    Token,
    Campaign(Address),
    Contribution(Address, Address), // (campaign_address, contributor)
}