use std::collections::HashMap;

pub struct ChampionQueueStatResponse {
    pub champion_id: i32,
    pub position: SummonersRiftPosition,
    pub queue_type: QueueType,
    pub rank_tier: RankedTier,
}

pub enum SummonersRiftPosition {
    All,
    Unknown,
    Top,
    Jungle,
    Mid,
    Bottom,
    Support,
}

pub enum QueueType {
    Draft5,
    Rank5Flex,
    Rank5Solo,
    Blind5,
    Aram,
    Blind3,
    Rank3Flex,
    Other,
}

pub enum RankedTier {
    All,
    Unranked,
    Iron,
    Bronze,
    Silver,
    Gold,
    Platinum,
    Emerald,
    Diamond,
    Master,
    Grandmaster,
    Challenger,
}