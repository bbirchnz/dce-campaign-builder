use std::collections::HashMap;

use crate::{
    editable::{Editable, FieldType, HeaderField, NestedEditable, ValidationResult},
    loadouts_internal::LoadoutsInternal,
    mission::Payload,
    miz_environment::MizEnvironment,
    serde_utils::LuaFileBased,
    DCEInstance, NewFromMission,
};
use anyhow::{anyhow, Ok};
use bevy_reflect::{FromReflect, Reflect};
use serde::{Deserialize, Serialize};

pub type Loadouts = HashMap<String, AirframeLoadout>;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct AirframeLoadout {
    #[serde(rename = "Strike")]
    pub strike: Option<HashMap<String, StrikeLoadout>>,
    #[serde(rename = "Anti-ship Strike")]
    pub anti_ship: Option<HashMap<String, AntiShipLoadout>>,
    #[serde(rename = "CAP")]
    pub cap: Option<HashMap<String, CAPLoadout>>,
    #[serde(rename = "AWACS")]
    pub awacs: Option<HashMap<String, AWACSLoadout>>,
    #[serde(rename = "Refueling")]
    pub aar: Option<HashMap<String, AARLoadout>>,
    #[serde(rename = "Escort")]
    pub escort: Option<HashMap<String, EscortLoadout>>,
    #[serde(rename = "SEAD")]
    pub sead: Option<HashMap<String, SEADLoadout>>,
    #[serde(rename = "Intercept")]
    pub intercept: Option<HashMap<String, InterceptLoadout>>,
    // fighter sweep and intercept use the same functions
    #[serde(rename = "Fighter Sweep")]
    pub sweep: Option<HashMap<String, SweepLoadout>>,
    #[serde(rename = "Transport")]
    pub transport: Option<HashMap<String, TransportLoadout>>,
}

/// Match tasks of any case to the proper cased format required by DCE, throws error if
/// not a valid task
pub fn str_to_task(original: &str) -> Result<String, anyhow::Error> {
    match original.to_lowercase().as_str() {
        "strike" => Ok("Strike".to_owned()),
        "anti-ship strike" => Ok("Anti-ship Strike".to_owned()),
        "cap" => Ok("CAP".to_owned()),
        "awacs" => Ok("AWACS".to_owned()),
        "refueling" => Ok("Refueling".to_owned()),
        "sead" => Ok("SEAD".to_owned()),
        "escort" => Ok("Escort".to_owned()),
        "intercept" => Ok("Intercept".to_owned()),
        "fighter sweep" => Ok("Fighter Sweep".to_owned()),
        "transport" => Ok("Transport".to_owned()),
        _ => Err(anyhow!("{:?} is not a supported task", original)),
    }
}

impl Default for AirframeLoadout {
    fn default() -> Self {
        Self {
            strike: Some(Default::default()),
            anti_ship: Some(Default::default()),
            cap: Some(Default::default()),
            awacs: Some(Default::default()),
            aar: Some(Default::default()),
            escort: Some(Default::default()),
            sead: Some(Default::default()),
            intercept: Some(Default::default()),
            sweep: Some(Default::default()),
            transport: Some(Default::default()),
        }
    }
}

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
    #[serde(default)]
    pub attributes: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Reflect, FromReflect)]
pub struct AntiShipLoadout {
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
    #[serde(default)]
    pub attributes: Vec<String>,
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
    #[serde(default)]
    pub attributes: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Reflect, FromReflect)]
pub struct SweepLoadout {
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
    pub standoff: f64,
    pub _airframe: String,
    pub _name: String,
    #[serde(default)]
    pub attributes: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Reflect, FromReflect)]
pub struct AWACSLoadout {
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
    pub stores: Payload,
    #[serde(default)]
    pub sortie_rate: u32,
    pub _airframe: String,
    pub _name: String,
    #[serde(default)]
    pub attributes: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Reflect, FromReflect)]
