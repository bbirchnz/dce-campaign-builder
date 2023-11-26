use bevy_reflect::{Reflect, ReflectMut, ReflectRef, Struct};

use log::warn;

use crate::{trigger::Actions, DCEInstance};
use anyhow::anyhow;
use chrono::{NaiveTime, Timelike};

/// Common methods for editable entities
pub trait Editable {
    fn get_name(&self) -> String;

    fn validate(&self, instance: &DCEInstance) -> ValidationResult;

    fn get_mut_by_name<'a>(instance: &'a mut DCEInstance, name: &str) -> &'a mut Self
    where
        Self: Sized;
    fn get_header() -> Vec<HeaderField>
    where
        Self: Sized;

    /// Returns bool indicating where this type can be reset to default values.
    ///
    /// Use to decide whether to draw buttons for example
    fn can_reset_from_miz() -> bool
    where
        Self: Sized,
    {
        false
    }

    /// Wipe out the list of entities and recreate them from the mission.
    ///
    /// Use when you have updated the base mission (say with new loadouts) and you want to
    /// bring the changes into the campaign.
    fn reset_all_from_miz(_: &mut DCEInstance) -> Result<(), anyhow::Error>
    where
        Self: Sized,
    {
        Ok(())
    }

    fn delete_by_name(instance: &mut DCEInstance, name: &str) -> Result<(), anyhow::Error>
    where
        Self: Sized;

    fn can_delete() -> bool
    where
        Self: Sized,
    {
        false
    }

    /// returns a vec of functions that can make changes
    /// to the whole set, or really anything in the DCEInstance.
    ///
    /// Will be used to create an array of buttons in the UI.
    ///
    /// Example: "create intercepts for all capable squadrons"
    fn actions_all_entities() -> Vec<AllEntityTemplateAction>
    where
        Self: Sized,
    {
        // default is no actions defined
        Vec::default()
    }

    /// returns a vec of functions that can apply changes to a single entity
    ///
    /// Will be used to create an array of buttons in the UI on that entity
    ///
    /// Example: "Set this strike loadout up for good defaults for a low level bomb attack"
    fn actions_one_entity() -> Vec<EntityTemplateAction<Self>>
    where
        Self: Sized,
    {
        // default is no actions defined
        Vec::default()
    }

    fn related(&self, _instance: &DCEInstance) -> Vec<Box<dyn Editable>> {
        Vec::default()
    }
}

/// Behaviours required to support editing of structures that are members
/// of an Editable object
pub trait NestedEditable
where
    Self: Struct,
{
    fn validate(&self, instance: &DCEInstance) -> ValidationResult;

    fn get_header() -> Vec<HeaderField>
    where
        Self: Sized;
}

/// An action that can be applied to the full DCEInstance and perform multiple entity
/// changes (e.g. delete and recreate all Intercept targets)
pub struct AllEntityTemplateAction {
    /// short name for use in button text
    pub name: String,
    /// description for use in tooltop
    pub description: String,
    /// function to execute when button clicked
    pub function: fn(&mut DCEInstance) -> Result<(), anyhow::Error>,
}

/// An action that while intended to affect one entity,
/// it can be applied to the full DCEInstance and perform multiple entity
/// changes - e.g. set player to squadron needs to also edit all other squadrons
pub struct EntityTemplateAction<T> {
    /// short name for use in button text
    pub name: String,
    /// description for use in tooltop
    pub description: String,
    /// function to execute when button clicked
    pub function: fn(&mut T, &mut DCEInstance) -> Result<(), anyhow::Error>,
}

impl<T> EntityTemplateAction<T> {
    pub fn new(
        name: &str,
        description: &str,
        function: fn(&mut T, &mut DCEInstance) -> Result<(), anyhow::Error>,
    ) -> EntityTemplateAction<T> {
        EntityTemplateAction {
            name: name.to_owned(),
            description: description.to_owned(),
            function,
        }
    }
}

