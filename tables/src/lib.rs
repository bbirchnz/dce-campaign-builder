use anyhow::anyhow;
use bevy_reflect::Struct;
use chrono::{NaiveTime, Timelike};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub trait TableHeader {
    fn get_header() -> Vec<HeaderField>;
}

#[derive(PartialEq)]
pub struct HeaderField {
    pub field: String,
    pub display: String,
    pub type_: FieldType,
    pub editable: bool,
}

const METERS_TO_FEET: f64 = 3.281;
const FEET_TO_METERS: f64 = 1. / METERS_TO_FEET;

const MS_to_KNOTS: f64 = 3.6 / 1.852;
const KNOTS_TO_MS: f64 = 1. / MS_to_KNOTS;

const NM_TO_METERS: f64 = 1852.;
const METERS_TO_NM: f64 = 1. / NM_TO_METERS;

impl HeaderField {
    pub fn get_value_string(&self, item: &dyn Struct) -> String {
        match self.type_ {
            FieldType::String => item
                .field(&self.field)
                .unwrap()
                .downcast_ref::<String>()
                .unwrap()
                .to_string(),
            FieldType::Float(func) => {
                let value = item
                    .field(&self.field)
                    .unwrap()
                    .downcast_ref::<f64>()
                    .expect(&format!("Failed to get field {} as f64", &self.field));
                func(*value)
            } // .to_string(),
            FieldType::Int => item
                .field(&self.field)
                .unwrap()
                .downcast_ref::<u32>()
                .expect(&format!("Failed to get field {} as u32", &self.field))
                .to_string(),
            FieldType::Enum => "".into(),
            FieldType::VecString => item
                .field(&self.field)
                .unwrap()
                .downcast_ref::<Vec<String>>()
                .expect(&format!(
                    "Failed to get field {} as Vec<String>",
                    &self.field
                ))
                .join(", "),
            FieldType::Debug => {
                let v = item.field(&self.field).unwrap();
                format!("{:?}", v)
            }
            FieldType::IntTime => {
                let seconds_since_midnight = item
                    .field(&self.field)
                    .unwrap()
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
                    .unwrap()
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
                    .unwrap()
                    .downcast_ref::<f64>()
                    .expect(&format!("Failed to get field {} as f64", &self.field));
                format!("{:.0}", meters * METERS_TO_FEET)
            }
            FieldType::SpeedKnotsTAS => {
                let ms = item
                    .field(&self.field)
                    .unwrap()
                    .downcast_ref::<f64>()
                    .expect(&format!("Failed to get field {} as f64", &self.field));
                format!("{:.0}", ms * MS_to_KNOTS)
            }
            FieldType::DistanceNM => {
                let meters = item
                    .field(&self.field)
                    .unwrap()
                    .downcast_ref::<f64>()
                    .expect(&format!("Failed to get field {} as f64", &self.field));
                format!("{:.0}", meters * METERS_TO_NM)
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
            FieldType::VecString => todo!(),
            FieldType::Debug => todo!(),
            FieldType::IntTime => {
                let time =
                    NaiveTime::parse_from_str(value, "%H:%M:%S").expect("Expected HH:MM:SS string");
                item.field_mut(&self.field)
                    .ok_or(anyhow!("Couldn't get field {}", &self.field))?
                    .apply(&time.num_seconds_from_midnight());
            }
            FieldType::Bool => {
                let selected = if value == "true" { true } else { false };

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
        };
        Ok(())
    }
}

#[derive(PartialEq)]
pub enum FieldType {
    String,
    Float(fn(f64) -> String),
    Int,
    Enum,
    VecString,
    Debug,
    IntTime,
    Bool,
    AltitudeFeet,
    SpeedKnotsTAS,
    DistanceNM,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
