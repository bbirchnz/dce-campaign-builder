use serde::{Deserialize, Serialize};

use crate::serde_utils::LuaFileBased;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct ConfMod {
    #[serde(rename = "SelectLoadout")]
    pub select_loadout: String,
}

impl ConfMod {
    pub fn new() -> ConfMod {
        ConfMod {
            select_loadout: "init".into(),
        }
    }
}

impl LuaFileBased<'_> for ConfMod {}
