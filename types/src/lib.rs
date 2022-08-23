use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct EchoResponse {
    pub item: String
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
pub enum PostCategory {
    Events,
    Announcements,
    Newsletter,
    Engineering,
    Marketing,
    Business,
    #[default]
    NotCategorized,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
pub struct PostResponse {
    pub id: String,
    pub title: String,
    pub author: String,
    pub categories: String,
    pub body: String,
    pub time_created: String,
}
