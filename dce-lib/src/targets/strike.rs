use bevy_reflect::{FromReflect, Reflect};
use serde::{Deserialize, Serialize};
use tables::{FieldType, HeaderField, TableHeader};

use crate::{
    editable::{Editable, ValidationError, ValidationResult},
    DCEInstance,
};

use super::TargetFirepower;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Reflect, FromReflect)]
pub struct Strike {
    pub priority: u32,
    #[serde(default)]
    pub text: String,
    #[serde(default)]
    pub inactive: bool,
    pub firepower: TargetFirepower,
    #[serde(default = "default_class")]
    pub class: String,
    #[serde(rename = "name")]
    pub class_template: Option<String>,
    pub elements: Option<Vec<StrikeElement>>,
    #[serde(default)]
    pub _name: String,
    #[serde(default)]
    pub _side: String,
}

fn default_class() -> String {
    "static".to_string()
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Reflect, FromReflect)]
#[serde(untagged)]
pub enum StrikeElement {
    FixedCoord(StrikeFixedCoordTarget),
    NamedStatic(StrikeNamedStaticTarget),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Reflect, FromReflect)]
pub struct StrikeFixedCoordTarget {
    pub name: String,
    pub x: f64,
    pub y: f64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Reflect, FromReflect)]
pub struct StrikeNamedStaticTarget {
    pub name: String,
}

impl TableHeader for Strike {
    fn get_header() -> Vec<HeaderField> {
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
                display: "Inactive".into(),
                field: "inactive".into(),
                type_: FieldType::Bool,
                editable: true,
            },
        ]
    }
}

impl Editable for Strike {
    fn get_mut_by_name<'a>(instance: &'a mut DCEInstance, name: &str) -> &'a mut Self {
        instance
            .target_list
            .strike
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
        if let Some(vg_name) = self.class_template.clone() {
            match self.class.as_str() {
                "vehicle" => {
                    if let None = instance
                        .mission
                        .get_vehicle_groups()
                        .iter()
                        .find(|g| g.name == vg_name)
                    {
                        errors.push(ValidationError::new(
                            "class_template",
                            "Target group name",
                            "Target group must be a vehicle group name defined in base_mission.miz",
                        ));
                    }
                }
                "ship" => {
                    if let None = instance
                        .mission
                        .get_ship_groups()
                        .iter()
                        .find(|g| g.name == vg_name)
                    {
                        errors.push(ValidationError::new(
                            "class_template",
                            "Target group name",
                            "Target group must be a ship group name defined in base_mission.miz",
                        ));
                    }
                }
                _ => {
                    errors.push(ValidationError::new(
                        "class",
                        "Target Class",
                        "Target class must be vehicle or ship",
                    ));
                }
            }
        }

        if errors.is_empty() {
            return ValidationResult::Pass;
        }
        ValidationResult::Fail(errors)
    }
}
