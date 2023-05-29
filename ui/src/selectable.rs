use dce_lib::{
    campaign_header::Header,
    db_airbases::{AirStartBase, FixedAirBase, ShipBase},
    loadouts::{CAPLoadout, StrikeLoadout},
    mappable::MapPoint,
    oob_air::Squadron,
    targets::{
        anti_ship::AntiShipStrike, awacs::AWACS, cap::CAP, refueling::Refueling, strike::Strike,
    },
    trigger::Trigger,
    DCEInstance,
};

#[derive(Clone, PartialEq)]
pub enum Selectable {
    Squadron(Squadron),
    TargetStrike(Strike),
    TargetCAP(CAP),
    TargetAntiShip(AntiShipStrike),
    TargetAAR(Refueling),
    TargetAWACS(AWACS),
    FixedAirBase(FixedAirBase),
    ShipAirBase(ShipBase),
    AirstartBase(AirStartBase),
    CampaignSettings(Header),
    LoadoutCAP(CAPLoadout),
    LoadoutStrike(StrikeLoadout),
    Trigger(Trigger),
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
            "TargetAntiShipStrike" => {
                let item = instance
                    .target_list
                    .antiship
                    .iter()
                    .find(|c| c._name == map_point.name)
                    .unwrap()
                    .clone();
                Selectable::TargetAntiShip(item)
            }
            "TargetRefuel" => {
                let item = instance
                    .target_list
                    .refuel
                    .iter()
                    .find(|c| c._name == map_point.name)
                    .unwrap()
                    .clone();
                Selectable::TargetAAR(item)
            }
            "TargetAWACS" => {
                let item = instance
                    .target_list
                    .awacs
                    .iter()
                    .find(|c| c._name == map_point.name)
                    .unwrap()
                    .clone();
                Selectable::TargetAWACS(item)
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
            "ShipAirBase" => {
                let item = instance
                    .airbases
                    .ship
                    .iter()
                    .find(|item| item._name == map_point.name)
                    .unwrap()
                    .clone();
                Selectable::ShipAirBase(item)
            }
            "Airstart" => {
                let item = instance
                    .airbases
                    .air_start
                    .iter()
                    .find(|item| item._name == map_point.name)
                    .unwrap()
                    .clone();
                Selectable::AirstartBase(item)
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

impl ToSelectable for AWACS {
    fn to_selectable(&self) -> Selectable {
        Selectable::TargetAWACS(self.clone())
    }

    fn from_selectable(sel: &Selectable) -> Option<Self>
    where
        Self: Sized,
    {
        if let Selectable::TargetAWACS(t) = sel {
            return Some(t.clone());
        }
        None
    }
}

impl ToSelectable for Refueling {
    fn to_selectable(&self) -> Selectable {
        Selectable::TargetAAR(self.clone())
    }

    fn from_selectable(sel: &Selectable) -> Option<Self>
    where
        Self: Sized,
    {
        if let Selectable::TargetAAR(t) = sel {
            return Some(t.clone());
        }
        None
    }
}

impl ToSelectable for AntiShipStrike {
    fn to_selectable(&self) -> Selectable {
        Selectable::TargetAntiShip(self.clone())
    }

    fn from_selectable(sel: &Selectable) -> Option<Self>
    where
        Self: Sized,
    {
        if let Selectable::TargetAntiShip(t) = sel {
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

impl ToSelectable for AirStartBase {
    fn to_selectable(&self) -> Selectable {
        Selectable::AirstartBase(self.clone())
    }

    fn from_selectable(sel: &Selectable) -> Option<Self>
    where
        Self: Sized,
    {
        if let Selectable::AirstartBase(t) = sel {
            return Some(t.clone());
        }
        None
    }
}

impl ToSelectable for ShipBase {
    fn to_selectable(&self) -> Selectable {
        Selectable::ShipAirBase(self.clone())
    }

    fn from_selectable(sel: &Selectable) -> Option<Self>
    where
        Self: Sized,
    {
        if let Selectable::ShipAirBase(t) = sel {
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

impl ToSelectable for Trigger {
    fn to_selectable(&self) -> Selectable {
        Selectable::Trigger(self.to_owned())
    }

    fn from_selectable(sel: &Selectable) -> Option<Self>
    where
        Self: Sized,
    {
        if let Selectable::Trigger(trigger) = sel {
            return Some(trigger.clone());
        }
        None
    }
}
