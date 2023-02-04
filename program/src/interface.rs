use solana_program::pubkey::Pubkey;

pub enum Feature {
    MasterEdition,
    Royalties,
    Subscription,
}

pub trait MasterEdition {
    fn supply(&self) -> u64;
    fn max_supply(&self) -> Option<u64>;
}

pub trait Royalties {
    fn creators(&self) -> Vec<Pubkey>;
}

pub trait Subscription {
    fn price(&self) -> u64;
    fn period(&self) -> u64;
}

pub trait Metadata {
    fn features(&self) -> &[Feature];

    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn uri(&self) -> &str;
    fn master_edition(&self) -> Option<&dyn MasterEdition>;
    fn royalties(&self) -> Option<&dyn Royalties>;
    fn subscription(&self) -> Option<&dyn Subscription>;
}
