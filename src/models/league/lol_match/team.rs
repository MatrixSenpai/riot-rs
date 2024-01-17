use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamDto {
    pub bans: Vec<BanDto>,
    pub objectives: ObjectivesDto,
    pub team_id: i32,
    pub win: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BanDto {
    pub champion_id: i32,
    pub pick_turn: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectivesDto {
    pub baron: ObjectiveDto,
    pub champion: ObjectiveDto,
    pub dragon: ObjectiveDto,
    pub inhibitor: ObjectiveDto,
    pub rift_herald: ObjectiveDto,
    pub tower: ObjectiveDto,
    pub horde: Option<ObjectiveDto>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectiveDto {
    pub first: bool,
    pub kills: i32,
}