pub struct AARLoadout {
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
    pub stores: Payload,
    #[serde(default)]
    pub sortie_rate: u32,
    pub _airframe: String,
    pub _name: String,
    #[serde(default)]
    pub attributes: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Reflect, FromReflect)]
pub struct EscortLoadout {
    pub day: bool,
    pub night: bool,
    #[serde(rename = "adverseWeather")]
    pub adverse_weather: bool,
    pub range: f64,
    pub capability: u32,
    pub firepower: u32,
    #[serde(rename = "vCruise")]
    pub v_cruise: f64,
    #[serde(rename = "LDSD")]
    pub ldsd: bool,
    pub standoff: f64,
    pub stores: Payload,
    #[serde(default)]
    pub sortie_rate: u32,
    pub _airframe: String,
    pub _name: String,
    #[serde(default)]
    pub attributes: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Reflect, FromReflect)]
pub struct InterceptLoadout {
    pub day: bool,
    pub night: bool,
    #[serde(rename = "adverseWeather")]
    pub adverse_weather: bool,
    pub range: f64,
    pub capability: u32,
    pub firepower: u32,
    #[serde(rename = "LDSD")]
    pub ldsd: bool,
    pub stores: Payload,
    #[serde(default)]
    pub sortie_rate: u32,
    pub _airframe: String,
    pub _name: String,
    #[serde(default)]
    pub attributes: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Reflect, FromReflect)]
pub struct TransportLoadout {
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
    pub stores: Payload,
    #[serde(default)]
    pub sortie_rate: u32,
    pub _airframe: String,
    pub _name: String,
    #[serde(default)]
    pub attributes: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Reflect, FromReflect)]
pub struct SEADLoadout {
    pub day: bool,
    pub night: bool,
    #[serde(rename = "adverseWeather")]
    pub adverse_weather: bool,
    pub range: f64,
    pub capability: u32,
    pub firepower: u32,
    #[serde(rename = "vCruise")]
    pub v_cruise: f64,
    pub stores: Payload,
    #[serde(default)]
    pub sortie_rate: u32,
    pub _airframe: String,
    pub _name: String,
    #[serde(default)]
    pub attributes: Vec<String>,
}

// SAR
// CSAR

