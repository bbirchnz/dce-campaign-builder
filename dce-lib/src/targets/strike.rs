use super::TargetFirepower;
use crate::{
    editable::{
        AllEntityTemplateAction, Editable, EntityTemplateAction, FieldType, HeaderField,
        NestedEditable, ValidationError, ValidationResult,
    },
    target_list::TargetList,
    target_list_internal::TargetListInternal,
    trigger::{Actions, Trigger},
    DCEInstance, NewFromMission,
};
use anyhow::anyhow;
use bevy_reflect::{FromReflect, Reflect};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Reflect, FromReflect)]
pub struct Strike {
    pub priority: u32,
    #[serde(default)]
    pub text: String,
    #[serde(default)]
    pub inactive: bool,
    pub firepower: TargetFirepower,
    #[serde(default = "default_class")]
    pub class: Option<String>,
    #[serde(rename = "name")]
    pub class_template: Option<String>,
    pub elements: Option<Vec<StrikeElement>>,
    #[serde(default)]
    pub _name: String,
    #[serde(default)]
    pub _side: String,
    #[serde(default)]
    pub attributes: Vec<String>,
    #[serde(default)]
    pub picture: Vec<String>,
}

fn default_class() -> Option<String> {
    Some("static".to_string())
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Reflect, FromReflect)]
#[serde(untagged)]
pub enum StrikeElement {
    FixedCoord(StrikeFixedCoordTarget),
    NamedStatic(StrikeNamedStaticTarget),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Reflect, FromReflect)]
pub struct StrikeFixedCoordTarget {
    pub name: String,
    pub x: f64,
    pub y: f64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Reflect, FromReflect)]
pub struct StrikeNamedStaticTarget {
    pub name: String,
}

impl Editable for Strike {
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
            HeaderField::new("inactive", "Inactive", FieldType::Bool, true),
            HeaderField::new(
                "class_template",
                "DCS Group Name",
                FieldType::OptionString,
                false,
            ),
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
    fn get_mut_by_name<'a>(instance: &'a mut DCEInstance, name: &str) -> &'a mut Self {
        instance
            .target_list
            .strike
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

        if let Some(vg_name) = self.class_template.clone() {
            match self.class.as_ref() {
                Some(class) if class.as_str() == "vehicle" => {
                    if !instance
                        .miz_env
                        .mission
                        .get_vehicle_groups()
                        .iter()
                        .any(|g| g.name == vg_name)
                    {
                        errors.push(ValidationError::new(
                            "class_template",
                            "Target group name",
                            "Target group must be a vehicle group name defined in base_mission.miz",
                        ));
                    }
                }
                Some(class) if class.as_str() == "ship" => {
                    if !instance
                        .miz_env
                        .mission
                        .get_ship_groups()
                        .iter()
                        .any(|g| g.name == vg_name)
                    {
                        errors.push(ValidationError::new(
                            "class_template",
                            "Target group name",
                            "Target group must be a ship group name defined in base_mission.miz",
                        ));
                    }
                }
                Some(class) if class.as_str() == "airbase" => {
                    if !instance
                        .airbases
                        .fixed
                        .iter()
                        .any(|f| f.get_name() == vg_name)
                    {
                        errors.push(ValidationError::new(
                            "class_template",
                            "Target group name",
                            "Target group must be a fixed airbase name if class is airbase",
                        ))
                    }
                }
                _ => {
                    // only an error if elements is None
                    if self.elements.is_none() {
                        errors.push(ValidationError::new(
                            "class",
                            "Target Class",
                            "Target class must be vehicle, ship, or airbase",
                        ));
                    }
                }
            }
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

    fn can_reset_from_miz() -> bool {
        true
    }

    fn reset_all_from_miz(instance: &mut DCEInstance) -> Result<(), anyhow::Error> {
        let new_target_list =
            TargetListInternal::from_target_list(&TargetList::new_from_mission(&instance.miz_env)?);

        instance.target_list.strike = new_target_list.strike;

        Ok(())
    }

    fn delete_by_name(instance: &mut DCEInstance, name: &str) -> Result<(), anyhow::Error> {
        let container = &mut instance.target_list.strike;

        if let Some(index) = container.iter().position(|i| i._name == name) {
            container.remove(index);
            return Ok(());
        }

        Err(anyhow!("Didn't find {}", name))
    }

    fn actions_one_entity() -> Vec<crate::editable::EntityTemplateAction<Self>>
    where
        Self: Sized,
    {
        let hide_action = EntityTemplateAction::new("Hide Target", "Hide and disable the target and its associated group by creating a set of triggers that can be adjusted", 
        |item: &mut Strike, instance| {

            if item.class.is_some() && item.class.as_ref().unwrap() == "vehicle" && item.class_template.is_some() {
                let actions = vec![
                    // target inactive
                    format!("Action.TargetActive(\"{}\", false)", item.get_name()),
                    // hide group in mission editor
                    format!("Action.GroupHidden(\"{}\", true)", item.class_template.as_ref().unwrap()),
                    // set group probability to zero (in the mission editor, but won't show in mission)
                    format!("Action.GroupProbability(\"{}\", 0)", item.class_template.as_ref().unwrap()),
                    ];

                    let trigger = Trigger {
                    active: true,
                    once: false,
                    condition: "true".into(),
                    action: Actions::Many(actions),
                    _name: item.text.to_owned() + " - Hide",
                };
                // sets target inactive now, so its obvious in the UI
                item.inactive = true;
                instance.triggers.push(trigger);
                return Ok(())
            }

            Err(anyhow::anyhow!("Only works for Strike targets on ground vehicle groups"))
        });

        vec![hide_action]
    }

    fn actions_all_entities() -> Vec<crate::editable::AllEntityTemplateAction> {
        vec![AllEntityTemplateAction::new(
            "Generate OCA Strikes",
            "Generates an OCA strike for all airbases with squadrons",
            Strike::generate_airbase_strikes,
        )]
    }
}

impl Strike {
    pub fn generate_airbase_strikes(instance: &mut DCEInstance) -> Result<(), anyhow::Error> {
        let mut new_strikes: Vec<Strike> = Vec::default();

        // for each airbase with a squadron, generate a strike target
        instance
            .airbases
            .fixed
            .iter()
            .filter(|fixed| {
                fixed.side != "neutral"
                    && instance.oob_air.squadrons_for_airbase(&fixed._name).len() > 0
            })
            .for_each(|fixed| {
                let name = fixed.get_name() + " OCA Strike";
                new_strikes.push(Strike {
                    priority: 1,
                    text: name.to_owned(),
                    inactive: false,
                    firepower: TargetFirepower { min: 2, max: 2 },
                    class: Some("airbase".to_string()),
                    class_template: Some(fixed.get_name()),
                    elements: None,
                    _name: name.to_owned(),
                    _side: if fixed.side == "red" {
                        "blue".to_string()
                    } else {
                        "red".to_string()
                    },
                    attributes: vec!["parked_aircraft".into()],
                    picture: Vec::default(),
                })
            });

        let strikes = &mut instance.target_list.strike;
        strikes.append(&mut new_strikes);

        Ok(())
    }
}
