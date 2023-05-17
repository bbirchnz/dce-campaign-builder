use serde::{Deserialize, Serialize};

use crate::serde_utils::LuaFileBased;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct CMPFile {
    pub picture: String,
    #[serde(rename = "startStage")]
    pub start_stage: u32,
    pub name: String,
    pub description: String,
    #[serde(rename = "necessaryUnits")]
    pub necessary_units: Vec<String>,
    pub stages: Vec<Stage>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Stage {
    pub name: String,
    pub missions: Vec<Mission>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Mission {
    interval: Vec<u32>,
    file: String,
    description: String,
}

impl CMPFile {
    pub fn new(name: String) -> CMPFile {
        CMPFile {
            picture: "dummy.png".into(),
            start_stage: 1,
            name: name.to_string(),
            description: "".into(),
            necessary_units: Vec::default(),
            stages: vec![
                Stage {
                    name: "Stage 1".into(),
                    missions: vec![Mission {
                        interval: vec![0, 100],
                        file: name.to_string() + "_first.miz",
                        description: "".into(),
                    }],
                },
                Stage {
                    name: "Stage 2".into(),
                    missions: vec![Mission {
                        interval: vec![0, 100],
                        file: name + "_ongoing.miz",
                        description: "".into(),
                    }],
                },
            ],
        }
    }
}

impl LuaFileBased<'_> for CMPFile {}
