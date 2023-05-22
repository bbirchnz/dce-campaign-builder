use std::collections::HashMap;

use bevy_reflect::{FromReflect, Reflect};
use serde::{Deserialize, Serialize};
use tables::{FieldType, HeaderField, TableHeader};

use crate::{
    editable::{Editable, ValidationResult},
    mission::Payload,
    serde_utils::LuaFileBased,
    DCEInstance, NewFromMission,
};

pub type Loadouts = HashMap<String, AirframeLoadout>;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct AirframeLoadout {
    #[serde(rename = "Strike")]
    pub strike: Option<HashMap<String, StrikeLoadout>>,
    #[serde(rename = "Anti-ship Strike")]
    pub anti_ship: Option<HashMap<String, AntiShipLoadout>>,
    #[serde(rename = "CAP")]
    pub cap: Option<HashMap<String, CAPLoadout>>,
}

pub type AntiShipLoadout = StrikeLoadout;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Reflect, FromReflect)]
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
    pub capability: u32,
    pub firepower: u32,
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
    #[serde(default)]
    pub t_station: u32,
    #[serde(rename = "LDSD")]
    pub ldsd: bool,
    pub stores: Payload,
    #[serde(default)]
    pub self_escort: bool,
    pub sortie_rate: u32,
    #[serde(default)]
    pub _airframe: String,
    pub _name: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Reflect, FromReflect)]
pub struct CAPLoadout {
    pub day: bool,
    pub night: bool,
    #[serde(rename = "adverseWeather")]
    pub adverse_weather: bool,
    pub range: f64,
    pub capability: u32,
    pub firepower: u32,
    #[serde(rename = "vCruise")]
    pub v_cruise: f64,
    #[serde(rename = "vAttack")]
    pub v_attack: f64,
    #[serde(rename = "hCruise")]
    pub h_cruise: f64,
    #[serde(rename = "hAttack")]
    pub h_attack: f64,
    #[serde(rename = "tStation")]
    #[serde(default)]
    pub t_station: u32,
    #[serde(rename = "LDSD")]
    pub ldsd: bool,
    pub stores: Payload,
    #[serde(default)]
    pub sortie_rate: u32,
    pub _airframe: String,
    pub _name: String,
}

fn common_headers() -> Vec<HeaderField> {
    vec![
        HeaderField {
            field: "_name".into(),
            display: "Name".into(),
            type_: FieldType::String,
            editable: true,
        },
        HeaderField {
            field: "_airframe".into(),
            display: "Airframe".into(),
            type_: FieldType::String,
            editable: false,
        },
        HeaderField {
            field: "day".into(),
            display: "Day".into(),
            type_: FieldType::Bool,
            editable: true,
        },
        HeaderField {
            field: "night".into(),
            display: "Night".into(),
            type_: FieldType::Bool,
            editable: true,
        },
        HeaderField {
            field: "adverse_weather".into(),
            display: "Adverse Weather".into(),
            type_: FieldType::Bool,
            editable: true,
        },
        HeaderField {
            field: "range".into(),
            display: "Range (nm)".into(),
            type_: FieldType::DistanceNM,
            editable: true,
        },
        HeaderField {
            field: "capability".into(),
            display: "Capability".into(),
            type_: FieldType::Int,
            editable: true,
        },
        HeaderField {
            field: "firepower".into(),
            display: "Firepower".into(),
            type_: FieldType::Int,
            editable: true,
        },
    ]
}

impl TableHeader for CAPLoadout {
    fn get_header() -> Vec<tables::HeaderField> {
        let mut common = common_headers();
        common.extend(vec![
            HeaderField {
                field: "v_cruise".into(),
                display: "Cruise Speed (knots TAS)".into(),
                type_: FieldType::SpeedKnotsTAS,
                editable: true,
            },
            HeaderField {
                field: "h_cruise".into(),
                display: "Cruise Altitude (ft)".into(),
                type_: FieldType::AltitudeFeet,
                editable: true,
            },
            HeaderField {
                field: "t_station".into(),
                display: "Time on station (min)".into(),
                type_: FieldType::DurationMin,
                editable: true,
            },
        ]);
        common
    }
}

