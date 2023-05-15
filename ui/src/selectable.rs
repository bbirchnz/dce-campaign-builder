use dce_lib::{mappable::MapPoint, oob_air::Squadron, target_list::Strike, DCEInstance};

#[derive(Clone, PartialEq)]
pub enum Selectable {
    Squadron(Squadron),
    TargetStrike(Strike),
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

    fn get_mut_by_name<'a>(instance: &'a mut DCEInstance, name: &str) -> &'a mut Self;

    fn from_selectable(sel: &Selectable) -> Option<Self>
    where
        Self: Sized;

    fn get_name(&self) -> String;
}

impl ToSelectable for Squadron {
    fn to_selectable(&self) -> Selectable {
        Selectable::Squadron(self.clone())
    }

    fn get_mut_by_name<'a>(instance: &'a mut DCEInstance, name: &str) -> &'a mut Squadron {
        instance
            .oob_air
            .red
            .iter_mut()
            .find(|s| s.name == name)
            .unwrap()
    }

    fn from_selectable(sel: &Selectable) -> Option<Self> {
        if let Selectable::Squadron(squad) = sel {
            return Some(squad.clone());
        }
        None
    }

    fn get_name(&self) -> String {
        self.name.to_string()
    }
}

impl ToSelectable for Strike {
    fn to_selectable(&self) -> Selectable {
        Selectable::TargetStrike(self.clone())
    }

    fn get_mut_by_name<'a>(instance: &'a mut DCEInstance, name: &str) -> &'a mut Self {
        instance
            .target_list
            .strike
            .iter_mut()
            .find(|s| s._name == name)
            .unwrap()
    }

    fn from_selectable(sel: &Selectable) -> Option<Self>
    where
        Self: Sized,
    {
        if let Selectable::TargetStrike(t) = sel {
            return Some(t.clone());
        }
        None
    }

    fn get_name(&self) -> String {
        self.text.to_string()
    }
}
