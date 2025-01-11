use crate::loadouts::common_headers;
use crate::loadouts::Support;
use crate::mission::PlaneUnit;
use crate::Loadouts;
use crate::{
    editable::{Editable, FieldType, HeaderField, NestedEditable, ValidationResult},
    loadouts_internal::LoadoutsInternal,
    mission::Payload,
    DCEInstance, NewFromMission,
};
use anyhow::{anyhow, Ok};
use bevy_reflect::{FromReflect, Reflect};
use serde::{Deserialize, Serialize};

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
    pub ingress: Option<f64>,
    pub egress: Option<f64>,
    #[serde(rename = "MaxAttackOffset")]
    pub max_attack_offset: Option<f64>,
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

impl StrikeLoadout {
    pub fn from_unit(unit: &PlaneUnit) -> StrikeLoadout {
        let mut loadout = StrikeLoadout {
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
            ingress: None,
            egress: None,
            max_attack_offset: None,
            t_station: 0,
            ldsd: true,
            stores: unit.payload.clone(),
            self_escort: false,
            sortie_rate: 6,
            _airframe: unit._type.to_owned(),
            _name: unit.name.to_owned(),
            attributes: Vec::default(),
        };

        // apply overrides based on rules in loadout_rules.md
        if unit.name.contains(" day ") | unit.name.ends_with(" day") {
            loadout.night = false;
            loadout.adverse_weather = false;
        }

        if unit.name.contains(" lgb ") | unit.name.ends_with(" lgb") {
            loadout.adverse_weather = false;
            loadout.weapon_type = "Guided bombs".into();
            loadout.capability = 2;
            loadout.expend = "Auto".into();
            loadout.standoff = Some(15001.);
            loadout.egress = Some(10000.);
            loadout.attributes.push("precise".into());
        }

        if unit.name.contains(" cbu ") | unit.name.ends_with(" cbu") {
            loadout.attributes.push("soft".into());
            loadout.attributes.push("parked_aircraft".into());
        }

        if unit.name.contains(" jdam ") | unit.name.ends_with(" jdam") {
            loadout.weapon_type = "Guided bombs".into();
            loadout.capability = 2;
            loadout.standoff = Some(9000.);
            loadout.expend = "Auto".into();
            loadout.attributes.push("precise".into());
            loadout.sortie_rate = 3;
        }

        if unit.name.contains(" rockets ") | unit.name.ends_with(" rockets") {
            loadout.adverse_weather = false;
            loadout.weapon_type = "Rockets".into();
            loadout.attributes.push("soft".into());
            loadout.attributes.push("parked_aircraft".into());
        }

        if unit.name.contains(" saturation ") | unit.name.ends_with(" saturation") {
            loadout.capability = 10;
            loadout.firepower = 4;
            loadout.sortie_rate = 1;
            loadout.range = 1000000.;
            loadout.weapon_type = "ASM".into();
            loadout.standoff = Some(150000.);
            loadout.ingress = Some(50000.);
            loadout.egress = Some(50000.);
            loadout.max_attack_offset = Some(60.);
            loadout.support.sead = false;
            loadout.attributes.push("saturation".into());
        }

        loadout
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
