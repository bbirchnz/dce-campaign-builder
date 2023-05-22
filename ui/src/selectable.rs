use dce_lib::{
    campaign_header::Header,
    db_airbases::FixedAirBase,
    loadouts::{CAPLoadout, StrikeLoadout},
    mappable::MapPoint,
    oob_air::Squadron,
    targets::{cap::CAP, strike::Strike},
    DCEInstance,
};

#[derive(Clone, PartialEq)]
pub enum Selectable {
    Squadron(Squadron),
    TargetStrike(Strike),
    TargetCAP(CAP),
    FixedAirBase(FixedAirBase),
    CampaignSettings(Header),
    LoadoutCAP(CAPLoadout),
    LoadoutStrike(StrikeLoadout),
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

    fn from_selectable(sel: &Selectable) -> Option<Self>
    where
        Self: Sized;
}

impl ToSelectable for Squadron {
    fn to_selectable(&self) -> Selectable {
        Selectable::Squadron(self.clone())
    }

    fn from_selectable(sel: &Selectable) -> Option<Self> {
        if let Selectable::Squadron(squad) = sel {
            return Some(squad.clone());
        }
        None
    }
}

impl ToSelectable for Strike {
    fn to_selectable(&self) -> Selectable {
        Selectable::TargetStrike(self.clone())
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
}

impl ToSelectable for CAP {
    fn to_selectable(&self) -> Selectable {
        Selectable::TargetCAP(self.clone())
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
}

impl ToSelectable for FixedAirBase {
    fn to_selectable(&self) -> Selectable {
        Selectable::FixedAirBase(self.clone())
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
}

impl ToSelectable for Header {
    fn to_selectable(&self) -> Selectable {
        Selectable::CampaignSettings(self.clone())
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
}

impl ToSelectable for CAPLoadout {
    fn to_selectable(&self) -> Selectable {
        Selectable::LoadoutCAP(self.to_owned())
    }

    fn from_selectable(sel: &Selectable) -> Option<Self>
    where
        Self: Sized,
    {
        if let Selectable::LoadoutCAP(cap) = sel {
            return Some(cap.clone());
        }
        None
    }
}

impl ToSelectable for StrikeLoadout {
    fn to_selectable(&self) -> Selectable {
        Selectable::LoadoutStrike(self.to_owned())
    }

    fn from_selectable(sel: &Selectable) -> Option<Self>
    where
        Self: Sized,
    {
        if let Selectable::LoadoutStrike(strike) = sel {
            return Some(strike.clone());
        }
        None
    }
}
