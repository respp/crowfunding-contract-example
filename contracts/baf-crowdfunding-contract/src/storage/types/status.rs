use soroban_sdk::contracttype;

#[derive(Clone)]
#[contracttype]
pub enum CampaignStatus {
    Active,
    Completed,
    Failed,
    Canceled,
}