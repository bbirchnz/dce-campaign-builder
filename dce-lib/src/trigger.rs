use std::collections::HashMap;

use mlua::Lua;
use serde::{Deserialize, Serialize};

use crate::{
    dce_utils::ValidateSelf, lua_utils::load_trigger_mocks, serde_utils::LuaFileBased,
    NewFromMission,
};

pub type Triggers = HashMap<String, Trigger>;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Trigger {
    #[serde(default)]
    pub active: bool,
    #[serde(default)]
    pub once: bool,
    pub condition: String,
    pub action: Actions,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum Actions {
    One(String),
    Many(Vec<String>),
}
impl LuaFileBased<'_> for Triggers {}

impl ValidateSelf for Trigger {
    fn validate_self(&self) -> Result<(), anyhow::Error> {
        let lua = Lua::new();
        load_trigger_mocks(&lua)?;

        let condition = "local condition = ".to_string();
        let condition = condition + &self.condition;

        lua.load(&condition).exec()?;

        match &self.action {
            Actions::One(action) => lua.load(action).exec()?,
            Actions::Many(actions) => actions
                .iter()
                .for_each(|action| lua.load(action).exec().unwrap()),
        }

        Ok(())
    }
}

impl ValidateSelf for Triggers {
    fn validate_self(&self) -> Result<(), anyhow::Error> {
        self.iter()
            .for_each(|(_, trigger)| trigger.validate_self().unwrap());
        Ok(())
    }
}

impl NewFromMission for Triggers {
    fn new_from_mission(_mission: &crate::mission::Mission) -> Result<Self, anyhow::Error>
    where
        Self: Sized,
    {
        Ok(HashMap::from([(
            "Campaign Briefing".into(),
            Trigger {
                active: true,
                once: true,
                condition: "true".into(),
                action: Actions::One("Action.Text(\"Welcome to your new campaign\")".into()),
            },
        )]))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        dce_utils::ValidateSelf, mission::Mission, serde_utils::LuaFileBased, NewFromMission,
    };

    use super::Triggers;

    #[test]
    fn load_example() {
        let result = Triggers::from_lua_file("C:\\Users\\Ben\\Saved Games\\DCS.openbeta\\Mods\\tech\\DCE\\Missions\\Campaigns\\War over Tchad 1987-Blue-Mirage-F1EE-3-30 Lorraine\\Init\\camp_triggers_init.lua".into(), "camp_triggers".into());

        let result = result.unwrap();
        result.validate_self().unwrap();
    }

    #[test]
    fn save_example() {
        let loaded = Triggers::from_lua_file("C:\\Users\\Ben\\Saved Games\\DCS.openbeta\\Mods\\tech\\DCE\\Missions\\Campaigns\\War over Tchad 1987-Blue-Mirage-F1EE-3-30 Lorraine\\Init\\camp_triggers_init.lua".into(), "camp_triggers".into()).unwrap();
        loaded
            .to_lua_file("camp_triggers_sa.lua".into(), "camp".into())
            .unwrap();
    }

    #[test]
    fn from_miz() {
        let mission = Mission::from_miz("C:\\Users\\Ben\\Saved Games\\DCS.openbeta\\Mods\\tech\\DCE\\Missions\\Campaigns\\Falklands v1\\Init\\base_mission.miz".into()).unwrap();
        let triggers = Triggers::new_from_mission(&mission).unwrap();

        triggers
            .to_lua_file("camp_trigger_init_sa.lua".into(), "camp_triggers".into())
            .unwrap();
    }
}
