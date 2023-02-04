use crate::interface::{self, Feature, Metadata};
use rkyv::{Archive, Deserialize, Serialize};
use solana_program::pubkey::Pubkey;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Archive, Serialize, Deserialize, Debug, Default)]
pub struct MasterEdition {
    #[wasm_bindgen(getter_with_clone)]
    pub supply: u64,
    #[wasm_bindgen(getter_with_clone)]
    pub max_supply: Option<u64>,
    /// A field that's specific to our implementation.
    #[wasm_bindgen(getter_with_clone)]
    pub artist_name: String,
}

impl interface::MasterEdition for MasterEdition {
    fn supply(&self) -> u64 {
        self.supply
    }

    fn max_supply(&self) -> Option<u64> {
        self.max_supply
    }
}

#[wasm_bindgen]
impl MasterEdition {
    #[wasm_bindgen(constructor)]
    pub fn new(supply: u64, max_supply: Option<u64>, artist_name: String) -> Self {
        Self {
            supply,
            max_supply,
            artist_name,
        }
    }

    #[wasm_bindgen]
    pub fn get_supply(master_edition: &MasterEdition) -> u64 {
        interface::MasterEdition::supply(master_edition)
    }

    #[wasm_bindgen]
    pub fn get_master_edition(master_edition: &MasterEdition) -> String {
        format!("{:#?}", master_edition)
    }
}

#[wasm_bindgen]
#[derive(Clone, Archive, Serialize, Deserialize, Debug, Default)]
pub struct Royalties {
    #[wasm_bindgen(skip)]
    pub creators: Vec<[u8; 32]>,
}

impl interface::Royalties for Royalties {
    fn creators(&self) -> Vec<Pubkey> {
        self.creators.iter().map(|c| Pubkey::new(c)).collect()
    }
}

#[wasm_bindgen]
#[derive(Clone, Archive, Serialize, Deserialize, Debug)]
pub struct MetadataAccount {
    #[wasm_bindgen(getter_with_clone)]
    pub name: String,
    #[wasm_bindgen(getter_with_clone)]
    pub description: String,
    #[wasm_bindgen(getter_with_clone)]
    pub uri: String,
    #[wasm_bindgen(getter_with_clone)]
    pub master_edition: MasterEdition,
    #[wasm_bindgen(getter_with_clone)]
    pub royalties: Royalties,
}

// #[wasm_bindgen]
impl Metadata for MetadataAccount {
    fn features(&self) -> &[interface::Feature] {
        &[Feature::MasterEdition, Feature::Royalties]
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn uri(&self) -> &str {
        &self.uri
    }

    fn master_edition(&self) -> Option<&dyn interface::MasterEdition> {
        Some(&self.master_edition)
    }

    fn royalties(&self) -> Option<&dyn interface::Royalties> {
        Some(&self.royalties)
    }

    fn subscription(&self) -> Option<&dyn interface::Subscription> {
        None
    }
}
