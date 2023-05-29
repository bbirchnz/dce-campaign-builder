use bevy_reflect::{FromReflect, Reflect};
use serde::{Deserialize, Serialize};

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
