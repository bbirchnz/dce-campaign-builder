use bevy_reflect::{Reflect, Struct};

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

#[derive(PartialEq, Debug, Clone)]
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
                    .unwrap_or_else(|| panic!("Field {} should exist", &self.field))
                    .downcast_ref::<Actions>()
                    .unwrap_or_else(|| panic!("Failed to get field {} as Actions", &self.field));
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
        values: Vec<String>,
    ) -> Result<(), anyhow::Error> {
        match self.type_ {
            FieldType::TriggerActions => {
                let action = Actions::Many(values);
                // have to set this to `Actions::One`, then back to proper result.
                // this due to the `Reflect` behaviour for lists changing existing values
                // not replacing the entire list
                item.field_mut(&self.field)
                    .ok_or(anyhow!("Couldn't get field {}", &self.field))?
                    .apply(&Actions::One("".into()));

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
            FieldType::String => get_string(item, &self.field),
            FieldType::Float(func) => {
                let value = get_f64(item, &self.field);
                func(value)
            }
            FieldType::Int => get_u32(item, &self.field).to_string(),
            FieldType::Enum => "".into(),
            FieldType::Debug => {
                let v = item.field(&self.field).unwrap();
                format!("{:?}", v)
            }
            FieldType::IntTime => {
                let seconds_since_midnight = get_u32(item, &self.field);
                let time = chrono::NaiveTime::from_num_seconds_from_midnight_opt(
                    seconds_since_midnight,
                    0,
                )
                .expect("A valid number of seconds since midnight");

                return time.format("%H:%M:%S").to_string();
            }
            FieldType::Bool => {
                let val = item
                    .field(&self.field)
                    .unwrap_or_else(|| panic!("Field {} should exist", &self.field))
                    .downcast_ref::<bool>()
                    .unwrap_or_else(|| panic!("Failed to get field {} as bool", &self.field));
                if *val {
                    "true".into()
                } else {
                    "false".into()
                }
            }
            FieldType::AltitudeFeet => {
                let meters = get_f64(item, &self.field);
                format!("{:.0}", meters * METERS_TO_FEET)
            }
            FieldType::SpeedKnotsTAS => {
                let ms = get_f64(item, &self.field);
                format!("{:.0}", ms * MS_TO_KNOTS)
            }
            FieldType::DistanceNM => {
                let meters = get_f64(item, &self.field);
                format!("{:.0}", meters * METERS_TO_NM)
            }
            FieldType::DurationMin => {
                let seconds = get_u32(item, &self.field);
                format!("{:.0}", seconds / 60)
            }
            FieldType::TriggerActions => {
                // Just return the number of actions as a string. Used in table
                let val = item
                    .field(&self.field)
                    .unwrap_or_else(|| panic!("Field {} should exist", &self.field))
                    .downcast_ref::<Actions>()
                    .unwrap_or_else(|| panic!("Failed to get field {} as bool", &self.field));
                match val {
                    Actions::One(_) => "1".into(),
                    Actions::Many(actions) => format!("{}", actions.len()),
                }
            }
            FieldType::FixedEnum(_) => get_string(item, &self.field),
        }
    }

    pub fn set_value_fromstr(
        &self,
        item: &mut dyn Struct,
        value: &str,
    ) -> Result<(), anyhow::Error> {
        match self.type_ {
            FieldType::String => apply_value(item, &self.field, &value.to_owned()),
            FieldType::Float(_) => {
                apply_value(
                    item,
                    &self.field,
                    &value
                        .parse::<f64>()
                        .unwrap_or_else(|_| panic!("Can't parse {} to f64", &value)),
                );
            }
            FieldType::Int => {
                apply_value(
                    item,
                    &self.field,
                    &value
                        .parse::<u32>()
                        .unwrap_or_else(|_| panic!("Can't parse {} to u32", &value)),
                );
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
                apply_value(item, &self.field, &time.num_seconds_from_midnight());
            }
            FieldType::Bool => {
                let selected = value == "true";
                apply_value(item, &self.field, &selected);
            }
            FieldType::AltitudeFeet => {
                let meters = value.parse::<f64>()? * FEET_TO_METERS;
                apply_value(item, &self.field, &meters);
            }
            FieldType::SpeedKnotsTAS => {
                let ms = value.parse::<f64>()? * KNOTS_TO_MS;
                apply_value(item, &self.field, &ms);
            }
            FieldType::DistanceNM => {
                let meters = value.parse::<f64>()? * NM_TO_METERS;
                apply_value(item, &self.field, &meters);
            }
            FieldType::DurationMin => {
                let seconds = value.parse::<u32>()? * 60;
                apply_value(item, &self.field, &seconds);
            }
            FieldType::TriggerActions => {
                apply_value(item, &self.field, &value.to_owned());
            }
            FieldType::FixedEnum(_) => apply_value(item, &self.field, &value.to_owned()),
        };
        Ok(())
    }
}

fn get_string(item: &dyn Struct, field: &str) -> String {
    item.field(field)
        .unwrap_or_else(|| panic!("Field {} should exist", field))
        .downcast_ref::<String>()
        .unwrap_or_else(|| panic!("Failed to get field {} as String", field))
        .to_string()
}

fn get_f64(item: &dyn Struct, field: &str) -> f64 {
    *item
        .field(field)
        .unwrap_or_else(|| panic!("Field {} should exist", field))
        .downcast_ref::<f64>()
        .unwrap_or_else(|| panic!("Failed to get field {} as f64", field))
}

fn get_u32(item: &dyn Struct, field: &str) -> u32 {
    *item
        .field(field)
        .unwrap_or_else(|| panic!("Field {} should exist", field))
        .downcast_ref::<u32>()
        .unwrap_or_else(|| panic!("Failed to get field {} as u32", field))
}

fn apply_value(item: &mut dyn Struct, field: &str, value: &dyn Reflect) {
    item.field_mut(field)
        .unwrap_or_else(|| panic!("Field {} doesn't exist on type", field))
        .apply(value.to_owned());
}

#[derive(PartialEq, Debug, Clone)]
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
    FixedEnum(Vec<String>),
}
