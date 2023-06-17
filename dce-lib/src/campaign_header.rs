use anyhow::anyhow;
use bevy_reflect::{FromReflect, Reflect};
use chrono::Datelike;
use serde::{Deserialize, Serialize};

use crate::{
    editable::{Editable, FieldType, HeaderField, ValidationError, ValidationResult},
    miz_environment::MizEnvironment,
    serde_utils::LuaFileBased,
    DCEInstance, NewFromMission,
};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Reflect, FromReflect)]
pub struct Header {
    #[serde(rename = "CampaignOriginal")]
    pub original: bool,
    pub title: String,
    pub version: String,
    pub mission: u8,
    pub date: Date,
    pub time: u32,
    pub dawn: u32,
    pub dusk: u32,
    pub mission_duration: u32,
    pub idle_time_min: u32,
    pub idle_time_max: u32,
    pub startup: u32,
    pub units: String,
    pub weather: Weather,
    #[serde(rename = "variation")]
    pub mag_var: f64,
    pub debug: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Reflect, FromReflect)]
pub struct HeaderInternal {
    #[serde(rename = "CampaignOriginal")]
    pub original: bool,
    pub title: String,
    pub version: String,
    // this has to be Reflect, so can't be chrono::NaiveDate
    pub date_str: String,
    pub time: u32,
    pub dawn: u32,
    pub dusk: u32,
    pub mission_duration: u32,
    pub idle_time_min: u32,
    pub idle_time_max: u32,
    pub startup: u32,
    pub units: String,
    pub weather_high_prob: f64,
    pub weather_low_prob: f64,
    pub weather_reference_temp: f64,
    #[serde(rename = "variation")]
    pub mag_var: f64,
    pub debug: bool,
}

impl From<Header> for HeaderInternal {
    fn from(value: Header) -> Self {
        HeaderInternal {
            original: value.original,
            title: value.title,
            version: value.version,
            date_str: chrono::NaiveDate::from_ymd_opt(
                value.date.year,
                value.date.month,
                value.date.day,
            )
            .unwrap_or_else(|| panic!("date should be parseable"))
            .format("%Y-%m-%d")
            .to_string(),
            time: value.time,
            dawn: value.dawn,
            dusk: value.dusk,
            mission_duration: value.mission_duration,
            idle_time_min: value.idle_time_min,
            idle_time_max: value.idle_time_max,
            startup: value.startup,
            units: value.units,
            weather_high_prob: value.weather.high_prob,
            weather_low_prob: value.weather.low_prob,
            weather_reference_temp: value.weather.reference_temp,
            mag_var: value.mag_var,
            debug: value.debug,
        }
    }
}

