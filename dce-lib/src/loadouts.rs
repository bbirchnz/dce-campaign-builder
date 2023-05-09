use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{mission::Payload, serde_utils::LuaFileBased, NewFromMission};

pub type Loadouts = HashMap<String, AirframeLoadout>;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct AirframeLoadout {
    #[serde(rename = "Strike")]
    pub strike: Option<HashMap<String, StrikeLoadout>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct StrikeLoadout {
    pub minscore: f64,
    pub support: Support,
    #[serde(rename = "weaponType")]
    pub weapon_type: String,
    pub expend: String,
    pub day: bool,
    pub night: bool,
    #[serde(rename = "adverseWeather")]
    pub adverse_weather: bool,
    pub range: f64,
    pub capability: f64,
    pub firepower: f64,
    #[serde(rename = "vCruise")]
    pub v_cruise: f64,
    #[serde(rename = "vAttack")]
    pub v_attack: f64,
    #[serde(rename = "hCruise")]
    pub h_cruise: f64,
    #[serde(rename = "hAttack")]
    pub h_attack: f64,
    pub standoff: Option<f64>,
    #[serde(rename = "tStation")]
    pub t_station: Option<f64>,
    #[serde(rename = "LDSD")]
    pub ldsd: bool,
    pub stores: Payload,
    #[serde(default)]
    pub self_escort: bool,
    pub sortie_rate: u32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CAPLoadout {
    pub day: bool,
    pub night: bool,
    #[serde(rename = "adverseWeather")]
    pub adverse_weather: bool,
    pub range: f64,
    pub capability: f64,
    pub firepower: f64,
    #[serde(rename = "vCruise")]
    pub v_cruise: f64,
    #[serde(rename = "vAttack")]
    pub v_attack: f64,
    #[serde(rename = "hCruise")]
    pub h_cruise: f64,
    #[serde(rename = "hAttack")]
    pub h_attack: f64,
    #[serde(rename = "tStation")]
    pub t_station: Option<f64>,
    #[serde(rename = "LDSD")]
    pub ldsd: bool,
    pub stores: Payload,
    #[serde(default)]
    pub sortie_rate: u32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Support {
    #[serde(default)]
    #[serde(rename = "Escort")]
    escort: bool,
    #[serde(default)]
    #[serde(rename = "SEAD")]
    sead: bool,
    #[serde(default)]
    #[serde(rename = "Escort Jammer")]
    escort_jammer: bool,
}

impl LuaFileBased<'_> for Loadouts {}

impl NewFromMission for Loadouts {
    fn new_from_mission(mission: &crate::mission::Mission) -> Result<Self, anyhow::Error>
    where
        Self: Sized,
    {
        let mut loadout: Loadouts = HashMap::default();
        mission
            .coalition
            .blue
            .countries
            .iter()
            .chain(mission.coalition.red.countries.iter())
            .filter_map(|c| c.plane.as_ref())
            .flat_map(|pg| pg.groups.as_slice())
            .flat_map(|g| g.units.as_slice())
            .for_each(|u| {
                let name_parts = u.name.split('_').collect::<Vec<_>>();
                let unit_record = loadout
                    .entry(u._type.to_owned())
                    .or_insert(AirframeLoadout {
                        strike: Some(HashMap::default()),
                    });
                match name_parts[1] {
                    "Strike" => {
                        unit_record.strike.as_mut().unwrap().insert(
                            u.name.to_owned(),
                            StrikeLoadout {
                                minscore: 0.3,
                                support: Support {
                                    escort: true,
                                    sead: true,
                                    escort_jammer: false,
                                },
                                weapon_type: "Bombs".into(),
                                expend: "All".into(),
                                day: true,
                                night: true,
                                adverse_weather: true,
                                range: 500000.,
                                capability: 1.,
                                firepower: 1.,
                                v_cruise: 225.,
                                v_attack: 277.5,
                                h_cruise: 7000.,
                                h_attack: 6706.,
                                standoff: None,
                                t_station: None,
                                ldsd: false,
                                stores: u.payload.clone(),
                                self_escort: false,
                                sortie_rate: 6,
                            },
                        );
                    }
                    _ => {}
                }
            });

        Ok(loadout)
    }
}

#[cfg(test)]
mod tests {
    use crate::{mission::Mission, serde_utils::LuaFileBased, NewFromMission};

    use super::Loadouts;

    // #[test]
    // fn load_example() {
    //     let result = Loadouts::from_lua_file("C:\\Users\\Ben\\Saved Games\\DCS.openbeta\\Mods\\tech\\DCE\\Missions\\Campaigns\\War over Tchad 1987-Blue-Mirage-F1EE-3-30 Lorraine\\Init\\db_airbases.lua".into(), "db_airbases".into());

    //     result.unwrap();
    // }

    // #[test]
    // fn save_example() {
    //     let loadouts = Loadouts::from_lua_file("C:\\Users\\Ben\\Saved Games\\DCS.openbeta\\Mods\\tech\\DCE\\Missions\\Campaigns\\War over Tchad 1987-Blue-Mirage-F1EE-3-30 Lorraine\\Init\\db_airbases.lua".into(), "db_airbases".into()).unwrap();
    //     loadouts.to_lua_file("db_airbases.lua".into(), "db_airbases".into())
    //         .unwrap();
    // }

    #[test]
    fn from_miz() {
        let mission = Mission::from_miz("C:\\Users\\Ben\\Saved Games\\DCS.openbeta\\Mods\\tech\\DCE\\Missions\\Campaigns\\Falklands v1\\Init\\base_mission.miz".into()).unwrap();
        let loadouts = Loadouts::new_from_mission(&mission).unwrap();

        loadouts
            .to_lua_file("db_loadouts.lua".into(), "db_loadouts".into())
            .unwrap();
    }
}
