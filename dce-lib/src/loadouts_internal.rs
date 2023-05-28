use bevy_reflect::{FromReflect, Reflect};
use serde::{Deserialize, Serialize};

use crate::loadouts::{
    AARLoadout, AWACSLoadout, AirframeLoadout, CAPLoadout, Loadouts, StrikeLoadout,
};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Reflect, FromReflect)]
pub struct LoadoutsInternal {
    pub antiship: Vec<StrikeLoadout>,
    pub cap: Vec<CAPLoadout>,
    pub strike: Vec<StrikeLoadout>,
    pub awacs: Vec<AWACSLoadout>,
    pub aar: Vec<AARLoadout>,
}

impl LoadoutsInternal {
    pub fn from_loadouts(loadouts: &Loadouts) -> LoadoutsInternal {
        let mut antiship = Vec::default();
        let mut cap = Vec::default();
        let mut strike = Vec::default();
        let mut awacs = Vec::default();
        let mut aar = Vec::default();

        loadouts.iter().for_each(|(af, loadout_collection)| {
            if let Some(s) = &loadout_collection.strike {
                s.iter().for_each(|(name, v)| {
                    let mut v = v.to_owned();
                    v._airframe = af.to_owned();
                    v._name = name.to_owned();
                    strike.push(v)
                });
            }
            if let Some(s) = &loadout_collection.cap {
                s.iter().for_each(|(name, v)| {
                    let mut v = v.to_owned();
                    v._airframe = af.to_owned();
                    v._name = name.to_owned();
                    cap.push(v)
                });
            }
            if let Some(s) = &loadout_collection.anti_ship {
                s.iter().for_each(|(name, v)| {
                    let mut v = v.to_owned();
                    v._airframe = af.to_owned();
                    v._name = name.to_owned();
                    antiship.push(v)
                });
            }
            if let Some(s) = &loadout_collection.awacs {
                s.iter().for_each(|(name, v)| {
                    let mut v = v.to_owned();
                    v._airframe = af.to_owned();
                    v._name = name.to_owned();
                    awacs.push(v)
                });
            }
            if let Some(s) = &loadout_collection.aar {
                s.iter().for_each(|(name, v)| {
                    let mut v = v.to_owned();
                    v._airframe = af.to_owned();
                    v._name = name.to_owned();
                    aar.push(v)
                });
            }
        });

        LoadoutsInternal {
            antiship,
            cap,
            strike,
            awacs,
            aar,
        }
    }

    pub fn to_loadouts(&self) -> Loadouts {
        let mut loadout = Loadouts::default();

        self.cap.iter().for_each(|l| {
            let unit_record = loadout
                .entry(l._airframe.to_owned())
                .or_insert(AirframeLoadout::default());
            unit_record
                .cap
                .as_mut()
                .unwrap()
                .insert(l._name.to_owned(), l.to_owned());
        });

        self.strike.iter().for_each(|l| {
            let unit_record = loadout
                .entry(l._airframe.to_owned())
                .or_insert(AirframeLoadout::default());
            unit_record
                .strike
                .as_mut()
                .unwrap()
                .insert(l._name.to_owned(), l.to_owned());
        });

        self.antiship.iter().for_each(|l| {
            let unit_record = loadout
                .entry(l._airframe.to_owned())
                .or_insert(AirframeLoadout::default());
            unit_record
                .anti_ship
                .as_mut()
                .unwrap()
                .insert(l._name.to_owned(), l.to_owned());
        });

        self.awacs.iter().for_each(|l| {
            let unit_record = loadout
                .entry(l._airframe.to_owned())
                .or_insert(AirframeLoadout::default());
            unit_record
                .awacs
                .as_mut()
                .unwrap()
                .insert(l._name.to_owned(), l.to_owned());
        });

        self.aar.iter().for_each(|l| {
            let unit_record = loadout
                .entry(l._airframe.to_owned())
                .or_insert(AirframeLoadout::default());
            unit_record
                .aar
                .as_mut()
                .unwrap()
                .insert(l._name.to_owned(), l.to_owned());
        });

        loadout
    }
}