fn common_headers() -> Vec<HeaderField> {
    vec![
        HeaderField::new("_name", "Name", FieldType::String, true),
        HeaderField::new("_airframe", "AirFrame", FieldType::String, false),
        HeaderField::new("day", "Day", FieldType::Bool, true),
        HeaderField::new("night", "Night", FieldType::Bool, true),
        HeaderField::new("adverse_weather", "Adverse Weather", FieldType::Bool, true),
        HeaderField::new("range", "Range (nm)", FieldType::DistanceNM, true),
        HeaderField::new("capability", "Capability", FieldType::Int, true),
        HeaderField::new("firepower", "Firepower", FieldType::Int, true),
        HeaderField::new(
            "v_cruise",
            "Cruise Speed (knots TAS)",
            FieldType::SpeedKnotsTAS,
            true,
        ),
        HeaderField::new(
            "h_cruise",
            "Cruise Altitude (ft)",
            FieldType::AltitudeFeet,
            true,
        ),
        HeaderField::new(
            "v_attack",
            "Attack Speed (knots TAS)",
            FieldType::SpeedKnotsTAS,
            true,
        ),
        HeaderField::new(
            "h_attack",
            "Attack Altitude (ft)",
            FieldType::AltitudeFeet,
            true,
        ),
    ]
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

impl NestedEditable for Support {
    fn validate(&self, _: &DCEInstance) -> ValidationResult {
        ValidationResult::Pass
    }

    fn get_header() -> Vec<HeaderField>
    where
        Self: Sized,
    {
        vec![
            HeaderField::new("escort", "Needs AA Escort", FieldType::Bool, true),
            HeaderField::new("sead", "Needs SEAD Escort", FieldType::Bool, true),
            HeaderField::new(
                "escort_jammer",
                "Needs Jammer Escort",
                FieldType::Bool,
                true,
            ),
        ]
    }
}

impl LuaFileBased<'_> for Loadouts {}

impl NewFromMission for Loadouts {
    fn new_from_mission(miz: &MizEnvironment) -> Result<Self, anyhow::Error>
    where
        Self: Sized,
    {
        let mut loadout: Loadouts = HashMap::default();
        let countries = miz
            .mission
            .coalition
            .blue
            .countries
            .iter()
            .chain(miz.mission.coalition.red.countries.iter());

        countries
            .clone()
            .filter_map(|c| c.plane.as_ref())
            .chain(countries.filter_map(|c| c.helicopter.as_ref()))
            .flat_map(|pg| pg.groups.as_slice())
            .flat_map(|g| g.units.as_slice())
            .try_for_each(|u| {
                let name_parts = u.name.split('_').collect::<Vec<_>>();
                let task = name_parts[1].to_lowercase();
                let unit_record = loadout
                    .entry(u._type.to_owned())
                    .or_insert(AirframeLoadout::default());
                match task.as_str() {
                    "strike" => {
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
                                v_cruise: 246.,
                                v_attack: 277.5,
                                h_cruise: 9090.,
                                h_attack: 9090.,
                                standoff: None,
                                t_station: 0,
                                ldsd: false,
                                stores: u.payload.clone(),
                                self_escort: false,
                                sortie_rate: 6,
                                _airframe: u._type.to_owned(),
                                _name: u.name.to_owned(),
                                attributes: Vec::default(),
                            },
                        );
                        Ok(())
                    }
                    "cap" => {
                        unit_record.cap.as_mut().unwrap().insert(
                            u.name.to_owned(),
                            CAPLoadout {
                                day: true,
                                night: true,
                                adverse_weather: true,
                                range: 2000000.,
                                capability: 1,
                                firepower: 1,
                                v_cruise: 246.,
                                v_attack: 246.,
                                h_cruise: 9090.,
                                h_attack: 9090.,
                                t_station: 2400,
                                ldsd: true,
                                stores: u.payload.clone(),
                                sortie_rate: 6,
                                _airframe: u._type.to_owned(),
                                _name: u.name.to_owned(),
                                attributes: Vec::default(),
                            },
                        );
                        Ok(())
                    }
                    "anti-ship strike" => {
                        unit_record.anti_ship.as_mut().unwrap().insert(
                            u.name.to_owned(),
                            AntiShipLoadout {
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
                                v_cruise: 246.,
                                v_attack: 277.5,
                                h_cruise: 9090.,
                                h_attack: 6706.,
                                standoff: None,
                                t_station: 0,
                                ldsd: false,
                                stores: u.payload.clone(),
                                self_escort: false,
                                sortie_rate: 6,
                                _airframe: u._type.to_owned(),
                                _name: u.name.to_owned(),
                                attributes: Vec::default(),
                            },
                        );
                        Ok(())
                    }
                    "awacs" => {
                        unit_record.awacs.as_mut().unwrap().insert(
                            u.name.to_owned(),
                            AWACSLoadout {
                                day: true,
                                night: true,
                                adverse_weather: true,
                                range: 500000.,
                                capability: 10,
                                firepower: 1,
                                v_cruise: 152.778,
                                v_attack: 138.889,
                                h_cruise: 9090.2,
                                h_attack: 9090.2,
                                t_station: 14400,
                                stores: u.payload.clone(),
                                sortie_rate: 12,
                                _airframe: u._type.to_owned(),
                                _name: u.name.to_owned(),
                                attributes: Vec::default(),
                            },
                        );
                        Ok(())
                    }
                    "refueling" => {
                        unit_record.aar.as_mut().unwrap().insert(
                            u.name.to_owned(),
                            AARLoadout {
                                day: true,
                                night: true,
                                adverse_weather: true,
                                range: 500000.,
                                capability: 10,
                                firepower: 1,
                                v_cruise: 200.,
                                v_attack: 150.,
                                h_cruise: 1828.,
                                h_attack: 1828.,
                                t_station: 10800,
                                stores: u.payload.clone(),
                                sortie_rate: 12,
                                _airframe: u._type.to_owned(),
                                _name: u.name.to_owned(),
                                attributes: Vec::default(),
                            },
                        );
                        Ok(())
                    }
                    "escort" => {
                        unit_record.escort.as_mut().unwrap().insert(
                            u.name.to_owned(),
                            EscortLoadout {
                                day: true,
                                night: true,
                                adverse_weather: true,
                                range: 500000.,
                                capability: 1,
                                firepower: 1,
                                v_cruise: 246.,
                                stores: u.payload.clone(),
                                sortie_rate: 12,
                                _airframe: u._type.to_owned(),
                                _name: u.name.to_owned(),
                                ldsd: true,
                                standoff: 100000.,
                                attributes: Vec::default(),
                            },
                        );
                        Ok(())
                    }
                    "intercept" => {
                        unit_record.intercept.as_mut().unwrap().insert(
                            u.name.to_owned(),
                            InterceptLoadout {
                                day: true,
                                night: true,
                                adverse_weather: true,
                                range: 500000.,
                                capability: 1,
                                firepower: 1,
                                stores: u.payload.clone(),
                                sortie_rate: 12,
                                _airframe: u._type.to_owned(),
                                _name: u.name.to_owned(),
                                ldsd: true,
                                attributes: Vec::default(),
                            },
                        );
                        Ok(())
                    }
                    "fighter sweep" => {
                        unit_record.sweep.as_mut().unwrap().insert(
                            u.name.to_owned(),
                            SweepLoadout {
                                day: true,
                                night: true,
                                adverse_weather: true,
                                range: 2000000.,
                                capability: 1,
                                firepower: 1,
                                v_cruise: 246.,
                                v_attack: 246.,
                                h_cruise: 9096.,
                                h_attack: 9096.,
                                t_station: 2400,
                                ldsd: false,
                                stores: u.payload.clone(),
                                sortie_rate: 6,
                                _airframe: u._type.to_owned(),
                                _name: u.name.to_owned(),
                                attributes: Vec::default(),
                                standoff: 50000.,
                            },
                        );
                        Ok(())
                    }
                    "sead" => {
                        unit_record.sead.as_mut().unwrap().insert(
                            u.name.to_owned(),
                            SEADLoadout {
                                day: true,
                                night: true,
                                adverse_weather: true,
                                range: 500000.,
                                capability: 1,
                                firepower: 1,
                                stores: u.payload.clone(),
                                sortie_rate: 12,
                                _airframe: u._type.to_owned(),
                                _name: u.name.to_owned(),
                                attributes: Vec::default(),
                                v_cruise: 246.,
                            },
                        );
                        Ok(())
                    }
                    "transport" => {
                        unit_record.transport.as_mut().unwrap().insert(
                            u.name.to_owned(),
                            TransportLoadout {
                                day: true,
                                night: true,
                                adverse_weather: true,
                                range: 500000.,
                                capability: 1,
                                firepower: 1,
                                stores: u.payload.clone(),
                                sortie_rate: 12,
                                _airframe: u._type.to_owned(),
                                _name: u.name.to_owned(),
                                attributes: Vec::default(),
                                v_cruise: 152.778,
                                v_attack: 138.889,
                                h_cruise: 7315.2,
                                h_attack: 7315.2,
                            },
                        );
                        Ok(())
                    }
                    _ => Err(anyhow!("Don't know how to handle loadout {}", u.name)),
                }
            })?;

        Ok(loadout)
    }
}

