use bevy_reflect::{FromReflect, Reflect};
use serde::{Deserialize, Serialize};

use super::TargetFirepower;
use crate::{
    editable::{
        AllEntityTemplateAction, Editable, FieldType, HeaderField, NestedEditable, ValidationError,
        ValidationResult,
    },
    DCEInstance,
};
use anyhow::anyhow;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Reflect, FromReflect)]
pub struct Intercept {
    pub priority: u32,
    #[serde(default)]
    pub text: String,
    pub base: String,
    #[serde(default)]
    pub inactive: bool,
    pub radius: f64,
    pub firepower: TargetFirepower,
    #[serde(default)]
    pub _name: String,
    #[serde(default)]
    pub _side: String,
    #[serde(default)]
    pub attributes: Vec<String>,
}

impl Editable for Intercept {
    fn get_header() -> Vec<HeaderField> {
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
            HeaderField::new("radius", "Radius (nm)", FieldType::DistanceNM, true),
            HeaderField::new("inactive", "Inactive", FieldType::Bool, true),
            HeaderField::new("attributes", "Loadout Tags", FieldType::VecString, true),
        ]
    }
    fn get_mut_by_name<'a>(instance: &'a mut DCEInstance, name: &str) -> &'a mut Self {
        instance
            .target_list
            .intercept
            .iter_mut()
            .find(|s| s._name == name)
            .unwrap()
    }

    fn get_name(&self) -> String {
        self._name.to_string()
    }

    fn validate(&self, instance: &DCEInstance) -> ValidationResult {
        let mut errors = Vec::default();

        if self._side != "blue" && self._name == "red" {
            errors.push(ValidationError::new(
                "_side",
                "Target Side",
                "Side must be blue or red",
            ));
        }

        if let ValidationResult::Fail(mut firepower_errors) =
            TargetFirepower::validate(&self.firepower, instance)
        {
            errors.append(&mut firepower_errors)
        }

        if errors.is_empty() {
            return ValidationResult::Pass;
        }
        ValidationResult::Fail(errors)
    }

    fn can_delete() -> bool {
        true
    }

    fn delete_by_name(instance: &mut DCEInstance, name: &str) -> Result<(), anyhow::Error> {
        let container = &mut instance.target_list.intercept;

        if let Some(index) = container.iter().position(|i| i._name == name) {
            container.remove(index);
            return Ok(());
        }

        Err(anyhow!("Didn't find {}", name))
    }

    fn actions_all_entities() -> Vec<AllEntityTemplateAction> {
        vec![AllEntityTemplateAction::new(
            "Generate Intercepts",
            "Generates an intercept task for each base with a capable squadron",
            Intercept::generate_intercepts,
        )]
    }
}

impl Intercept {
    pub fn generate_intercepts(instance: &mut DCEInstance) -> Result<(), anyhow::Error> {
        let mut new_intercepts: Vec<Intercept> = Vec::default();
        // fixed airbases
        instance
            .airbases
            .fixed
            .iter()
            .filter(|fixed| {
                let squadrons = instance.oob_air.squadrons_for_airbase(fixed._name.as_str());
                fixed.side != "neutral"
                    && squadrons.iter().any(|s| s.tasks.contains_key("Intercept"))
            })
            .for_each(|fixed| {
                let name = fixed._name.to_owned() + " Alert";
                new_intercepts.push(Intercept {
                    priority: 5,
                    text: name.to_owned(),
                    base: fixed._name.to_owned(),
                    inactive: false,
                    radius: 200000.,
                    firepower: TargetFirepower { min: 2, max: 2 },
                    _name: name,
                    _side: fixed.side.to_owned(),
                    attributes: Vec::default(),
                })
            });

        // ship bases:
        instance
            .airbases
            .ship
            .iter()
            .filter(|ship| {
                let squadrons = instance.oob_air.squadrons_for_airbase(ship._name.as_str());
                ship.side != "neutral"
                    && squadrons.iter().any(|s| s.tasks.contains_key("Intercept"))
            })
            .for_each(|ship| {
                let name = ship._name.to_owned() + " Alert";
                new_intercepts.push(Intercept {
                    priority: 5,
                    text: name.to_owned(),
                    base: ship._name.to_owned(),
                    inactive: false,
                    radius: 200000.,
                    firepower: TargetFirepower { min: 2, max: 2 },
                    _name: name,
                    _side: ship.side.to_owned(),
                    attributes: Vec::default(),
                })
            });

        let intercepts = &mut instance.target_list.intercept;
        intercepts.append(&mut new_intercepts);

        Ok(())
    }
}