impl AllEntityTemplateAction {
    pub fn new(
        name: &str,
        description: &str,
        function: fn(&mut DCEInstance) -> Result<(), anyhow::Error>,
    ) -> AllEntityTemplateAction {
        AllEntityTemplateAction {
            name: name.to_owned(),
            description: description.to_owned(),
            function,
        }
    }
}

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
        match &self.type_ {
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
            FieldType::VecString => {
                // returns the vec string, or a vec with an empty string (so we get one edit box, this is removed on save)
                let items = item
                    .field(&self.field)
                    .unwrap_or_else(|| panic!("Field {} should exist", &self.field))
                    .downcast_ref::<Vec<String>>()
                    .unwrap_or_else(|| panic!("Failed to get field {} as Actions", &self.field))
                    .to_vec();
                if items.is_empty() {
                    vec!["".into()]
                } else {
                    items
                }
            }
            FieldType::NestedEditable(sub_headers) => {
                let ReflectRef::Struct(sub_item) = item
                    .field(&self.field)
                    .unwrap_or_else(|| panic!("Field {} should exist", &self.field))
                    .reflect_ref()
                else {
                    panic!("This must be a Struct type")
                };

                sub_headers
                    .iter()
                    .map(|sub_h| sub_h.get_value_string(sub_item))
                    .collect::<Vec<_>>()
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
        match &self.type_ {
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
            FieldType::VecString => {
                let field = item
                    .field_mut(&self.field)
                    .ok_or(anyhow!("Couldn't get field {}", &self.field))?
                    .downcast_mut::<Vec<String>>()
                    .unwrap_or_else(|| {
                        panic!("Failed to get field {} as Vec<String>", &self.field)
                    });
                // if we get nothing but a single empty string, assume its completely empty
                if field.len() == 1 && field[0].len() == 0 {
                    *field = Vec::default();
                } else {
                    *field = values;
                }
                Ok(())
            }
            FieldType::NestedEditable(sub_headers) => {
                let ReflectMut::Struct(sub_item) = item
                    .field_mut(&self.field)
                    .unwrap_or_else(|| panic!("Field {} should exist", &self.field))
                    .reflect_mut()
                else {
                    panic!("This must be a Struct type")
                };

                for (sub_h, val) in sub_headers.iter().zip(values) {
                    sub_h.set_value_fromstr(sub_item, &val)?
                }
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
            FieldType::OptionString => {
                let val = item
                    .field(&self.field)
                    .unwrap_or_else(|| panic!("Field {} should exist", &self.field))
                    .downcast_ref::<Option<String>>()
                    .unwrap_or_else(|| panic!("Failed to get field {} as bool", &self.field));
                match val {
                    Some(str) => str.to_owned(),
                    None => "".to_owned(),
                }
            }
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
                    .unwrap_or_else(|| panic!("Failed to get field {} as Actions", &self.field));
                match val {
                    Actions::One(_) => "1".into(),
                    Actions::Many(actions) => format!("{}", actions.len()),
                }
            }
            FieldType::FixedEnum(_) => get_string(item, &self.field),
            FieldType::DateStr => get_string(item, &self.field),
            FieldType::VecString => {
                let val = item
                    .field(&self.field)
                    .unwrap_or_else(|| panic!("Field {} should exist", &self.field))
                    .downcast_ref::<Vec<String>>()
                    .unwrap_or_else(|| {
                        panic!("Failed to get field {} as Vec<String>", &self.field)
                    });
                val.join(", ")
            }
            FieldType::NestedEditable(_) => {
                let v = item.field(&self.field).unwrap();
                format!("{:?}", v)
            }
        }
    }

    pub fn set_value_fromstr(
        &self,
        item: &mut dyn Struct,
        value: &str,
    ) -> Result<(), anyhow::Error> {
        match self.type_ {
            FieldType::String => apply_value(item, &self.field, &value.to_owned()),
            FieldType::OptionString => {
                if value != "" {
                    apply_value(item, &self.field, &Some(value.to_owned()))
                } else {
                    apply_value(item, &self.field, &None::<String>)
                }
            }
            FieldType::Float(_) => {
                let parsed = value.parse::<f64>();
                if parsed.is_ok() {
                    apply_value(item, &self.field, parsed.as_ref().unwrap());
                }
            }
            FieldType::Int => {
                let parsed = &value.parse::<u32>();

                if parsed.is_ok() {
                    apply_value(item, &self.field, parsed.as_ref().unwrap());
                }
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
            FieldType::VecString => {
                apply_value(item, &self.field, &value.to_owned());
            }
            FieldType::DateStr => apply_value(item, &self.field, &value.to_owned()),
            FieldType::NestedEditable(_) => {
                return Err(anyhow!(
                    "Can't set_value_fromstr on NestedEditable, use stringvec form"
                ))
            }
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
    DateStr,
    IntTime,
    Bool,
    AltitudeFeet,
    SpeedKnotsTAS,
    DistanceNM,
    DurationMin,
    TriggerActions,
    FixedEnum(Vec<String>),
    OptionString,
    VecString,
    NestedEditable(Vec<HeaderField>),
}
