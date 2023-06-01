use bevy_reflect::{FromReflect, Reflect};
use serde::{Deserialize, Serialize};

use super::TargetFirepower;
use crate::{
    editable::{Editable, FieldType, HeaderField, ValidationError, ValidationResult},
    target_list::TargetList,
    target_list_internal::TargetListInternal,
    DCEInstance, NewFromMission,
};
use anyhow::anyhow;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Reflect, FromReflect)]
pub struct Intercept {
    pub priority: u32,
    #[serde(default)]
    pub text: String,
    pub base: String,
    #[serde(default)]
    pub inactive: bool,
    pub firepower: TargetFirepower,
    #[serde(default)]
    pub _name: String,
    #[serde(default)]
    pub _side: String,
    #[serde(default)]
    pub _firepower_min: u32,
    #[serde(default)]
    pub _firepower_max: u32,
}

impl Editable for Intercept {
    fn get_header() -> Vec<HeaderField> {
        vec![
            HeaderField::new("text", "Display Text", FieldType::String, true),
            HeaderField::new("_side", "Side", FieldType::String, false),
            HeaderField::new("priority", "Priority", FieldType::Int, true),
            HeaderField::new("inactive", "Inactive", FieldType::Bool, true),
        ]
    }
    fn get_mut_by_name<'a>(instance: &'a mut DCEInstance, name: &str) -> &'a mut Self {
        instance
            .target_list
            .intercept
            .iter_mut()
            .find(|s| s._name == name)
            .unwrap()
    }

    fn get_name(&self) -> String {
        self._name.to_string()
    }

    fn validate(&self, _: &DCEInstance) -> ValidationResult {
        let mut errors = Vec::default();

        if self._side != "blue" && self._name == "red" {
            errors.push(ValidationError::new(
                "_side",
                "Target Side",
                "Side must be blue or red",
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
            TargetListInternal::from_target_list(&TargetList::new_from_mission(&instance.mission)?);

        instance.target_list.intercept = new_target_list.intercept;

        Ok(())
    }

    fn delete_by_name(instance: &mut DCEInstance, name: &str) -> Result<(), anyhow::Error> {
        let container = &mut instance.target_list.intercept;

        if let Some(index) = container.iter().position(|i| i._name == name) {
            container.remove(index);
            return Ok(());
        }

        Err(anyhow!("Didn't find {}", name))
    }
}
