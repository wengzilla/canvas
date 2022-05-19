use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::state::PixelData;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    BuyPixel {x: u32, y: u32, color: u32, price: u64, for_sale: bool, message: String},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetPixel returns the PixelData associated with a pixel
    GetPixel {x: u32, y: u32},
    GetColors {},
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PixelResponse {
    pub x: u32,
    pub y: u32,
    pub color: u32,
    pub pixel_data: PixelData,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ColorsResponse {
    pub colors: Vec<u32>
}

