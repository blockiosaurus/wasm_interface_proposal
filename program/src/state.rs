use rkyv::{Archive, Deserialize, Serialize};
use solana_program::wasm_bindgen;

#[wasm_bindgen]
#[repr(C)]
#[derive(Clone, Archive, Serialize, Deserialize, Debug)]
pub struct AccountThingy {
    pub thing: u64,
}

#[wasm_bindgen]
impl AccountThingy {
    pub fn new() -> Self {
        Self { thing: 0 }
    }

    pub fn set_thing(&mut self, thing: u64) {
        self.thing = thing;
    }

    pub fn get_thing(&self) -> u64 {
        self.thing
    }
}
