use bevy_reflect::{FromReflect, Reflect};
use serde::{Deserialize, Serialize};
use tables::{FieldType, HeaderField, TableHeader};

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
