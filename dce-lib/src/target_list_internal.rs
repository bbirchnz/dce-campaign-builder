use std::{collections::HashMap, iter::repeat};

use crate::{
    mappable::{MapPoint, Mappables},
    projections::{convert_dcs_lat_lon, offset},
    target_list::{self, FighterSweep, Intercept, Refueling, Strike, Target, TargetList, CAP},
};
use anyhow::anyhow;
use bevy_reflect::{FromReflect, Reflect};
use log::info;
use proj::Proj;
use serde::{Deserialize, Serialize};

/// A much more convenient form where everythings in vecs appropriate to their type
/// and has name and side all within one
///
#[derive(Deserialize, Serialize, Debug, PartialEq, Reflect, FromReflect, Clone)]
pub struct TargetListInternal {
    pub strike: Vec<Strike>,
    pub cap: Vec<CAP>,
    pub refuel: Vec<Refueling>,
    pub antiship: Vec<Strike>,
    pub intercept: Vec<Intercept>,
    pub fighter_sweep: Vec<FighterSweep>,
}

impl TargetListInternal {
    pub fn from_target_list(tlist: &TargetList) -> TargetListInternal {
        let mut strike = Vec::default();
        let mut cap = Vec::default();
        let mut antiship = Vec::default();
        let mut refuel = Vec::default();
        let mut intercept = Vec::default();
        let mut fighter_sweep = Vec::default();

        tlist
            .blue
            .iter()
            .zip(repeat("blue"))
            .chain(tlist.red.iter().zip(repeat("red")))
            .for_each(|((name, tgt), side)| match tgt {
                target_list::Target::CAP(i) => {
                    let mut i = i.clone();
                    i._name = name.to_owned();
                    i._side = side.to_owned();
                    cap.push(i);
                }
                target_list::Target::Refueling(i) => {
                    let mut i = i.clone();
                    i._name = name.to_owned();
                    i._side = side.to_owned();
                    refuel.push(i);
                }
                target_list::Target::Intercept(i) => {
                    let mut i = i.clone();
                    i._name = name.to_owned();
                    i._side = side.to_owned();
                    intercept.push(i);
                }
                target_list::Target::FighterSweep(i) => {
                    let mut i = i.clone();
                    i._name = name.to_owned();
                    i._side = side.to_owned();
                    fighter_sweep.push(i);
                }
                target_list::Target::Strike(i) => {
                    let mut i = i.clone();
                    i._name = name.to_owned();
                    i._side = side.to_owned();
                    strike.push(i);
                }
                target_list::Target::AntiShipStrike(i) => {
                    let mut i = i.clone();
                    i._name = name.to_owned();
                    i._side = side.to_owned();
                    antiship.push(i);
                }
            });

        TargetListInternal {
            strike,
            cap,
            antiship,
            refuel,
            intercept,
            fighter_sweep,
        }
    }

    pub fn to_target_list(&self) -> Result<TargetList, anyhow::Error> {
        let mut blue = HashMap::default();
        let mut red = HashMap::default();

        self.antiship.iter().try_for_each(|item| {
            match item._side.as_str() {
                "blue" => {
                    let _ =
                        blue.insert(item._name.to_owned(), Target::AntiShipStrike(item.clone()));
                }
                "red" => {
                    let _ = red.insert(item._name.to_owned(), Target::AntiShipStrike(item.clone()));
                }
                _ => return Err(anyhow!("Got side == {}", item._side)),
            }
            Ok(())
        })?;

        self.cap.iter().try_for_each(|item| {
            match item._side.as_str() {
                "blue" => {
                    let _ = blue.insert(item._name.to_owned(), Target::CAP(item.clone()));
                }
                "red" => {
                    let _ = red.insert(item._name.to_owned(), Target::CAP(item.clone()));
                }
                _ => return Err(anyhow!("Got side == {}", item._side)),
            }
            Ok(())
        })?;

        self.intercept.iter().try_for_each(|item| {
            match item._side.as_str() {
                "blue" => {
                    let _ = blue.insert(item._name.to_owned(), Target::Intercept(item.clone()));
                }
                "red" => {
                    let _ = red.insert(item._name.to_owned(), Target::Intercept(item.clone()));
                }
                _ => return Err(anyhow!("Got side == {}", item._side)),
            }
            Ok(())
        })?;

        self.strike.iter().try_for_each(|item| {
            match item._side.as_str() {
                "blue" => {
                    let _ = blue.insert(item._name.to_owned(), Target::Strike(item.clone()));
                }
                "red" => {
                    let _ = red.insert(item._name.to_owned(), Target::Strike(item.clone()));
                }
                _ => return Err(anyhow!("Got side == {}", item._side)),
            }
            Ok(())
        })?;

        self.fighter_sweep.iter().try_for_each(|item| {
            match item._side.as_str() {
                "blue" => {
                    let _ = blue.insert(item._name.to_owned(), Target::FighterSweep(item.clone()));
                }
                "red" => {
                    let _ = red.insert(item._name.to_owned(), Target::FighterSweep(item.clone()));
                }
                _ => return Err(anyhow!("Got side == {}", item._side)),
            }
            Ok(())
        })?;

        self.refuel.iter().try_for_each(|item| {
            match item._side.as_str() {
                "blue" => {
                    let _ = blue.insert(item._name.to_owned(), Target::Refueling(item.clone()));
                }
                "red" => {
                    let _ = red.insert(item._name.to_owned(), Target::Refueling(item.clone()));
                }
                _ => return Err(anyhow!("Got side == {}", item._side)),
            }
            Ok(())
        })?;

        Ok(TargetList { blue, red })
    }
}

