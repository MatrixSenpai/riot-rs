use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// A player account
pub struct AccountDto {
    /// The player id
    pub puuid: String,
    /// This field may be excluded from the response if the account doesn't have a game name
    pub game_name: Option<String>,
    /// This field may be excluded from the response if the account doesn't have a tag line
    pub tag_line: Option<String>,
}

impl Eq for AccountDto {}
impl PartialEq for AccountDto {
    fn eq(&self, other: &Self) -> bool {
        self.puuid == other.puuid
    }
}

impl Into<String> for AccountDto {
    fn into(self) -> String {
        self.puuid
    }
}

impl Display for AccountDto {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,
            "{}: {}#{}",
            &self.puuid,
            &self.game_name.clone().unwrap_or("EMPTY".to_string()),
            &self.tag_line.clone().unwrap_or("NTL".to_string())
        )
    }
}