impl TableHeader for StrikeLoadout {
    fn get_header() -> Vec<tables::HeaderField> {
        let mut common = common_headers();
        common.extend(vec![
            HeaderField {
                field: "v_cruise".into(),
                display: "Cruise Speed (knots TAS)".into(),
                type_: FieldType::SpeedKnotsTAS,
                editable: true,
            },
            HeaderField {
                field: "h_cruise".into(),
                display: "Cruise Altitude (ft)".into(),
                type_: FieldType::AltitudeFeet,
                editable: true,
            },
            HeaderField {
                field: "v_attack".into(),
                display: "Attack Speed (knots TAS)".into(),
                type_: FieldType::SpeedKnotsTAS,
                editable: true,
            },
            HeaderField {
                field: "h_attack".into(),
                display: "Attack Altitude (ft)".into(),
                type_: FieldType::AltitudeFeet,
                editable: true,
            },
        ]);
        common
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Reflect, FromReflect)]
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
                        cap: Some(HashMap::default()),
                        anti_ship: Some(HashMap::default()),
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
                                capability: 1,
                                firepower: 1,
                                v_cruise: 225.,
                                v_attack: 277.5,
                                h_cruise: 7000.,
                                h_attack: 6706.,
                                standoff: None,
                                t_station: 0,
                                ldsd: false,
                                stores: u.payload.clone(),
                                self_escort: false,
                                sortie_rate: 6,
                                _airframe: u._type.to_owned(),
                                _name: u.name.to_owned(),
                            },
                        );
                    }
                    "CAP" => {
                        unit_record.cap.as_mut().unwrap().insert(
                            u.name.to_owned(),
                            CAPLoadout {
                                day: true,
                                night: true,
                                adverse_weather: true,
                                range: 2000000.,
                                capability: 1,
                                firepower: 1,
                                v_cruise: 225.,
                                v_attack: 246.,
                                h_cruise: 6096.,
                                h_attack: 6096.,
                                t_station: 2400,
                                ldsd: false,
                                stores: u.payload.clone(),
                                sortie_rate: 6,
                                _airframe: u._type.to_owned(),
                                _name: u.name.to_owned(),
                            },
                        );
                    }
                    "Anti-ship Strike" => {
                        unit_record.anti_ship.as_mut().unwrap().insert(
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
                                capability: 1,
                                firepower: 1,
                                v_cruise: 225.,
                                v_attack: 277.5,
                                h_cruise: 7000.,
                                h_attack: 6706.,
                                standoff: None,
                                t_station: 0,
                                ldsd: false,
                                stores: u.payload.clone(),
                                self_escort: false,
                                sortie_rate: 6,
                                _airframe: u._type.to_owned(),
                                _name: u.name.to_owned(),
                            },
                        );
                    }
                    _ => {}
                }
            });

        Ok(loadout)
    }
}

impl Editable for CAPLoadout {
    fn get_mut_by_name<'a>(instance: &'a mut DCEInstance, name: &str) -> &'a mut Self {
        instance
            .loadouts
            .cap
            .iter_mut()
            .find(|item| item._name == name)
            .unwrap()
    }

    fn get_name(&self) -> String {
        self._name.to_owned()
    }

    fn validate(&self, _: &DCEInstance) -> ValidationResult {
        let errors = Vec::default();

        // todo: Probably want to put some limits on speeds/altitudes

        if errors.is_empty() {
            return ValidationResult::Pass;
        }
        ValidationResult::Fail(errors)
    }
}

impl Editable for StrikeLoadout {
    fn get_mut_by_name<'a>(instance: &'a mut DCEInstance, name: &str) -> &'a mut Self {
        instance
            .loadouts
            .strike
            .iter_mut()
            .find(|item| item._name == name)
            .unwrap()
    }

    fn get_name(&self) -> String {
        self._name.to_owned()
    }

    fn validate(&self, _: &DCEInstance) -> ValidationResult {
        let errors = Vec::default();

        // todo: Probably want to put some limits on speeds/altitudes
        if errors.is_empty() {
            return ValidationResult::Pass;
        }
        ValidationResult::Fail(errors)
    }
}

#[cfg(test)]
mod tests {
    use crate::{mission::Mission, serde_utils::LuaFileBased, NewFromMission};

    use super::Loadouts;

    #[test]
    fn from_miz() {
        let mission = Mission::from_miz("C:\\Users\\Ben\\Saved Games\\DCS.openbeta\\Mods\\tech\\DCE\\Missions\\Campaigns\\Falklands v1\\Init\\base_mission.miz".into()).unwrap();
        let loadouts = Loadouts::new_from_mission(&mission).unwrap();

        loadouts
            .to_lua_file("db_loadouts.lua".into(), "db_loadouts".into())
            .unwrap();
    }
}
