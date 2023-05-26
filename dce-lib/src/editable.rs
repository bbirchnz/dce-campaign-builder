use bevy_reflect::Struct;
use itertools::Itertools;
use log::warn;

use crate::{trigger::Actions, DCEInstance};
use anyhow::anyhow;
use chrono::{NaiveTime, Timelike};

pub trait Editable {
    fn get_name(&self) -> String;

    fn validate(&self, instance: &DCEInstance) -> ValidationResult;

    fn get_mut_by_name<'a>(instance: &'a mut DCEInstance, name: &str) -> &'a mut Self;
    fn get_header() -> Vec<HeaderField>;
}

// pub trait TableHeader {

#[derive(PartialEq, Debug)]
pub struct HeaderField {
    pub field: String,
    pub display: String,
    pub type_: FieldType,
    pub editable: bool,
}

const METERS_TO_FEET: f64 = 3.281;
const FEET_TO_METERS: f64 = 1. / METERS_TO_FEET;

const MS_TO_KNOTS: f64 = 3.6 / 1.852;
const KNOTS_TO_MS: f64 = 1. / MS_TO_KNOTS;

const NM_TO_METERS: f64 = 1852.;
const METERS_TO_NM: f64 = 1. / NM_TO_METERS;

#[derive(PartialEq, Clone)]
pub enum ValidationResult {
    Pass,
    Fail(Vec<ValidationError>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ValidationError {
    pub field_name: String,
    pub display_name: String,
    pub error: String,
}

impl ValidationError {
    pub fn new(field_name: &str, display_name: &str, error: &str) -> ValidationError {
        ValidationError {
            field_name: field_name.to_owned(),
            display_name: display_name.to_owned(),
            error: error.to_owned(),
        }
    }
}

impl HeaderField {
    pub fn new(field: &str, display: &str, type_: FieldType, editable: bool) -> HeaderField {
        HeaderField {
            field: field.to_owned(),
            display: display.to_owned(),
            type_,
            editable,
        }
    }

    /// Attempt to get a value as a string array. For most types this is a wrapper
    /// around `get_value_string()` but comes into its own when used with `TriggerActions`
    pub fn get_value_stringvec(&self, item: &dyn Struct) -> Vec<String> {
        match self.type_ {
            FieldType::TriggerActions => {
                let actions = item
                    .field(&self.field)
                    .expect(&format!("Field {} should exist", &self.field))
                    .downcast_ref::<Actions>()
                    .expect(&format!("Failed to get field {} as Actions", &self.field));
                match actions {
                    Actions::One(action) => vec![action.to_owned()],
                    Actions::Many(actions) => actions.to_owned(),
                }
            }
            _ => {
                warn!(
                    "get_value_stringvec called with a {:?} that doesn't need it",
                    self
                );
                vec![self.get_value_string(item)]
            }
        }
    }

    pub fn set_value_from_stringvec(
        &self,
        item: &mut dyn Struct,
        values: &[String],
    ) -> Result<(), anyhow::Error> {
        match self.type_ {
            FieldType::TriggerActions => {
                let action = Actions::Many(values.iter().map(|v| v.to_string()).collect_vec());
                item.field_mut(&self.field)
                    .ok_or(anyhow!("Couldn't get field {}", &self.field))?
                    .apply(&action);
                Ok(())
            }
            _ => Err(anyhow!(
                "set_value_from_stringvec called with a {:?} that doesn't need it",
                self
            )),
        }
    }

    pub fn get_value_string(&self, item: &dyn Struct) -> String {
        match self.type_ {
            FieldType::String => item
                .field(&self.field)
                .expect(&format!("Field {} should exist", &self.field))
                .downcast_ref::<String>()
                .expect(&format!("Failed to get field {} as String", &self.field))
                .to_string(),
            FieldType::Float(func) => {
                let value = item
                    .field(&self.field)
                    .expect(&format!("Field {} should exist", &self.field))
                    .downcast_ref::<f64>()
                    .expect(&format!("Failed to get field {} as f64", &self.field));
                func(*value)
            } // .to_string(),
            FieldType::Int => item
                .field(&self.field)
                .expect(&format!("Field {} should exist", &self.field))
                .downcast_ref::<u32>()
                .expect(&format!("Failed to get field {} as u32", &self.field))
                .to_string(),
            FieldType::Enum => "".into(),
            FieldType::Debug => {
                let v = item.field(&self.field).unwrap();
                format!("{:?}", v)
            }
            FieldType::IntTime => {
                let seconds_since_midnight = item
                    .field(&self.field)
                    .expect(&format!("Field {} should exist", &self.field))
                    .downcast_ref::<u32>()
                    .expect(&format!("Failed to get field {} as u32", &self.field));
                let time = chrono::NaiveTime::from_num_seconds_from_midnight_opt(
                    *seconds_since_midnight,
                    0,
                )
                .expect("A valid number of seconds since midnight");

                return time.format("%H:%M:%S").to_string();
            }
            FieldType::Bool => {
                let val = item
                    .field(&self.field)
                    .expect(&format!("Field {} should exist", &self.field))
                    .downcast_ref::<bool>()
                    .expect(&format!("Failed to get field {} as bool", &self.field));
                if *val {
                    "true".into()
                } else {
                    "false".into()
                }
            }
            FieldType::AltitudeFeet => {
                let meters = item
                    .field(&self.field)
                    .expect(&format!("Field {} should exist", &self.field))
                    .downcast_ref::<f64>()
                    .expect(&format!("Failed to get field {} as f64", &self.field));
                format!("{:.0}", meters * METERS_TO_FEET)
            }
            FieldType::SpeedKnotsTAS => {
                let ms = item
                    .field(&self.field)
                    .expect(&format!("Field {} should exist", &self.field))
                    .downcast_ref::<f64>()
                    .expect(&format!("Failed to get field {} as f64", &self.field));
                format!("{:.0}", ms * MS_TO_KNOTS)
            }
            FieldType::DistanceNM => {
                let meters = item
                    .field(&self.field)
                    .expect(&format!("Field {} should exist", &self.field))
                    .downcast_ref::<f64>()
                    .expect(&format!("Failed to get field {} as f64", &self.field));
                format!("{:.0}", meters * METERS_TO_NM)
            }
            FieldType::DurationMin => {
                let seconds = item
                    .field(&self.field)
                    .expect(&format!("Field {} should exist", &self.field))
                    .downcast_ref::<u32>()
                    .expect(&format!("Failed to get field {} as f64", &self.field));
                format!("{:.0}", seconds / 60)
            }
            FieldType::TriggerActions => {
                panic!("Shouldn't get here, TriggerAction should use stringvec methods")
            }
        }
    }

    pub fn set_value_fromstr(
        &self,
        item: &mut dyn Struct,
        value: &str,
    ) -> Result<(), anyhow::Error> {
        match self.type_ {
            FieldType::String => {
                item.field_mut(&self.field)
                    .ok_or(anyhow!("Couldn't get field {}", &self.field))?
                    .apply(&value.to_owned());
            }
            FieldType::Float(_) => {
                item.field_mut(&self.field)
                    .ok_or(anyhow!("Couldn't get field {}", &self.field))?
                    .apply(&value.parse::<f64>()?);
            }
            FieldType::Int => {
                item.field_mut(&self.field)
                    .ok_or(anyhow!("Couldn't get field {}", &self.field))?
                    .apply(&value.parse::<u32>()?);
            }
            FieldType::Enum => todo!(),
            FieldType::Debug => todo!(),
            FieldType::IntTime => {
                let attempt_hms = NaiveTime::parse_from_str(value, "%H:%M:%S");
                let time = match attempt_hms {
                    Ok(t) => t,
                    Err(_) => NaiveTime::parse_from_str(value, "%H:%M")
                        .expect("Expected HH:MM:SS or HH:MM"),
                };
                item.field_mut(&self.field)
                    .ok_or(anyhow!("Couldn't get field {}", &self.field))?
                    .apply(&time.num_seconds_from_midnight());
            }
            FieldType::Bool => {
                let selected = value == "true";

                item.field_mut(&self.field)
                    .ok_or(anyhow!("Couldn't get field {}", &self.field))?
                    .apply(&selected);
            }
            FieldType::AltitudeFeet => {
                let meters = value.parse::<f64>()? * FEET_TO_METERS;

                item.field_mut(&self.field)
                    .ok_or(anyhow!("Couldn't get field {}", &self.field))?
                    .apply(&meters);
            }
            FieldType::SpeedKnotsTAS => {
                let ms = value.parse::<f64>()? * KNOTS_TO_MS;

                item.field_mut(&self.field)
                    .ok_or(anyhow!("Couldn't get field {}", &self.field))?
                    .apply(&ms);
            }
            FieldType::DistanceNM => {
                let meters = value.parse::<f64>()? * NM_TO_METERS;

                item.field_mut(&self.field)
                    .ok_or(anyhow!("Couldn't get field {}", &self.field))?
                    .apply(&meters);
            }
            FieldType::DurationMin => {
                let seconds = value.parse::<u32>()? * 60;

                item.field_mut(&self.field)
                    .ok_or(anyhow!("Couldn't get field {}", &self.field))?
                    .apply(&seconds);
            }
            FieldType::TriggerActions => {
                item.field_mut(&self.field)
                    .ok_or(anyhow!("Couldn't get field {}", &self.field))?
                    .apply(&value.to_owned());
            }
        };
        Ok(())
    }
}

#[derive(PartialEq, Debug)]
pub enum FieldType {
    String,
    Float(fn(f64) -> String),
    Int,
    Enum,
    Debug,
    IntTime,
    Bool,
    AltitudeFeet,
    SpeedKnotsTAS,
    DistanceNM,
    DurationMin,
    TriggerActions,
}
