use bevy_reflect::{FromReflect, Reflect};
use proj::Proj;
use serde::{Deserialize, Serialize};

use crate::{
    db_airbases::{
        AirBase, AirStartBase, DBAirbases, FarpBase, FixedAirBase, ReserveBase, ShipBase,
    },
    mappable::{MapPoint, Mappables},
    mission_warehouses::Warehouses,
};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Reflect, FromReflect)]
pub struct DBAirbasesInternal {
    pub fixed: Vec<FixedAirBase>,
    pub ship: Vec<ShipBase>,
    pub farp: Vec<FarpBase>,
    pub reserve: Vec<ReserveBase>,
    pub air_start: Vec<AirStartBase>,
}

impl DBAirbasesInternal {
    pub fn from_db_airbases(db_airbases: &DBAirbases, warehouses: &Warehouses) -> Self {
        let mut result = DBAirbasesInternal {
            fixed: Vec::default(),
            ship: Vec::default(),
            farp: Vec::default(),
            reserve: Vec::default(),
            air_start: Vec::default(),
        };

        db_airbases.iter().for_each(|(key, value)| match value {
            AirBase::Fixed(item) => {
                let mut item = item.clone();
                // fix sides as we go
                let warehouse = warehouses
                    .airports
                    .get(&item.airdrome_id)
                    .expect("Airport must have an entry in warehouses");
                item.side = warehouse.coalition.to_lowercase();
                item._name = key.to_owned();
                result.fixed.push(item);
            }
            AirBase::Ship(item) => {
                let mut item = item.clone();
                item._name = key.to_owned();
                result.ship.push(item);
            }
            AirBase::Farp(item) => {
                let mut item = item.clone();
                item._name = key.to_owned();
                result.farp.push(item);
            }
            AirBase::Reserve(item) => {
                let mut item = item.clone();
                item._name = key.to_owned();
                result.reserve.push(item);
            }
            AirBase::AirStart(item) => {
                let mut item = item.clone();
                item._name = key.to_owned();
                result.air_start.push(item);
            }
        });

        result
    }

    pub fn to_db_airbases(&self) -> DBAirbases {
        let mut result = DBAirbases::default();

        self.fixed.iter().for_each(|item| {
            result.insert(item._name.to_owned(), AirBase::Fixed(item.clone()));
        });

        self.ship.iter().for_each(|item| {
            result.insert(item._name.to_owned(), AirBase::Ship(item.clone()));
        });

        self.farp.iter().for_each(|item| {
            result.insert(item._name.to_owned(), AirBase::Farp(item.clone()));
        });

        self.reserve.iter().for_each(|item| {
            result.insert(item._name.to_owned(), AirBase::Reserve(item.clone()));
        });

        self.air_start.iter().for_each(|item| {
            result.insert(item._name.to_owned(), AirBase::AirStart(item.clone()));
        });

        result
    }

    pub fn airbase_exists(&self, name: &str) -> bool {
        if self.fixed.iter().any(|n| n._name == name) {
            return true;
        }
        if self.ship.iter().any(|n| n._name == name) {
            return true;
        }
        if self.air_start.iter().any(|n| n._name == name) {
            return true;
        }
        if self.reserve.iter().any(|n| n._name == name) {
            return true;
        }
        if self.farp.iter().any(|n| n._name == name) {
            return true;
        }

        false
    }
}

impl Mappables for DBAirbasesInternal {
    fn to_mappables(
        &self,
        instance: &crate::DCEInstance,
        proj: &Proj,
    ) -> Vec<crate::mappable::MapPoint> {
        let mut result = Vec::default();

        self.fixed.iter().for_each(|item| {
            result.push(MapPoint::new_from_dcs(
                item.x,
                item.y,
                &item._name,
                &item.side,
                "FixedAirBase",
                proj,
            ));
        });

        self.ship.iter().for_each(|item| {
            let groups = instance.mission.get_ship_groups();
            let unit = groups
                .iter()
                .flat_map(|g| g.units.as_slice())
                .find(|s| s.name == item.unitname);
            if let Some(unit) = unit {
                result.push(MapPoint::new_from_dcs(
                    unit.x,
                    unit.y,
                    &item._name,
                    &item.side,
                    "ShipAirBase",
                    proj,
                ));
            }
        });

        self.farp.iter().for_each(|item| {
            result.push(MapPoint::new_from_dcs(
                item.x,
                item.y,
                &item._name,
                &item.side,
                "FARP",
                proj,
            ));
        });

        self.air_start.iter().for_each(|item| {
            result.push(MapPoint::new_from_dcs(
                item.x,
                item.y,
                &item._name,
                &item.side,
                "Airstart",
                proj,
            ));
        });

        result
    }
}
