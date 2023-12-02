use dce_lib::{
    bin_data::BinItem,
    campaign_header::HeaderInternal,
    db_airbases::{AirStartBase, FarpBase, FixedAirBase, ShipBase},
    loadouts::{
        AARLoadout, AWACSLoadout, AntiShipLoadout, CAPLoadout, EscortLoadout, InterceptLoadout,
        SEADLoadout, StrikeLoadout, TransportLoadout,
    },
    mappable::MapPoint,
    oob_air::Squadron,
    targets::{
        anti_ship::AntiShipStrike, awacs::AWACS, cap::CAP, intercept::Intercept,
        refueling::Refueling, strike::Strike,
    },
    trigger::Trigger,
    DCEInstance,
};

#[derive(Clone, PartialEq)]
pub enum Selectable {
    Squadron(Option<Squadron>),
    TargetStrike(Option<Strike>),
    TargetCAP(Option<CAP>),
    TargetAntiShip(Option<AntiShipStrike>),
    TargetAAR(Option<Refueling>),
    TargetAWACS(Option<AWACS>),
    // has to be option as there might not be any defined
    TargetIntercept(Option<Intercept>),
    FixedAirBase(Option<FixedAirBase>),
    FARPBase(Option<FarpBase>),
    ShipAirBase(Option<ShipBase>),
    AirstartBase(Option<AirStartBase>),
    CampaignSettings(HeaderInternal),
    LoadoutCAP(Option<CAPLoadout>),
    LoadoutStrike(Option<StrikeLoadout>),
    LoadoutAntiship(Option<AntiShipLoadout>),
    LoadoutAWACS(Option<AWACSLoadout>),
    LoadoutAAR(Option<AARLoadout>),
    LoadoutEscort(Option<EscortLoadout>),
    LoadoutIntercept(Option<InterceptLoadout>),
    LoadoutSEAD(Option<SEADLoadout>),
    LoadoutTransport(Option<TransportLoadout>),
    Trigger(Option<Trigger>),
    Image(Option<BinItem>),
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
                Selectable::TargetCAP(Some(cap))
            }
            "TargetStrike" => {
                let item = instance
                    .target_list
                    .strike
                    .iter()
                    .find(|c| c._name == map_point.name)
                    .unwrap()
                    .clone();
                Selectable::TargetStrike(Some(item))
            }
            "TargetAntiShipStrike" => {
                let item = instance
                    .target_list
                    .antiship
                    .iter()
                    .find(|c| c._name == map_point.name)
                    .unwrap()
                    .clone();
                Selectable::TargetAntiShip(Some(item))
            }
            "TargetRefuel" => {
                let item = instance
                    .target_list
                    .refuel
                    .iter()
                    .find(|c| c._name == map_point.name)
                    .unwrap()
                    .clone();
                Selectable::TargetAAR(Some(item))
            }
            "TargetAWACS" => {
                let item = instance
                    .target_list
                    .awacs
                    .iter()
                    .find(|c| c._name == map_point.name)
                    .unwrap()
                    .clone();
                Selectable::TargetAWACS(Some(item))
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
                Selectable::Squadron(Some(item))
            }
            "FixedAirBase" => {
                let item = instance
                    .airbases
                    .fixed
                    .iter()
                    .find(|item| item._name == map_point.name)
                    .unwrap()
                    .clone();
                Selectable::FixedAirBase(Some(item))
            }
            "ShipAirBase" => {
                let item = instance
                    .airbases
                    .ship
                    .iter()
                    .find(|item| item._name == map_point.name)
                    .unwrap()
                    .clone();
                Selectable::ShipAirBase(Some(item))
            }
            "Airstart" => {
                let item = instance
                    .airbases
                    .air_start
                    .iter()
                    .find(|item| item._name == map_point.name)
                    .unwrap()
                    .clone();
                Selectable::AirstartBase(Some(item))
            }
            "FARP" => {
                let item = instance
                    .airbases
                    .farp
                    .iter()
                    .find(|item| item._name == map_point.name)
                    .unwrap()
                    .clone();
                Selectable::FARPBase(Some(item))
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
        Selectable::Squadron(Some(self.clone()))
    }

    fn from_selectable(sel: &Selectable) -> Option<Self> {
        if let Selectable::Squadron(Some(squad)) = sel {
            return Some(squad.clone());
        }
        None
    }
}

