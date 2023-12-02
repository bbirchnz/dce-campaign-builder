/// Projections generated using methods from [PyDCS](https://github.com/pydcs/dcs)
use anyhow::anyhow;
use proj::Proj;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct TransverseMercator {
    central_meridian: i16,
    false_easting: f64,
    false_northing: f64,
    scale_factor: f64,
}

pub const PG: TransverseMercator = TransverseMercator {
    central_meridian: 57,
    false_easting: 75755.99999999645,
    false_northing: -2894933.0000000377,
    scale_factor: 0.9996,
};

pub const SA: TransverseMercator = TransverseMercator {
    central_meridian: -57,
    false_easting: 147639.99999997593,
    false_northing: 5815417.000000032,
    scale_factor: 0.9996,
};

pub const CAUC: TransverseMercator = TransverseMercator {
    central_meridian: 33,
    false_easting: -99516.99999997323,
    false_northing: -4998114.999999984,
    scale_factor: 0.9996,
};

pub const MAR: TransverseMercator = TransverseMercator {
    central_meridian: 147,
    false_easting: 238417.99999989968,
    false_northing: -1491840.000000048,
    scale_factor: 0.9996,
};

pub const NV: TransverseMercator = TransverseMercator {
    central_meridian: -117,
    false_easting: -193996.80999964548,
    false_northing: -4410028.063999966,
    scale_factor: 0.9996,
};

pub const NORM: TransverseMercator = TransverseMercator {
    central_meridian: -3,
    false_easting: -195526.00000000204,
    false_northing: -5484812.999999951,
    scale_factor: 0.9996,
};

pub const SY: TransverseMercator = TransverseMercator {
    central_meridian: 39,
    false_easting: 282801.00000003993,
    false_northing: -3879865.9999999935,
    scale_factor: 0.9996,
};

pub const SI: TransverseMercator = TransverseMercator {
    central_meridian: 33,
    false_easting: 169221.9999999585,
    false_northing: -3325312.9999999693,
    scale_factor: 0.9996,
};

pub fn projection_from_theatre(theatre: &str) -> Result<TransverseMercator, anyhow::Error> {
    match theatre {
        "PersianGulf" => Ok(PG),
        "Falklands" => Ok(SA),
        "Caucasus" => Ok(CAUC),
        "MarianaIslands" => Ok(MAR),
        "Nevada" => Ok(NV),
        "Normandy" => Ok(NORM),
        "Syria" => Ok(SY),
        "SinaiMap" => Ok(SI),
        _ => Err(anyhow!("TransverseMercator not known for {}", theatre)),
    }
}

pub fn proj_from_map(map: &TransverseMercator) -> Result<Proj, anyhow::Error> {
    Proj::new_known_crs(
        &format!(
            "+proj=tmerc +lat_0=0 +lon_0={} +k_0={} +x_0={} +y_0={}",
            map.central_meridian, map.scale_factor, map.false_easting, map.false_northing
        ),
        "WGS84",
        None,
    )
    .map_err(|e| anyhow!("{:?}", e))
}

pub fn convert_dcs_lat_lon(x: f64, y: f64, proj: &Proj) -> (f64, f64) {
    proj.convert((y, x)).unwrap()
}

pub fn offset(x_init: f64, y_init: f64, axis_deg: f64, distance_m: f64) -> (f64, f64) {
    let axis_rad = (axis_deg - 0.).to_radians();
    let x2 = x_init + (distance_m * axis_rad.cos());
    let y2 = y_init + (distance_m * axis_rad.sin());
    (x2, y2)
}

#[cfg(test)]
mod tests {
    use super::{convert_dcs_lat_lon, offset};
    use crate::projections::{proj_from_map, PG};
    use approx_eq::assert_approx_eq;

    #[test]
    fn can_convert_to_lat_lon() {
        let (x, y) =
            convert_dcs_lat_lon(-100594.371094, -88875.371094, &proj_from_map(&PG).unwrap());

        assert_approx_eq!(x, 55.3652612);
        assert_approx_eq!(y, 25.25637587);
    }

    #[test]
    fn add_dist_90deg() {
        let (x, y) = (10., 20.);
        let (x2, y2) = offset(x, y, 90., 10.);
        assert_approx_eq!(x2, 20.);
        assert_approx_eq!(y2, 20.);
    }

    #[test]
    fn add_dist_180deg() {
        let (x, y) = (10., 20.);
        let (x2, y2) = offset(x, y, 180., 10.);
        assert_approx_eq!(x2, 10.);
        assert_approx_eq!(y2, 10.);
    }

    #[test]
    fn add_dist_0deg() {
        let (x, y) = (10., 20.);
        let (x2, y2) = offset(x, y, 0., 10.);
        assert_approx_eq!(x2, 10.);
        assert_approx_eq!(y2, 30.);
    }

    #[test]
    fn add_dist_270deg() {
        let (x, y) = (10., 20.);
        let (x2, y2) = offset(x, y, 270., 10.);
        assert_approx_eq!(x2, 0.);
        assert_approx_eq!(y2, 20.);
    }
}