impl Editable for CAPLoadout {
    fn get_header() -> Vec<HeaderField> {
        let mut common = common_headers();
        common.extend(vec![
            HeaderField::new(
                "t_station",
                "Time on station (min)",
                FieldType::DurationMin,
                true,
            ),
            HeaderField::new("attributes", "Loadout Tags", FieldType::VecString, true),
        ]);
        common
    }

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

    fn can_reset_from_miz() -> bool {
        true
    }

    fn can_delete() -> bool
    where
        Self: Sized,
    {
        true
    }

    fn reset_all_from_miz(instance: &mut DCEInstance) -> Result<(), anyhow::Error> {
        let new_loadouts =
            LoadoutsInternal::from_loadouts(&Loadouts::new_from_mission(&instance.miz_env)?);

        instance.loadouts.cap = new_loadouts.cap;

        Ok(())
    }

    fn delete_by_name(instance: &mut DCEInstance, name: &str) -> Result<(), anyhow::Error> {
        let container = &mut instance.loadouts.cap;

        if let Some(index) = container.iter().position(|i| i._name == name) {
            container.remove(index);
            return Ok(());
        }

        Err(anyhow!("Didn't find {}", name))
    }
}

impl Editable for SweepLoadout {
    fn get_header() -> Vec<HeaderField> {
        let mut common = common_headers();
        common.extend(vec![
            HeaderField::new(
                "t_station",
                "Time on station (min)",
                FieldType::DurationMin,
                true,
            ),
            HeaderField::new("attributes", "Loadout Tags", FieldType::VecString, true),
            HeaderField::new("standoff", "Engage Range (nm)", FieldType::DistanceNM, true),
        ]);
        common
    }