impl ToSelectable for Strike {
    fn to_selectable(&self) -> Selectable {
        Selectable::TargetStrike(Some(self.clone()))
    }

    fn from_selectable(sel: &Selectable) -> Option<Self>
    where
        Self: Sized,
    {
        if let Selectable::TargetStrike(Some(t)) = sel {
            return Some(t.clone());
        }
        None
    }
}

impl ToSelectable for CAP {
    fn to_selectable(&self) -> Selectable {
        Selectable::TargetCAP(Some(self.clone()))
    }

    fn from_selectable(sel: &Selectable) -> Option<Self>
    where
        Self: Sized,
    {
        if let Selectable::TargetCAP(Some(t)) = sel {
            return Some(t.clone());
        }
        None
    }
}

impl ToSelectable for AWACS {
    fn to_selectable(&self) -> Selectable {
        Selectable::TargetAWACS(Some(self.clone()))
    }

    fn from_selectable(sel: &Selectable) -> Option<Self>
    where
        Self: Sized,
    {
        if let Selectable::TargetAWACS(Some(t)) = sel {
            return Some(t.clone());
        }
        None
    }
}

impl ToSelectable for Refueling {
    fn to_selectable(&self) -> Selectable {
        Selectable::TargetAAR(Some(self.clone()))
    }

    fn from_selectable(sel: &Selectable) -> Option<Self>
    where
        Self: Sized,
    {
        if let Selectable::TargetAAR(Some(t)) = sel {
            return Some(t.clone());
        }
        None
    }
}

impl ToSelectable for AntiShipStrike {
    fn to_selectable(&self) -> Selectable {
        Selectable::TargetAntiShip(Some(self.clone()))
    }

    fn from_selectable(sel: &Selectable) -> Option<Self>
    where
        Self: Sized,
    {
        if let Selectable::TargetAntiShip(Some(t)) = sel {
            return Some(t.clone());
        }
        None
    }
}

impl ToSelectable for Intercept {
    fn to_selectable(&self) -> Selectable {
        Selectable::TargetIntercept(Some(self.clone()))
    }

    fn from_selectable(sel: &Selectable) -> Option<Self>
    where
        Self: Sized,
    {
        if let Selectable::TargetIntercept(Some(t)) = sel {
            return Some(t.clone());
        }
        None
    }
}

impl ToSelectable for FixedAirBase {
    fn to_selectable(&self) -> Selectable {
        Selectable::FixedAirBase(Some(self.clone()))
    }

    fn from_selectable(sel: &Selectable) -> Option<Self>
    where
        Self: Sized,
    {
        if let Selectable::FixedAirBase(Some(t)) = sel {
            return Some(t.clone());
        }
        None
    }
}

impl ToSelectable for AirStartBase {
    fn to_selectable(&self) -> Selectable {
        Selectable::AirstartBase(Some(self.clone()))
    }

    fn from_selectable(sel: &Selectable) -> Option<Self>
    where
        Self: Sized,
    {
        if let Selectable::AirstartBase(Some(t)) = sel {
            return Some(t.clone());
        }
        None
    }
}

impl ToSelectable for ShipBase {
    fn to_selectable(&self) -> Selectable {
        Selectable::ShipAirBase(Some(self.clone()))
    }

    fn from_selectable(sel: &Selectable) -> Option<Self>
    where
        Self: Sized,
    {
        if let Selectable::ShipAirBase(Some(t)) = sel {
            return Some(t.clone());
        }
        None
    }
}

impl ToSelectable for HeaderInternal {
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
        Selectable::LoadoutCAP(Some(self.to_owned()))
    }

    fn from_selectable(sel: &Selectable) -> Option<Self>
    where
        Self: Sized,
    {
        if let Selectable::LoadoutCAP(Some(cap)) = sel {
            return Some(cap.clone());
        }
        None
    }
}

impl ToSelectable for StrikeLoadout {
    fn to_selectable(&self) -> Selectable {
        Selectable::LoadoutStrike(Some(self.to_owned()))
    }

