use bevy_reflect::{FromReflect, Reflect};
use serde::{Deserialize, Serialize};

use crate::editable::{FieldType, HeaderField, NestedEditable, ValidationError, ValidationResult};

pub mod anti_ship;
pub mod awacs;
pub mod cap;
pub mod fighter_sweep;
pub mod intercept;
pub mod refueling;
pub mod strike;
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Reflect, FromReflect)]
pub struct TargetFirepower {
    pub min: u32,
    pub max: u32,
}

impl NestedEditable for TargetFirepower {
    fn validate(&self, _: &crate::DCEInstance) -> crate::editable::ValidationResult {
        let mut errors = Vec::default();

        if self.min > self.max {
            errors.push(ValidationError::new(
                "min",
                "Minimum",
                "Minimum firepower must be <= Max Firepower",
            ));
        }

        if !errors.is_empty() {
            ValidationResult::Fail(errors)
        } else {
            ValidationResult::Pass
        }
    }

    fn get_header() -> Vec<crate::editable::HeaderField>
    where
        Self: Sized,
    {
        vec![
            HeaderField::new("min", "Minimum", FieldType::Int, true),
            HeaderField::new("max", "Maximum", FieldType::Int, true),
        ]
    }
}