    fn get_mut_by_name<'a>(instance: &'a mut DCEInstance, name: &str) -> &'a mut Self {
        instance
            .loadouts
            .sweep
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

    fn can_reset_from_miz() -> bool {
        true
    }

    fn can_delete() -> bool
    where
        Self: Sized,
    {
        true
    }

    fn reset_all_from_miz(instance: &mut DCEInstance) -> Result<(), anyhow::Error> {
        let new_loadouts =
            LoadoutsInternal::from_loadouts(&Loadouts::new_from_mission(&instance.miz_env)?);

        instance.loadouts.sweep = new_loadouts.sweep;

        Ok(())
    }

    fn delete_by_name(instance: &mut DCEInstance, name: &str) -> Result<(), anyhow::Error> {
        let container = &mut instance.loadouts.sweep;

        if let Some(index) = container.iter().position(|i| i._name == name) {
            container.remove(index);
            return Ok(());
        }

        Err(anyhow!("Didn't find {}", name))
    }
}

impl Editable for AARLoadout {
    fn get_header() -> Vec<HeaderField> {
        let mut common = common_headers();
        common.extend(vec![
            HeaderField::new(
                "t_station",
                "Time on station (min)",
                FieldType::DurationMin,
                true,
            ),
            HeaderField::new("attributes", "Loadout Tags", FieldType::VecString, true),
        ]);
        common
    }

    fn get_mut_by_name<'a>(instance: &'a mut DCEInstance, name: &str) -> &'a mut Self {
        instance
            .loadouts
            .aar
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
    fn can_reset_from_miz() -> bool {
        true
    }

    fn can_delete() -> bool
    where
        Self: Sized,
    {
        true
    }

    fn reset_all_from_miz(instance: &mut DCEInstance) -> Result<(), anyhow::Error> {
        let new_loadouts =
            LoadoutsInternal::from_loadouts(&Loadouts::new_from_mission(&instance.miz_env)?);

        instance.loadouts.aar = new_loadouts.aar;

        Ok(())
    }

    fn delete_by_name(instance: &mut DCEInstance, name: &str) -> Result<(), anyhow::Error> {
        let container = &mut instance.loadouts.aar;

        if let Some(index) = container.iter().position(|i| i._name == name) {
            container.remove(index);
            return Ok(());
        }

        Err(anyhow!("Didn't find {}", name))
    }
}

impl Editable for TransportLoadout {
    fn get_header() -> Vec<HeaderField> {
        let mut common = common_headers();
        common.extend(vec![HeaderField::new(
            "attributes",
            "Loadout Tags",
            FieldType::VecString,
            true,
        )]);
        common
    }

