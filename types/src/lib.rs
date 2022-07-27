use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct EchoResponse {
    pub item: String
}

#[derive(Deserialize)]
pub enum Status {
    Success,
    Error,
    Unknown,
}