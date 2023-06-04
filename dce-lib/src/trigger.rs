use std::collections::HashMap;

use bevy_reflect::{FromReflect, Reflect};
use itertools::Itertools;
use mlua::Lua;
use serde::{Deserialize, Serialize};

use crate::{
    editable::{
        AllEntityTemplateAction, Editable, FieldType, HeaderField, ValidationError,
        ValidationResult,
    },
    lua_utils::load_trigger_mocks,
    serde_utils::LuaFileBased,
    DCEInstance, NewFromMission,
};

use anyhow::anyhow;

/// A Hashmap string/trigger as serialized to lua
pub type Triggers = HashMap<String, Trigger>;

/// A Vec of triggers so you don't have to worry about keys
pub type TriggersFlat = Vec<Trigger>;

/// Convert Triggers Hashmap to TriggersFlat vec
/// Can't be a impl function as both types are type aliases
pub fn triggers_to_flat(triggers: &Triggers) -> TriggersFlat {
    triggers
        .iter()
        .map(|(k, v)| {
            let mut v = v.clone();
            v._name = k.to_owned();
            v
        })
        .collect::<Vec<_>>()
}

pub fn flat_to_triggers(flat_triggers: &TriggersFlat) -> Triggers {
    flat_triggers
        .iter()
        .map(|v| (v._name.to_owned(), v.clone()))
        .collect::<HashMap<_, _>>()
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Reflect, FromReflect)]
pub struct Trigger {
    #[serde(default)]
    pub active: bool,
    #[serde(default)]
    pub once: bool,
    pub condition: String,
    pub action: Actions,
    #[serde(default)]
    pub _name: String,
}

impl Trigger {
    pub fn new(name: &str, condition: &str, actions: &[&str], active: bool, once: bool) -> Trigger {
        Trigger {
            active,
            once,
            condition: condition.into(),
            action: Actions::Many(actions.iter().map(|s| s.to_string()).collect_vec()),
            _name: name.into(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Reflect, FromReflect)]
#[serde(untagged)]
pub enum Actions {
    One(String),
    Many(Vec<String>),
}
impl LuaFileBased<'_> for Triggers {}

impl Trigger {
    fn validate_lua(&self) -> Result<(), anyhow::Error> {
        let lua = Lua::new();
        load_trigger_mocks(&lua)?;

        let cond = format!(
            r#"function test()
   return {}
end

assert(type(test()) == 'boolean', "Must return a boolean result")
"#,
            self.condition
        );

        lua.load(&cond).exec()?;

        match &self.action {
            Actions::One(action) => lua.load(action).exec()?,
            Actions::Many(actions) => actions
                .iter()
                .try_for_each(|action| lua.load(action).exec())?,
        }

        Ok(())
    }
}

impl NewFromMission for Triggers {
    fn new_from_mission(_mission: &crate::mission::Mission) -> Result<Self, anyhow::Error>
    where
        Self: Sized,
    {
        Ok(HashMap::from([
            (
                "Campaign Briefing".into(),
                Trigger::new(
                    "Campaign Briefing",
                    "true",
                    &["Action.Text(\"Welcome to your new campaign\")"],
                    true,
                    true,
                ),
            ),
            (
                "Campaign Victory".into(),
                Trigger::new(
                    "Campaign Victory",
                    "GroundTarget[\"blue\"].percent < 40",
                    &[
                        "Action.CampaignEnd(\"win\")",
                        "Action.Text(\"After heavy losses the enemy is waving the white flag!\")",
                    ],
                    true,
                    true,
                ),
            ),
            (
                "Campaign Loss".into(),
                Trigger::new(
                    "Campaign Loss",
                    "GroundTarget[\"red\"].percent < 20",
                    &[
                        "Action.CampaignEnd(\"loss\")",
                        "Action.Text(\"We have suffered heavy losses, its all over!\")",
                    ],
                    true,
                    true,
                ),
            ),
            (
                "Blue Ground Target Briefing Intel".into(),
                Trigger::new(
                    "Blue Ground Target Briefing Intel",
                    "true",
                    &["Action.AddGroundTargetIntel(\"blue\")"],
                    true,
                    false,
                ),
            ),
            (
                "Red Ground Target Briefing Intel".into(),
                Trigger::new(
                    "Red Ground Target Briefing Intel",
                    "true",
                    &["Action.AddGroundTargetIntel(\"red\")"],
                    true,
                    false,
                ),
            ),
            (
                "Ground Unit Repair".into(),
                Trigger::new(
                    "Ground Unit Repair",
                    "true",
                    &["Action.GroundUnitRepair()"],
                    true,
                    false,
                ),
            ),
            (
                "Air Unit Repair".into(),
                Trigger::new(
                    "Air Unit Repair",
                    "true",
                    &["Action.AirUnitRepair()"],
                    true,
                    false,
                ),
            ),
        ]))
    }
}

impl Editable for Trigger {
    fn get_header() -> Vec<HeaderField> {
        vec![
            HeaderField::new("_name", "Name", FieldType::String, true),
            HeaderField::new("active", "Active", FieldType::Bool, true),
            HeaderField::new("once", "Once Only", FieldType::Bool, true),
            HeaderField::new("condition", "Condition", FieldType::String, true),
            HeaderField::new("action", "Actions", FieldType::TriggerActions, true),
        ]
    }
    fn get_name(&self) -> String {
        self._name.to_owned()
    }

    fn validate(&self, _: &crate::DCEInstance) -> ValidationResult {
        let mut errors = Vec::default();

        if let Err(e) = self.validate_lua() {
            errors.push(ValidationError::new(
                "action",
                "Action",
                format!("Lua error: {}", e).as_str(),
            ));
        }

        if self.condition.contains('\'') {
            errors.push(ValidationError::new(
                "condition",
                "Condition",
                "Lua snippets must not contain ', use double quotes",
            ))
        }

        if self.condition.contains(';') {
            errors.push(ValidationError::new(
                "condition",
                "Condition",
                "Lua snippets must not contain ;",
            ))
        }

        // TODO: validate the referenced groups in the conditions and actions

        if errors.is_empty() {
            return ValidationResult::Pass;
        }
        ValidationResult::Fail(errors)
    }

    fn get_mut_by_name<'a>(instance: &'a mut crate::DCEInstance, name: &str) -> &'a mut Self {
        instance
            .triggers
            .iter_mut()
            .find(|item| item._name == name)
            .expect("Item must exist in trigger vec")
    }