    fn get_mut_by_name<'a>(instance: &'a mut DCEInstance, name: &str) -> &'a mut Self {
        instance
            .loadouts
            .transport
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
    fn can_reset_from_miz() -> bool {
        true
    }

    fn can_delete() -> bool
    where
        Self: Sized,
    {
        true
    }

    fn reset_all_from_miz(instance: &mut DCEInstance) -> Result<(), anyhow::Error> {
        let new_loadouts =
            LoadoutsInternal::from_loadouts(&Loadouts::new_from_mission(&instance.miz_env)?);

        instance.loadouts.transport = new_loadouts.transport;

        Ok(())
    }

    fn delete_by_name(instance: &mut DCEInstance, name: &str) -> Result<(), anyhow::Error> {
        let container = &mut instance.loadouts.transport;

        if let Some(index) = container.iter().position(|i| i._name == name) {
            container.remove(index);
            return Ok(());
        }

        Err(anyhow!("Didn't find {}", name))
    }
}

impl Editable for AWACSLoadout {
    fn get_header() -> Vec<HeaderField> {
        let mut common = common_headers();
        common.extend(vec![
            HeaderField::new(
                "t_station",
                "Time on station (min)",
                FieldType::DurationMin,
                true,
            ),
            HeaderField::new("attributes", "Loadout Tags", FieldType::VecString, true),
        ]);
        common
    }

    fn get_mut_by_name<'a>(instance: &'a mut DCEInstance, name: &str) -> &'a mut Self {
        instance
            .loadouts
            .awacs
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

    fn can_reset_from_miz() -> bool {
        true
    }

    fn can_delete() -> bool
    where
        Self: Sized,
    {
        true
    }

    fn reset_all_from_miz(instance: &mut DCEInstance) -> Result<(), anyhow::Error> {
        let new_loadouts =
            LoadoutsInternal::from_loadouts(&Loadouts::new_from_mission(&instance.miz_env)?);

        instance.loadouts.awacs = new_loadouts.awacs;

        Ok(())
    }

    fn delete_by_name(instance: &mut DCEInstance, name: &str) -> Result<(), anyhow::Error> {
        let container = &mut instance.loadouts.awacs;

        if let Some(index) = container.iter().position(|i| i._name == name) {
            container.remove(index);
            return Ok(());
        }

        Err(anyhow!("Didn't find {}", name))
    }
}

impl Editable for StrikeLoadout {
    fn get_header() -> Vec<HeaderField> {
        let mut common = common_headers();
        common.extend(vec![
            HeaderField::new(
                "weapon_type",
                "Weapon Type",
                FieldType::FixedEnum(vec![
                    "Bombs".into(),
                    "Rockets".into(),
                    "ASM".into(),
                    "Guided bombs".into(),
                ]),
                true,
            ),
            // attackType: "Dive"
            HeaderField::new(
                "expend",
                "Expend Quantity",
                FieldType::FixedEnum(vec!["All".into(), "Auto".into()]),
                true,
            ),
            HeaderField::new(
                "support",
                "Support Required",
                FieldType::NestedEditable(Support::get_header()),
                true,
            ),
            HeaderField::new("attributes", "Loadout Tags", FieldType::VecString, true),
        ]);
        common
    }
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

    fn can_reset_from_miz() -> bool {
        true
    }

    fn can_delete() -> bool
    where
        Self: Sized,
    {
        true
    }

    fn reset_all_from_miz(instance: &mut DCEInstance) -> Result<(), anyhow::Error> {
        let new_loadouts =
            LoadoutsInternal::from_loadouts(&Loadouts::new_from_mission(&instance.miz_env)?);

        instance.loadouts.strike = new_loadouts.strike;

        Ok(())
    }
    fn delete_by_name(instance: &mut DCEInstance, name: &str) -> Result<(), anyhow::Error> {
        let container = &mut instance.loadouts.strike;

        if let Some(index) = container.iter().position(|i| i._name == name) {
            container.remove(index);
            return Ok(());
        }

        Err(anyhow!("Didn't find {}", name))
    }
}

