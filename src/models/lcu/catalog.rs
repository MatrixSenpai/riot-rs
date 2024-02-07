use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemWithDetails {
    pub assets: ItemAssets,
    pub bundled_discount_prices: Vec<Price>,
    // unknown type
    // pub bundled_items
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemAssets {
    pub colors: Vec<String>,
    pub emblems: Vec<SkinEmblem>,
    pub icon_path: String,
    pub splash_path: String,
    pub tile_path: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SkinEmblem {
    pub emblem_path: SkinEmblemPath,
    pub emblem_position: SkinEmblemPosition,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SkinEmblemPath {
    pub large: String,
    pub small: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SkinEmblemPosition {
    pub horizontal: String,
    pub vertical: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Price {
    pub cost: i64,
    pub cost_type: String,
    pub currency: String,
    pub sale: RetailDiscount,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RetailDiscount {
    pub cost: i64,
    pub discount: f64,
    pub end_date: String,
    pub start_date: String,
}

pub struct Item {
    pub active: bool,
    pub description: String,
    pub image_path: String,
    pub inactive_date: i64,
    pub inventory_type: String,
    pub item_id: i32,
    pub item_instance_id: String,

}