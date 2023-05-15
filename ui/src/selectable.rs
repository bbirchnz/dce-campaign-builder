use dce_lib::{
    mappable::{MapPoint},
    oob_air::Squadron,
    DCEInstance,
};

#[derive(Clone)]
pub enum Selectable {
    Squadron(Squadron),
    None,
}

impl Selectable {
    pub fn from_map(map_point: &MapPoint, instance: DCEInstance) -> Selectable {
        match map_point.class.as_str() {
            "TargetCAP" => Selectable::None,
            _ => Selectable::None,
        }
    }
}

pub trait ToSelectable {
    fn to_selectable(&self) -> Selectable;
}

impl ToSelectable for Squadron {
    fn to_selectable(&self) -> Selectable {
        Selectable::Squadron(self.clone())
    }
}
