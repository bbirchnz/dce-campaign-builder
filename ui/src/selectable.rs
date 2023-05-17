use dce_lib::{
    campaign_header::Header,
    db_airbases::FixedAirBase,
    mappable::MapPoint,
    oob_air::Squadron,
    target_list::{Strike, CAP},
    DCEInstance,
};

#[derive(Clone, PartialEq)]
pub enum Selectable {
    Squadron(Squadron),
    TargetStrike(Strike),
    TargetCAP(CAP),
    FixedAirBase(FixedAirBase),
    CampaignSettings(Header),
    None,
}

impl Selectable {
    pub fn from_map(map_point: &MapPoint, instance: &DCEInstance) -> Selectable {
        match map_point.class.as_str() {
            "TargetCAP" => {
                let cap = instance
                    .target_list
                    .cap
                    .iter()
                    .find(|c| c._name == map_point.name)
                    .unwrap()
                    .clone();
                Selectable::TargetCAP(cap)
            }
            "TargetStrike" => {
                let item = instance
                    .target_list
                    .strike
                    .iter()
                    .find(|c| c._name == map_point.name)
                    .unwrap()
                    .clone();
                Selectable::TargetStrike(item)
            }
            "Squadron" => {
                let item = instance
                    .oob_air
                    .blue
                    .iter()
                    .chain(instance.oob_air.red.iter())
                    .find(|c| c.name == map_point.name)
                    .unwrap()
                    .clone();
                Selectable::Squadron(item)
            }
            "FixedAirBase" => {
                let item = instance
                    .airbases
                    .fixed
                    .iter()
                    .find(|item| item._name == map_point.name)
                    .unwrap()
                    .clone();
                Selectable::FixedAirBase(item)
            }
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
            .chain(instance.oob_air.blue.iter_mut())
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

impl ToSelectable for CAP {
    fn to_selectable(&self) -> Selectable {
        Selectable::TargetCAP(self.clone())
    }

    fn get_mut_by_name<'a>(instance: &'a mut DCEInstance, name: &str) -> &'a mut Self {
        instance
            .target_list
            .cap
            .iter_mut()
            .find(|s| s._name == name)
            .unwrap()
    }

    fn from_selectable(sel: &Selectable) -> Option<Self>
    where
        Self: Sized,
    {
        if let Selectable::TargetCAP(t) = sel {
            return Some(t.clone());
        }
        None
    }

    fn get_name(&self) -> String {
        self.text.to_string()
    }
}

impl ToSelectable for FixedAirBase {
    fn to_selectable(&self) -> Selectable {
        Selectable::FixedAirBase(self.clone())
    }

    fn get_mut_by_name<'a>(instance: &'a mut DCEInstance, name: &str) -> &'a mut Self {
        instance
            .airbases
            .fixed
            .iter_mut()
            .find(|item| item._name == name)
            .unwrap()
    }

    fn from_selectable(sel: &Selectable) -> Option<Self>
    where
        Self: Sized,
    {
        if let Selectable::FixedAirBase(t) = sel {
            return Some(t.clone());
        }
        None
    }

    fn get_name(&self) -> String {
        self._name.to_owned()
    }
}

impl ToSelectable for Header {
    fn to_selectable(&self) -> Selectable {
        Selectable::CampaignSettings(self.clone())
    }

    fn get_mut_by_name<'a>(instance: &'a mut DCEInstance, _: &str) -> &'a mut Self {
        &mut instance.campaign_header
    }

    fn from_selectable(sel: &Selectable) -> Option<Self>
    where
        Self: Sized,
    {
        if let Selectable::CampaignSettings(header) = sel {
            return Some(header.clone());
        }
        None
    }

    fn get_name(&self) -> String {
        "settings".into()
    }
}