impl From<HeaderInternal> for Header {
    fn from(value: HeaderInternal) -> Self {
        let date = chrono::NaiveDate::parse_from_str(value.date_str.as_str(), "%Y-%m-%d")
            .expect("Should parse as %Y-%m-%d");
        Header {
            original: value.original,
            title: value.title,
            version: value.version,

            time: value.time,
            dawn: value.dawn,
            dusk: value.dusk,
            mission_duration: value.mission_duration,
            idle_time_min: value.idle_time_min,
            idle_time_max: value.idle_time_max,
            startup: value.startup,
            units: value.units,
            mag_var: value.mag_var,
            debug: value.debug,
            mission: 0,
            date: Date {
                day: date.day(),
                month: date.month(),
                year: date.year(),
            },
            weather: Weather {
                high_prob: value.weather_high_prob,
                low_prob: value.weather_low_prob,
                reference_temp: value.weather_reference_temp,
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Reflect, FromReflect)]
pub struct Date {
    pub day: u32,
    pub month: u32,
    pub year: i32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Reflect, FromReflect)]
pub struct Weather {
    #[serde(rename = "pHigh")]
    pub high_prob: f64,
    #[serde(rename = "pLow")]
    pub low_prob: f64,
    #[serde(rename = "refTemp")]
    pub reference_temp: f64,
}

impl LuaFileBased<'_> for Header {}

impl NewFromMission for Header {
    fn new_from_mission(miz: &MizEnvironment) -> Result<Self, anyhow::Error>
    where
        Self: Sized,
    {
        Ok(Header {
            original: true,
            title: "New Campaign".into(),
            version: "V0.1".into(),
            mission: 1,
            date: Date {
                day: miz.mission.date.day,
                month: miz.mission.date.month,
                year: miz.mission.date.year,
            },
            time: 11700,
            dawn: 19800,
            dusk: 68880,
            mission_duration: 5400,
            idle_time_min: 10800,
            idle_time_max: 14400,
            startup: 600,
            units: "imperial".into(),
            weather: Weather {
                high_prob: 20.,
                low_prob: 80.,
                reference_temp: 8.,
            },
            mag_var: 2.,
            debug: true,
        })
    }
}

impl Editable for HeaderInternal {
    fn get_header() -> Vec<HeaderField> {
        vec![
            HeaderField::new("title", "Title", FieldType::String, true),
            HeaderField::new("version", "Version", FieldType::String, true),
            HeaderField::new("date_str", "Start Date", FieldType::DateStr, true),
            HeaderField::new("dawn", "Dawn", FieldType::IntTime, true),
            HeaderField::new("dusk", "Dusk", FieldType::IntTime, true),
            HeaderField::new(
                "mission_duration",
                "Mission Duration (min)",
                FieldType::DurationMin,
                true,
            ),
            HeaderField::new(
                "startup",
                "Startup Time (min)",
                FieldType::DurationMin,
                true,
            ),
            HeaderField::new("units", "Unit of Measure", FieldType::String, true),
            HeaderField::new(
                "mag_var",
                "Magnetic Variation",
                FieldType::Float(|v| format!("{:.1}", v)),
                true,
            ),
            HeaderField::new(
                "weather_high_prob",
                "Weather - High System %",
                FieldType::Float(|v| format!("{v:.2}")),
                true,
            ),
            HeaderField::new(
                "weather_low_prob",
                "Weather - Low System %",
                FieldType::Float(|v| format!("{v:.2}")),
                true,
            ),
            HeaderField::new(
                "weather_reference_temp",
                "Weather - Temp C",
                FieldType::Float(|v| format!("{v:.1}")),
                true,
            ),
        ]
    }
    fn get_mut_by_name<'a>(instance: &'a mut DCEInstance, _: &str) -> &'a mut Self {
        &mut instance.campaign_header
    }
    fn get_name(&self) -> String {
        "settings".into()
    }

    fn validate(&self, _: &DCEInstance) -> ValidationResult {
        let mut errors = Vec::default();

        if self.dawn >= self.dusk {
            errors.push(ValidationError::new(
                "dawn",
                "Dawn time",
                "Dawn must be earlier than Dusk",
            ));
        }

        if self.units != "imperial" && self.units != "metric" {
            errors.push(ValidationError::new(
                "units",
                "Units of Measure",
                "Units must be 'imperial' or 'metric'",
            ))
        }

        if errors.is_empty() {
            return ValidationResult::Pass;
        }
        ValidationResult::Fail(errors)
    }

    fn delete_by_name(_: &mut DCEInstance, _: &str) -> Result<(), anyhow::Error> {
        Err(anyhow!("Can't delete the campaign settings!"))
    }
}

#[cfg(test)]
mod tests {
    use crate::{miz_environment::MizEnvironment, serde_utils::LuaFileBased, NewFromMission};

    use super::Header;

    #[test]
    fn from_miz() {
        let miz = MizEnvironment::from_miz("test_resources\\base_mission_falklands.miz").unwrap();
        let header = Header::new_from_mission(&miz).unwrap();

        header
            .to_lua_file("camp_init_sa.lua".into(), "camp".into())
            .unwrap();
    }
}
