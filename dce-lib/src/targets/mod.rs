use bevy_reflect::{FromReflect, Reflect};
use serde::{Deserialize, Serialize};

use crate::editable::Editable;

pub mod cap;
pub mod strike;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Reflect, FromReflect)]
pub struct TargetFirepower {
    pub min: u32,
    pub max: u32,
}

impl Editable for TargetFirepower {
    fn get_name(&self) -> String {
        "".into()
    }

    fn validate(&self, _: &crate::DCEInstance) -> crate::editable::ValidationResult {
        todo!()
    }

    fn get_mut_by_name<'a>(_: &'a mut crate::DCEInstance, _: &str) -> &'a mut Self {
        todo!()
    }
}
