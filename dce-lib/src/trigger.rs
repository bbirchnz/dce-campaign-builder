use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{serde_utils::LuaFileBased, NewFromMission};

type Triggers = HashMap<String, Trigger>;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Trigger {
    #[serde(default)]
    pub active: bool,
    #[serde(default)]
    pub once: bool,
    pub condition: String,
    pub action: Actions
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum Actions {
    One(String),
    Many(Vec<String>)
}
impl LuaFileBased<'_> for Triggers {}

// impl NewFromMission for Triggers {
//     fn new_from_mission(_mission: &crate::mission::Mission) -> Result<Self, anyhow::Error>
//     where
//         Self: Sized,
//     {
//         Ok(Header {
//             original: true,
//             title: "New Campaign".into(),
//             version: "V0.1".into(),
//             mission: 1,
//             date: Date {
//                 day: 9,
//                 month: 5,
//                 year: 2023,
//             },
//             time: 11700,
//             dawn: 19800,
//             dusk: 68880,
//             mission_duration: 5400,
//             idle_time_min: 10800,
//             idle_time_max: 14400,
//             startup: 600,
//             units: "imperial".into(),
//             weather: Weather {
//                 high_prob: 20.,
//                 low_prob: 80.,
//                 reference_temp: 8.,
//             },
//             mag_var: 2.,
//             debug: true,
//         })
//     }
// }

#[cfg(test)]
mod tests {
    use crate::{mission::Mission, serde_utils::LuaFileBased, NewFromMission};

    use super::Triggers;

    #[test]
    fn load_example() {
        let result = Triggers::from_lua_file("C:\\Users\\Ben\\Saved Games\\DCS.openbeta\\Mods\\tech\\DCE\\Missions\\Campaigns\\War over Tchad 1987-Blue-Mirage-F1EE-3-30 Lorraine\\Init\\camp_triggers_init.lua".into(), "camp_triggers".into());

        result.unwrap();
    }

    #[test]
    fn save_example() {
        let loaded = Triggers::from_lua_file("C:\\Users\\Ben\\Saved Games\\DCS.openbeta\\Mods\\tech\\DCE\\Missions\\Campaigns\\War over Tchad 1987-Blue-Mirage-F1EE-3-30 Lorraine\\Init\\camp_triggers_init.lua".into(), "camp_triggers".into()).unwrap();
        loaded
            .to_lua_file("camp_triggers_sa.lua".into(), "camp".into())
            .unwrap();
    }

    // #[test]
    // fn from_miz() {
    //     let mission = Mission::from_miz("C:\\Users\\Ben\\Saved Games\\DCS.openbeta\\Mods\\tech\\DCE\\Missions\\Campaigns\\Falklands v1\\Init\\base_mission.miz".into()).unwrap();
    //     let header = Header::new_from_mission(&mission).unwrap();

    //     header
    //         .to_lua_file("camp_init_sa.lua".into(), "camp".into())
    //         .unwrap();
    // }
}