impl Editable for AntiShipLoadout {
    fn get_header() -> Vec<HeaderField> {
        let mut common = common_headers();
        common.extend(vec![
            HeaderField::new(
                "weapon_type",
                "Weapon Type",
                FieldType::FixedEnum(vec![
                    "Bombs".into(),
                    "Rockets".into(),
                    "ASM".into(),
                    "Guided bombs".into(),
                ]),
                true,
            ),
            // attackType: "Dive"
            HeaderField::new(
                "expend",
                "Expend Quantity",
                FieldType::FixedEnum(vec!["All".into(), "Auto".into()]),
                true,
            ),
            HeaderField::new("attributes", "Loadout Tags", FieldType::VecString, true),
        ]);
        common
    }
    fn get_mut_by_name<'a>(instance: &'a mut DCEInstance, name: &str) -> &'a mut Self {
        instance
            .loadouts
            .antiship
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

    fn can_reset_from_miz() -> bool {
        true
    }

    fn can_delete() -> bool
    where
        Self: Sized,
    {
        true
    }

    fn reset_all_from_miz(instance: &mut DCEInstance) -> Result<(), anyhow::Error> {
        let new_loadouts =
            LoadoutsInternal::from_loadouts(&Loadouts::new_from_mission(&instance.miz_env)?);

        instance.loadouts.antiship = new_loadouts.antiship;

        Ok(())
    }

    fn delete_by_name(instance: &mut DCEInstance, name: &str) -> Result<(), anyhow::Error> {
        let container = &mut instance.loadouts.antiship;

        if let Some(index) = container.iter().position(|i| i._name == name) {
            container.remove(index);
            return Ok(());
        }

        Err(anyhow!("Didn't find {}", name))
    }
}

impl Editable for EscortLoadout {
    fn get_header() -> Vec<HeaderField> {
        vec![
            HeaderField::new("_name", "Name", FieldType::String, true),
            HeaderField::new("_airframe", "AirFrame", FieldType::String, false),
            HeaderField::new("day", "Day", FieldType::Bool, true),
            HeaderField::new("night", "Night", FieldType::Bool, true),
            HeaderField::new("adverse_weather", "Adverse Weather", FieldType::Bool, true),
            HeaderField::new("range", "Range (nm)", FieldType::DistanceNM, true),
            HeaderField::new("standoff", "Engagement Range", FieldType::DistanceNM, true),
            HeaderField::new("capability", "Capability", FieldType::Int, true),
            HeaderField::new("firepower", "Firepower", FieldType::Int, true),
            HeaderField::new("attributes", "Loadout Tags", FieldType::VecString, true),
        ]
    }
    fn get_mut_by_name<'a>(instance: &'a mut DCEInstance, name: &str) -> &'a mut Self {
        instance
            .loadouts
            .escort
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

    fn can_reset_from_miz() -> bool {
        true
    }

    fn can_delete() -> bool
    where
        Self: Sized,
    {
        true
    }

    fn reset_all_from_miz(instance: &mut DCEInstance) -> Result<(), anyhow::Error> {
        let new_loadouts =
            LoadoutsInternal::from_loadouts(&Loadouts::new_from_mission(&instance.miz_env)?);

        instance.loadouts.escort = new_loadouts.escort;

        Ok(())
    }

    fn delete_by_name(instance: &mut DCEInstance, name: &str) -> Result<(), anyhow::Error> {
        let container = &mut instance.loadouts.escort;

        if let Some(index) = container.iter().position(|i| i._name == name) {
            container.remove(index);
            return Ok(());
        }

        Err(anyhow!("Didn't find {}", name))
    }
}

