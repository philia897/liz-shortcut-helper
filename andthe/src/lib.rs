use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum StateCode {
    OK,
    FAIL,
    BUG,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LizCommand {
    pub action: String,
    pub args: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BlueBirdResponse {
    pub code: StateCode,
    pub results: Vec<String>,
}

impl LizCommand {
    pub fn serialize(&self) -> Option<Vec<u8>> {
        serde_json::to_vec(self).ok()
    }

    pub fn deserialize(json: &[u8]) -> Option<Self> {
        serde_json::from_slice(json).ok()
    }
}

impl BlueBirdResponse {
    pub fn serialize(&self) -> Option<Vec<u8>> {
        serde_json::to_vec(self).ok()
    }

    pub fn deserialize(json: &[u8]) -> Option<Self> {
        serde_json::from_slice(json).ok()
    }
}