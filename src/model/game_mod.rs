use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct GameMod {
    pub file_name: String,
    pub url: String,
    pub version: i32,
    pub mod_type: Option<String>,
    pub extract_to: Option<String>,
    pub root: Option<String>,
}
