use bevy_reflect::{FromReflect, Reflect};
use serde::{Deserialize, Serialize};

use crate::{
    editable::{Editable, FieldType, HeaderField, ValidationError, ValidationResult},
    DCEInstance,
};

use super::TargetFirepower;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Reflect, FromReflect)]
pub struct AWACS {
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
    pub _firepower_min: u32,
    #[serde(default)]
    pub _firepower_max: u32,
    // #[serde(default)]
    // pub attributes: Option<Vec<String>>,
}

impl Editable for AWACS {
    fn get_header() -> Vec<HeaderField> {
        vec![
            HeaderField::new("text", "Display Text", FieldType::String, true),
            HeaderField::new("_side", "Side", FieldType::String, false),
            HeaderField::new("priority", "Priority", FieldType::Int, true),
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
        ]
    }
    fn get_mut_by_name<'a>(instance: &'a mut DCEInstance, name: &str) -> &'a mut Self {
        instance
            .target_list
            .awacs
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
        if instance.mission.get_zone_by_name(&self.ref_point).is_err() {
            errors.push(ValidationError::new(
                "ref_point",
                "AWACS Reference Zone",
                "AWACS reference zone must exist in base_mission.miz",
            ));
        }
        if errors.is_empty() {
            return ValidationResult::Pass;
        }
        ValidationResult::Fail(errors)
    }
}
