use serde::{Deserialize, Serialize};

use super::thunderstore_mod::ThunderstoreMod;

#[derive(Serialize, Deserialize, Clone)]
pub struct ModContainer {
    pub name: String,
    pub version: Option<i32>,
    pub mods: Vec<ThunderstoreMod>,
}
