use soroban_sdk::{contracttype, Address, Vec};
use crate::storage::types::status::CampaignStatus;


#[derive(Clone)]
#[contracttype]
pub struct Campaign {
    pub goal: i128,
    pub min_donation: i128,
    pub total_raised: i128,
    pub supporters: u32,
    pub status: CampaignStatus,
    pub contributors: Vec<Address>,  // Lista de contribuyentes
}
