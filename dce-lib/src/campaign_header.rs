use bevy_reflect::{FromReflect, Reflect};
use serde::{Deserialize, Serialize};
use tables::{FieldType, HeaderField, TableHeader};

use crate::{serde_utils::LuaFileBased, NewFromMission};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Reflect, FromReflect)]
pub struct Header {
    #[serde(rename = "CampaignOriginal")]
    pub original: bool,
    pub title: String,
    pub version: String,
    pub mission: u8,
    pub date: Date,
    pub time: u32,
    pub dawn: u32,
    pub dusk: u32,
    pub mission_duration: u32,
    pub idle_time_min: u32,
    pub idle_time_max: u32,
    pub startup: u32,
    pub units: String,
    pub weather: Weather,
    #[serde(rename = "variation")]
    pub mag_var: f64,
    pub debug: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Reflect, FromReflect)]
pub struct Date {
    pub day: u8,
    pub month: u8,
    pub year: u16,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Reflect, FromReflect)]
pub struct Weather {
    #[serde(rename = "pHigh")]
    pub high_prob: f32,
    #[serde(rename = "pLow")]
    pub low_prob: f32,
    #[serde(rename = "refTemp")]
    pub reference_temp: f32,
}

impl LuaFileBased<'_> for Header {}

impl NewFromMission for Header {
    fn new_from_mission(_mission: &crate::mission::Mission) -> Result<Self, anyhow::Error>
    where
        Self: Sized,
    {
        Ok(Header {
            original: true,
            title: "New Campaign".into(),
            version: "V0.1".into(),
            mission: 1,
            date: Date {
                day: 9,
                month: 5,
                year: 2023,
            },
            time: 11700,
            dawn: 19800,
            dusk: 68880,
            mission_duration: 5400,
            idle_time_min: 10800,
            idle_time_max: 14400,
            startup: 600,
            units: "imperial".into(),
            weather: Weather {
                high_prob: 20.,
                low_prob: 80.,
                reference_temp: 8.,
            },
            mag_var: 2.,
            debug: true,
        })
    }
}

impl TableHeader for Header {
    fn get_header() -> Vec<tables::HeaderField> {
        vec![
            HeaderField {
                field: "title".into(),
                display: "Title".into(),
                type_: FieldType::String,
                editable: true,
            },
            HeaderField {
                field: "version".into(),
                display: "Version".into(),
                type_: FieldType::String,
                editable: true,
            },
            HeaderField {
                field: "dawn".into(),
                display: "Dawn".into(),
                type_: FieldType::IntTime,
                editable: true,
            },
            HeaderField {
                field: "dusk".into(),
                display: "Dusk".into(),
                type_: FieldType::IntTime,
                editable: true,
            },
            HeaderField {
                field: "mission_duration".into(),
                display: "Mission Duration".into(),
                type_: FieldType::Int,
                editable: true,
            },
            HeaderField {
                field: "startup".into(),
                display: "Startup Time".into(),
                type_: FieldType::Int,
                editable: true,
            },
            HeaderField {
                field: "units".into(),
                display: "Units of Measure".into(),
                type_: FieldType::String,
                editable: true,
            },
            HeaderField {
                field: "mag_var".into(),
                display: "Magnetic Variation".into(),
                type_: FieldType::Float(|v| format!("{:.1}", v)),
                editable: true,
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use crate::{mission::Mission, serde_utils::LuaFileBased, NewFromMission};

    use super::Header;

    #[test]
    fn load_example() {
        let result = Header::from_lua_file("C:\\Users\\Ben\\Saved Games\\DCS.openbeta\\Mods\\tech\\DCE\\Missions\\Campaigns\\War over Tchad 1987-Blue-Mirage-F1EE-3-30 Lorraine\\Init\\camp_init.lua".into(), "camp".into());

        result.unwrap();
    }

    #[test]
    fn save_example() {
        let loaded = Header::from_lua_file("C:\\Users\\Ben\\Saved Games\\DCS.openbeta\\Mods\\tech\\DCE\\Missions\\Campaigns\\War over Tchad 1987-Blue-Mirage-F1EE-3-30 Lorraine\\Init\\camp_init.lua".into(), "camp".into()).unwrap();
        loaded
            .to_lua_file("camp_init_sa.lua".into(), "camp".into())
            .unwrap();
    }

    #[test]
    fn from_miz() {
        let mission = Mission::from_miz("C:\\Users\\Ben\\Saved Games\\DCS.openbeta\\Mods\\tech\\DCE\\Missions\\Campaigns\\Falklands v1\\Init\\base_mission.miz".into()).unwrap();
        let header = Header::new_from_mission(&mission).unwrap();

        header
            .to_lua_file("camp_init_sa.lua".into(), "camp".into())
            .unwrap();
    }
}
