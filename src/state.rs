use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub owner: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CanvasState {
  pub colors: Vec<u32>
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PixelData {
    pub owner: String,
    pub price: u64,
    pub message: String,
    pub for_sale: bool,
}

pub const ROW_PIXELS: u32 = 5;
pub const COL_PIXELS: u32 = 5; 

pub const STATE: Item<State> = Item::new("state");
pub const COLORS: Item<CanvasState> = Item::new("colors");
pub const PIXELS: Map<String, PixelData> = Map::new("pixels");