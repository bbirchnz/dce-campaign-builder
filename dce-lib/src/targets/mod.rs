use bevy_reflect::{FromReflect, Reflect};
use serde::{Deserialize, Serialize};

pub mod cap;
pub mod strike;
pub mod awacs;
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Reflect, FromReflect)]
pub struct TargetFirepower {
    pub min: u32,
    pub max: u32,
}
