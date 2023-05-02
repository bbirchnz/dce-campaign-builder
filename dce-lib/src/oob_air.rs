use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::serde_utils::LuaFileBased;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct OobAir {
    blue: Vec<Squadron>,
    red: Vec<Squadron>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]

enum LiveryEnum {
    One(String),
    Many(Vec<String>),
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Squadron {
    name: String,
    inactive: Option<bool>,
    player: Option<bool>,

    #[serde(rename = "type")]
    _type: String,
    country: String,

    livery: LiveryEnum,

    base: String,
    skill: String,
    tasks: HashMap<String, bool>,

    #[serde(rename = "tasksCoef")]
    tasks_coef: Option<HashMap<String, f32>>,
    number: u16,
    reserve: u16,
}

impl LuaFileBased<'_> for OobAir {}

#[cfg(test)]
mod tests {
    use crate::serde_utils::LuaFileBased;

    use super::OobAir;

    #[test]
    fn load_example() {
        let result = OobAir::from_lua_file("C:\\Users\\Ben\\Saved Games\\DCS.openbeta\\Mods\\tech\\DCE\\Missions\\Campaigns\\War over Tchad 1987-Blue-Mirage-F1EE-3-30 Lorraine\\Init\\oob_air_init.lua".into(), "oob_air".into());

        result.unwrap();
    }

    #[test]
    fn save_example() {
        let oob = OobAir::from_lua_file("C:\\Users\\Ben\\Saved Games\\DCS.openbeta\\Mods\\tech\\DCE\\Missions\\Campaigns\\War over Tchad 1987-Blue-Mirage-F1EE-3-30 Lorraine\\Init\\oob_air_init.lua".into(), "oob_air".into()).unwrap();
        oob.to_lua_file("test.lua".into(), "oob_air".into())
            .unwrap();
    }
}