    fn from_selectable(sel: &Selectable) -> Option<Self>
    where
        Self: Sized,
    {
        if let Selectable::LoadoutStrike(Some(strike)) = sel {
            return Some(strike.clone());
        }
        None
    }
}
impl ToSelectable for AntiShipLoadout {
    fn to_selectable(&self) -> Selectable {
        Selectable::LoadoutAntiship(Some(self.to_owned()))
    }

    fn from_selectable(sel: &Selectable) -> Option<Self>
    where
        Self: Sized,
    {
        if let Selectable::LoadoutAntiship(Some(item)) = sel {
            return Some(item.clone());
        }
        None
    }
}
impl ToSelectable for AARLoadout {
    fn to_selectable(&self) -> Selectable {
        Selectable::LoadoutAAR(Some(self.to_owned()))
    }

    fn from_selectable(sel: &Selectable) -> Option<Self>
    where
        Self: Sized,
    {
        if let Selectable::LoadoutAAR(Some(item)) = sel {
            return Some(item.clone());
        }
        None
    }
}
impl ToSelectable for AWACSLoadout {
    fn to_selectable(&self) -> Selectable {
        Selectable::LoadoutAWACS(Some(self.to_owned()))
    }

    fn from_selectable(sel: &Selectable) -> Option<Self>
    where
        Self: Sized,
    {
        if let Selectable::LoadoutAWACS(Some(item)) = sel {
            return Some(item.clone());
        }
        None
    }
}

impl ToSelectable for EscortLoadout {
    fn to_selectable(&self) -> Selectable {
        Selectable::LoadoutEscort(Some(self.to_owned()))
    }

    fn from_selectable(sel: &Selectable) -> Option<Self>
    where
        Self: Sized,
    {
        if let Selectable::LoadoutEscort(Some(item)) = sel {
            return Some(item.clone());
        }
        None
    }
}

impl ToSelectable for InterceptLoadout {
    fn to_selectable(&self) -> Selectable {
        Selectable::LoadoutIntercept(Some(self.to_owned()))
    }

    fn from_selectable(sel: &Selectable) -> Option<Self>
    where
        Self: Sized,
    {
        if let Selectable::LoadoutIntercept(Some(item)) = sel {
            return Some(item.clone());
        }
        None
    }
}

impl ToSelectable for SEADLoadout {
    fn to_selectable(&self) -> Selectable {
        Selectable::LoadoutSEAD(Some(self.to_owned()))
    }

    fn from_selectable(sel: &Selectable) -> Option<Self>
    where
        Self: Sized,
    {
        if let Selectable::LoadoutSEAD(Some(item)) = sel {
            return Some(item.clone());
        }
        None
    }
}

impl ToSelectable for TransportLoadout {
    fn to_selectable(&self) -> Selectable {
        Selectable::LoadoutTransport(Some(self.to_owned()))
    }

    fn from_selectable(sel: &Selectable) -> Option<Self>
    where
        Self: Sized,
    {
        if let Selectable::LoadoutTransport(Some(item)) = sel {
            return Some(item.clone());
        }
        None
    }
}

impl ToSelectable for Trigger {
    fn to_selectable(&self) -> Selectable {
        Selectable::Trigger(Some(self.to_owned()))
    }

    fn from_selectable(sel: &Selectable) -> Option<Self>
    where
        Self: Sized,
    {
        if let Selectable::Trigger(Some(trigger)) = sel {
            return Some(trigger.clone());
        }
        None
    }
}

impl ToSelectable for FarpBase {
    fn to_selectable(&self) -> Selectable {
        Selectable::FARPBase(Some(self.to_owned()))
    }

    fn from_selectable(sel: &Selectable) -> Option<Self>
    where
        Self: Sized,
    {
        if let Selectable::FARPBase(Some(trigger)) = sel {
            return Some(trigger.clone());
        }
        None
    }
}

impl ToSelectable for BinItem {
    fn to_selectable(&self) -> Selectable {
        Selectable::Image(Some(self.to_owned()))
    }

    fn from_selectable(sel: &Selectable) -> Option<Self>
    where
        Self: Sized,
    {
        if let Selectable::Image(Some(trigger)) = sel {
            return Some(trigger.clone());
        }
        None
    }
}
