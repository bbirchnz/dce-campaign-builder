use bevy_reflect::{FromReflect, Reflect};
use serde::{Deserialize, Serialize};

use crate::{
    editable::{
        Editable, FieldType, HeaderField, NestedEditable, ValidationError, ValidationResult,
    },
    target_list::TargetList,
    target_list_internal::TargetListInternal,
    DCEInstance, NewFromMission,
};
use anyhow::anyhow;

use super::TargetFirepower;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Reflect, FromReflect)]
pub struct CAP {
    pub priority: u32,
    #[serde(rename = "refpoint")]
    pub ref_point: String,
    pub radius: f64,
    pub axis: f64,
    pub text: String,
    pub inactive: bool,
    pub firepower: TargetFirepower,
    #[serde(default)]
    pub _name: String,
    #[serde(default)]
    pub _side: String,
    #[serde(default)]
    pub attributes: Vec<String>,
}

impl Editable for CAP {
    fn get_header() -> Vec<HeaderField> {
        vec![
            HeaderField::new("text", "Display Text", FieldType::String, true),
            HeaderField::new("_side", "Side", FieldType::String, false),
            HeaderField::new("priority", "Priority", FieldType::Int, true),
            HeaderField::new(
                "firepower",
                "Firepower Required",
                FieldType::NestedEditable(TargetFirepower::get_header()),
                true,
            ),
            HeaderField::new(
                "axis",
                "Axis",
                FieldType::Float(|v| format!("{:.0}", v)),
                true,
            ),
            HeaderField::new(
                "radius",
                "Radius",
                FieldType::Float(|v| format!("{:.0}", v)),
                true,
            ),
            HeaderField::new("inactive", "Inactive", FieldType::Bool, true),
            HeaderField::new("attributes", "Loadout Tags", FieldType::VecString, true),
        ]
    }
    fn get_mut_by_name<'a>(instance: &'a mut DCEInstance, name: &str) -> &'a mut Self {
        instance
            .target_list
            .cap
            .iter_mut()
            .find(|s| s._name == name)
            .unwrap()
    }

    fn get_name(&self) -> String {
        self._name.to_string()
    }

    fn validate(&self, instance: &DCEInstance) -> ValidationResult {
        let mut errors = Vec::default();

        if self._side != "blue" && self._name == "red" {
            errors.push(ValidationError::new(
                "_side",
                "Target Side",
                "Side must be blue or red",
            ));
        }

        if let ValidationResult::Fail(mut firepower_errors) =
            TargetFirepower::validate(&self.firepower, instance)
        {
            errors.append(&mut firepower_errors)
        }

        if instance
            .miz_env
            .mission
            .get_zone_by_name(&self.ref_point)
            .is_err()
        {
            errors.push(ValidationError::new(
                "ref_point",
                "CAP Reference Zone",
                "CAP reference zone must exist in base_mission.miz",
            ));
        }
        if errors.is_empty() {
            return ValidationResult::Pass;
        }
        ValidationResult::Fail(errors)
    }

    fn can_reset_from_miz() -> bool {
        true
    }

    fn reset_all_from_miz(instance: &mut DCEInstance) -> Result<(), anyhow::Error> {
        let new_target_list =
            TargetListInternal::from_target_list(&TargetList::new_from_mission(&instance.miz_env)?);

        instance.target_list.cap = new_target_list.cap;

        Ok(())
    }

    fn delete_by_name(instance: &mut DCEInstance, name: &str) -> Result<(), anyhow::Error> {
        let container = &mut instance.target_list.cap;

        if let Some(index) = container.iter().position(|i| i._name == name) {
            container.remove(index);
            return Ok(());
        }

        Err(anyhow!("Didn't find {}", name))
    }
}
