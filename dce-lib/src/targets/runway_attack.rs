use bevy_reflect::{FromReflect, Reflect};
use serde::{Deserialize, Serialize};

use crate::{
    editable::{
        Editable, FieldType, HeaderField, NestedEditable, ValidationError, ValidationResult, AllEntityTemplateAction,
    },
    DCEInstance,
};

use super::TargetFirepower;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Reflect, FromReflect)]
pub struct RunwayAttack {
    pub firepower: TargetFirepower,
    #[serde(rename = "db_airbaseName")]
    pub airbase_name: String,
    pub priority: u32,
    #[serde(default)]
    pub text: String,
    #[serde(default)]
    pub inactive: bool,
    #[serde(default)]
    pub _name: String,
    #[serde(default)]
    pub _side: String,
    #[serde(default)]
    pub attributes: Vec<String>,
    #[serde(default)]
    pub picture: Vec<String>,
}

impl Editable for RunwayAttack {
    fn get_name(&self) -> String {
        self._name.to_string()
    }

    fn validate(&self, instance: &crate::DCEInstance) -> crate::editable::ValidationResult {
        let mut errors = Vec::default();

        if self._side != "blue" && self._name == "red" {
            errors.push(ValidationError::new(
                "_side",
                "Target Side",
                "Side must be blue or red",
            ));
        }

        if !instance
            .airbases
            .fixed
            .iter()
            .any(|f| f.get_name() == self.airbase_name)
        {
            errors.push(ValidationError::new(
                "class_template",
                "Target group name",
                "Target group must be a fixed airbase name if class is airbase",
            ))
        }
        // this will often have just a single empty string if nothing in UI
        if self.picture.len() != 1 || self.picture[0].len() > 0 {
            for p in &self.picture {
                if !instance.bin_data.images.iter().any(|i| &i.name == p) {
                    errors.push(ValidationError::new(
                        "picture",
                        "Briefing Images",
                        &format!("{} is not a valid image", p),
                    ));
                }
            }
        }

        if errors.is_empty() {
            return ValidationResult::Pass;
        }
        ValidationResult::Fail(errors)
    }

    fn get_mut_by_name<'a>(instance: &'a mut crate::DCEInstance, name: &str) -> &'a mut Self
    where
        Self: Sized,
    {
        instance
            .target_list
            .runway_attack
            .iter_mut()
            .find(|s| s._name == name)
            .unwrap()
    }

    fn get_header() -> Vec<crate::editable::HeaderField>
    where
        Self: Sized,
    {
        vec![
            HeaderField::new("text", "Display Text", FieldType::String, true),
            HeaderField::new("_side", "Side", FieldType::String, false),
            HeaderField::new("priority", "Priority", FieldType::Int, true),
            HeaderField::new(
                "firepower",
                "Firepower Required",
                FieldType::NestedEditable(TargetFirepower::get_header()),
                true,
            ),
            HeaderField::new("inactive", "Inactive", FieldType::Bool, true),
            HeaderField::new("airbase_name", "Airbase Name", FieldType::String, true),
            HeaderField::new(
                "picture",
                "Briefing Images",
                FieldType::VecStringOptions(|instance| {
                    instance
                        .bin_data
                        .images
                        .iter()
                        .map(|i| i.name.to_owned())
                        .collect::<Vec<_>>()
                }),
                true,
            ),
            HeaderField::new("attributes", "Loadout Tags", FieldType::VecString, true),
        ]
    }

    fn delete_by_name(instance: &mut crate::DCEInstance, name: &str) -> Result<(), anyhow::Error>
    where
        Self: Sized,
    {
        let container = &mut instance.target_list.runway_attack;

        if let Some(index) = container.iter().position(|i| i._name == name) {
            container.remove(index);
            return Ok(());
        }

        Err(anyhow::anyhow!("Didn't find {}", name))
    }

    fn actions_all_entities() -> Vec<crate::editable::AllEntityTemplateAction> {
        vec![AllEntityTemplateAction::new(
            "Generate Runway Attacks",
            "Generates a runway attack for all airbases with squadrons",
            RunwayAttack::generate_runway_strikes,
        )]
    }
}

impl RunwayAttack {
    pub fn generate_runway_strikes(instance: &mut DCEInstance) -> Result<(), anyhow::Error> {
        let mut new_attacks: Vec<RunwayAttack> = Vec::default();
        
        // for each airbase with a squadron, generate a runway attack target
        instance
            .airbases
            .fixed
            .iter()
            .filter(|fixed| {
                fixed.side != "neutral"
                    && instance.oob_air.squadrons_for_airbase(&fixed._name).len() > 0
            })
            .for_each(|fixed| {
                let name = fixed.get_name() + " Runway Attack";
                new_attacks.push(RunwayAttack {
                    priority: 1,
                    text: name.to_owned(),
                    inactive: false,
                    firepower: TargetFirepower { min: 2, max: 8 },
                    _name: name.to_owned(),
                    _side: if fixed.side == "red" {
                        "blue".to_string()
                    } else {
                        "red".to_string()
                    },
                    attributes: vec!["runway".into()],
                    picture: Vec::default(),
                    airbase_name: fixed.get_name(),
                })
            });

        let attacks = &mut instance.target_list.runway_attack;
        attacks.append(&mut new_attacks);

        Ok(())
    }
}
