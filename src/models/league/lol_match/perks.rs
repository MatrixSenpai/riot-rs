use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PerksDto {
    stat_perks: PerkStatsDto,
    styles: Vec<PerkStyleDto>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PerkStatsDto {
    pub defense: i32,
    pub flex: i32,
    pub offense: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PerkStyleDto {
    pub description: String,
    pub selections: Vec<PerkStyleSelectionDto>,
    pub style: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PerkStyleSelectionDto {
    pub perk: i32,
    pub var1: i32,
    pub var2: i32,
    pub var3: i32,
}