    fn can_reset_from_miz() -> bool {
        true
    }

    fn reset_all_from_miz(instance: &mut crate::DCEInstance) -> Result<(), anyhow::Error> {
        let new_triggers = triggers_to_flat(&Triggers::new_from_mission(&instance.mission)?);

        instance.triggers = new_triggers;

        Ok(())
    }

    fn delete_by_name(instance: &mut DCEInstance, name: &str) -> Result<(), anyhow::Error> {
        let container = &mut instance.triggers;

        if let Some(index) = container.iter().position(|i| i._name == name) {
            container.remove(index);
            return Ok(());
        }

        Err(anyhow!("Didn't find {}", name))
    }

    fn can_delete() -> bool {
        true
    }

    fn actions_all_entities() -> Vec<AllEntityTemplateAction> {
        let create_new = AllEntityTemplateAction::new("Add new", "Add a new trigger", |instance| {
            // make sure we haven't already got a "New Action"
            let existing_names = instance
                .triggers
                .iter()
                .filter(|i| i.get_name().contains("New Action"))
                .sorted_by(|i, i2| Ord::cmp(&i.get_name().len(), &i2.get_name().len()))
                .collect::<Vec<_>>();
            let name;
            if existing_names.is_empty() {
                name = "New Action".to_string();
            } else {
                name = existing_names.last().unwrap().get_name() + "_1";
            }
            instance.triggers.push(Trigger {
                active: true,
                once: false,
                condition: "true".into(),
                action: Actions::Many(vec!["".into()]),
                _name: name,
            });
            Ok(())
        });

        vec![create_new]
    }
}

#[cfg(test)]
mod tests {
    use crate::{mission::Mission, serde_utils::LuaFileBased, NewFromMission};

    use super::Triggers;

    #[test]
    fn load_example() {
        let result = Triggers::from_lua_file("C:\\Users\\Ben\\Saved Games\\DCS.openbeta\\Mods\\tech\\DCE\\Missions\\Campaigns\\War over Tchad 1987-Blue-Mirage-F1EE-3-30 Lorraine\\Init\\camp_triggers_init.lua".into(), "camp_triggers".into());

        let _ = result.unwrap();
    }

    #[test]
    fn save_example() {
        let loaded = Triggers::from_lua_file("C:\\Users\\Ben\\Saved Games\\DCS.openbeta\\Mods\\tech\\DCE\\Missions\\Campaigns\\War over Tchad 1987-Blue-Mirage-F1EE-3-30 Lorraine\\Init\\camp_triggers_init.lua".into(), "camp_triggers".into()).unwrap();
        loaded
            .to_lua_file("camp_triggers_sa.lua".into(), "camp".into())
            .unwrap();
    }

    #[test]
    fn from_miz() {
        let mission = Mission::from_miz("C:\\Users\\Ben\\Saved Games\\DCS.openbeta\\Mods\\tech\\DCE\\Missions\\Campaigns\\Falklands v1\\Init\\base_mission.miz".into()).unwrap();
        let triggers = Triggers::new_from_mission(&mission).unwrap();

        triggers
            .to_lua_file("camp_trigger_init_sa.lua".into(), "camp_triggers".into())
            .unwrap();
    }
}
