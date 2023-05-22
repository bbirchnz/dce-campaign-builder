use bevy_reflect::{FromReflect, Reflect};
use serde::{Deserialize, Serialize};
use tables::{FieldType, HeaderField, TableHeader};

use crate::{
    editable::{Editable, ValidationError, ValidationResult},
    DCEInstance,
};

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
    // #[serde(default)]
    // pub attributes: Option<Vec<String>>,
}

impl TableHeader for CAP {
    fn get_header() -> Vec<tables::HeaderField> {
        vec![
            HeaderField {
                field: "text".into(),
                display: "Display Text".into(),
                type_: FieldType::String,
                editable: true,
            },
            HeaderField {
                field: "_side".into(),
                display: "Side".into(),
                type_: FieldType::String,
                editable: false,
            },
            HeaderField {
                field: "priority".into(),
                display: "Priority".into(),
                type_: FieldType::Int,
                editable: true,
            },
            HeaderField {
                field: "firepower".into(),
                display: "Req Firepower".into(),
                type_: FieldType::Debug,
                editable: false,
            },
            HeaderField {
                field: "axis".into(),
                display: "Axis".into(),
                type_: FieldType::Float(|v| format!("{:.0}", v)),
                editable: true,
            },
            HeaderField {
                field: "radius".into(),
                display: "Radius".into(),
                type_: FieldType::Float(|v| format!("{:.0}", v)),
                editable: true,
            },
            HeaderField {
                display: "Inactive".into(),
                field: "inactive".into(),
                type_: FieldType::Bool,
                editable: true,
            },
        ]
    }
}

impl Editable for CAP {
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
        if let Err(_) = instance.mission.get_zone_by_name(&self.ref_point) {
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
}