impl Mappables for TargetListInternal {
    fn to_mappables(
        &self,
        instance: &crate::DCEInstance,
        proj: &Proj,
    ) -> Vec<crate::mappable::MapPoint> {
        let mut map_points = Vec::default();

        self.cap.iter().for_each(|cap| {
            let zone = instance.mission.get_zone_by_name(&cap.ref_point);
            match zone {
                Ok(zone) => {
                    let (x2, y2) = offset(zone.x, zone.y, cap.axis, cap.radius);
                    info!("{} {}, {} {}", zone.x, zone.y, x2, y2);
                    let (lon2, lat2) = convert_dcs_lat_lon(x2, y2, proj);
                    map_points.push(
                        MapPoint::new_from_dcs(
                            zone.x,
                            zone.y,
                            cap._name.to_owned(),
                            cap._side.to_owned(),
                            "TargetCAP".into(),
                            proj,
                        )
                        .add_extras(HashMap::from([
                            ("radius".to_string(), cap.radius),
                            ("axis".to_string(), cap.axis),
                            ("lat2".to_string(), lat2),
                            ("lon2".to_string(), lon2),
                        ])),
                    );
                }
                Err(e) => {
                    info!("{:?}", e);
                }
            }
        });

        self.refuel.iter().for_each(|refuel| {
            let zone = instance.mission.get_zone_by_name(&refuel.ref_point);
            match zone {
                Ok(zone) => {
                    let (x2, y2) = offset(zone.x, zone.y, refuel.axis, refuel.radius);
                    info!("{} {}, {} {}", zone.x, zone.y, x2, y2);
                    let (lon2, lat2) = convert_dcs_lat_lon(x2, y2, proj);
                    map_points.push(
                        MapPoint::new_from_dcs(
                            zone.x,
                            zone.y,
                            refuel._name.to_owned(),
                            refuel._side.to_owned(),
                            "TargetRefuel".into(),
                            proj,
                        )
                        .add_extras(HashMap::from([
                            ("radius".to_string(), refuel.radius),
                            ("axis".to_string(), refuel.axis),
                            ("lat2".to_string(), lat2),
                            ("lon2".to_string(), lon2),
                        ])),
                    );
                }
                Err(e) => {
                    info!("{:?}", e);
                }
            }
        });

        self.strike.iter().for_each(|strike| {
            if strike.class == "vehicle" && strike.class_template.is_some() {
                let all_groups = instance.mission.get_vehicle_groups();

                let groups = all_groups
                    .iter()
                    .filter(|g| &g.name == strike.class_template.as_ref().unwrap())
                    .collect::<Vec<_>>();

                if groups.len() == 1 {
                    map_points.push(MapPoint::new_from_dcs(
                        groups[0].x,
                        groups[0].y,
                        strike.text.to_owned(),
                        strike._side.to_owned(),
                        "TargetStrike".into(),
                        proj,
                    ));
                }
            }
        });

        self.antiship.iter().for_each(|antiship| {
            if antiship.class == "ship" && antiship.class_template.is_some() {
                let all_groups = instance.mission.get_ship_groups();

                let groups = all_groups
                    .iter()
                    .filter(|g| &g.name == antiship.class_template.as_ref().unwrap())
                    .collect::<Vec<_>>();

                if groups.len() == 1 {
                    map_points.push(MapPoint::new_from_dcs(
                        groups[0].x,
                        groups[0].y,
                        antiship.text.to_owned(),
                        antiship._side.to_owned(),
                        "TargetAntiShipStrike".into(),
                        proj,
                    ));
                }
            }
        });

        map_points
    }
}