impl Editable for SEADLoadout {
    fn get_header() -> Vec<HeaderField> {
        vec![
            HeaderField::new("_name", "Name", FieldType::String, true),
            HeaderField::new("_airframe", "AirFrame", FieldType::String, false),
            HeaderField::new("day", "Day", FieldType::Bool, true),
            HeaderField::new("night", "Night", FieldType::Bool, true),
            HeaderField::new("adverse_weather", "Adverse Weather", FieldType::Bool, true),
            HeaderField::new("range", "Range (nm)", FieldType::DistanceNM, true),
            HeaderField::new("capability", "Capability", FieldType::Int, true),
            HeaderField::new("firepower", "Firepower", FieldType::Int, true),
            HeaderField::new("attributes", "Loadout Tags", FieldType::VecString, true),
        ]
    }
    fn get_mut_by_name<'a>(instance: &'a mut DCEInstance, name: &str) -> &'a mut Self {
        instance
            .loadouts
            .sead
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

    fn can_reset_from_miz() -> bool {
        true
    }

    fn can_delete() -> bool
    where
        Self: Sized,
    {
        true
    }

    fn reset_all_from_miz(instance: &mut DCEInstance) -> Result<(), anyhow::Error> {
        let new_loadouts =
            LoadoutsInternal::from_loadouts(&Loadouts::new_from_mission(&instance.miz_env)?);

        instance.loadouts.sead = new_loadouts.sead;

        Ok(())
    }

    fn delete_by_name(instance: &mut DCEInstance, name: &str) -> Result<(), anyhow::Error> {
        let container = &mut instance.loadouts.sead;

        if let Some(index) = container.iter().position(|i| i._name == name) {
            container.remove(index);
            return Ok(());
        }

        Err(anyhow!("Didn't find {}", name))
    }
}

impl Editable for InterceptLoadout {
    fn get_header() -> Vec<HeaderField> {
        vec![
            HeaderField::new("_name", "Name", FieldType::String, true),
            HeaderField::new("_airframe", "AirFrame", FieldType::String, false),
            HeaderField::new("day", "Day", FieldType::Bool, true),
            HeaderField::new("night", "Night", FieldType::Bool, true),
            HeaderField::new("adverse_weather", "Adverse Weather", FieldType::Bool, true),
            HeaderField::new("range", "Range (nm)", FieldType::DistanceNM, true),
            HeaderField::new("capability", "Capability", FieldType::Int, true),
            HeaderField::new("firepower", "Firepower", FieldType::Int, true),
            HeaderField::new("attributes", "Loadout Tags", FieldType::VecString, true),
        ]
    }
    fn get_mut_by_name<'a>(instance: &'a mut DCEInstance, name: &str) -> &'a mut Self {
        instance
            .loadouts
            .intercept
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

    fn can_reset_from_miz() -> bool {
        true
    }

    fn can_delete() -> bool {
        true
    }

    fn reset_all_from_miz(instance: &mut DCEInstance) -> Result<(), anyhow::Error> {
        let new_loadouts =
            LoadoutsInternal::from_loadouts(&Loadouts::new_from_mission(&instance.miz_env)?);

        instance.loadouts.intercept = new_loadouts.intercept;

        Ok(())
    }

    fn delete_by_name(instance: &mut DCEInstance, name: &str) -> Result<(), anyhow::Error> {
        let container = &mut instance.loadouts.intercept;

        if let Some(index) = container.iter().position(|i| i._name == name) {
            container.remove(index);
            return Ok(());
        }

        Err(anyhow!("Didn't find {}", name))
    }
}

#[cfg(test)]
mod tests {
    use crate::{miz_environment::MizEnvironment, serde_utils::LuaFileBased, NewFromMission};

    use super::Loadouts;

    #[test]
    fn from_miz() {
        let miz = MizEnvironment::from_miz("test_resources\\base_mission.miz".into()).unwrap();
        let loadouts = Loadouts::new_from_mission(&miz).unwrap();

        loadouts
            .to_lua_file("..\\target\\db_loadouts.lua".into(), "db_loadouts".into())
            .unwrap();
    }
}
