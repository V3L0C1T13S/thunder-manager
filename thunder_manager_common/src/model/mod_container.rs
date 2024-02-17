use serde::{Deserialize, Serialize};

use super::game_mod::GameMod;

#[derive(Serialize, Deserialize, Clone)]
pub struct ModContainer {
    pub name: String,
    pub version: Option<i32>,
    pub mods: Vec<GameMod>,
}
