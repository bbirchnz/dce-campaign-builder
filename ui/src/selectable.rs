use dce_lib::{
    campaign_header::Header,
    db_airbases::FixedAirBase,
    loadouts::{CAPLoadout, StrikeLoadout},
    mappable::MapPoint,
    oob_air::Squadron,
    target_list::{Strike, CAP},
    DCEInstance,
};

#[derive(Clone, PartialEq)]
pub enum Selectable {
    Squadron(Squadron),
    TargetStrike(Strike),
    TargetCAP(CAP),
    FixedAirBase(FixedAirBase),
    CampaignSettings(Header),
    LoadoutCAP(CAPLoadout),
    LoadoutStrike(StrikeLoadout),
    None,
}

#[derive(PartialEq, Clone)]
pub enum ValidationResult {
    Pass,
    Fail(Vec<ValidationError>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ValidationError {
    pub field_name: String,
    pub display_name: String,
    pub error: String,
}

impl ValidationError {
    pub fn new(field_name: &str, display_name: &str, error: &str) -> ValidationError {
        ValidationError {
            field_name: field_name.to_owned(),
            display_name: display_name.to_owned(),
            error: error.to_owned(),
        }
    }
}

impl Selectable {
    pub fn from_map(map_point: &MapPoint, instance: &DCEInstance) -> Selectable {
        match map_point.class.as_str() {
            "TargetCAP" => {
                let cap = instance
                    .target_list
                    .cap
                    .iter()
                    .find(|c| c._name == map_point.name)
                    .unwrap()
                    .clone();
                Selectable::TargetCAP(cap)
            }
            "TargetStrike" => {
                let item = instance
                    .target_list
                    .strike
                    .iter()
                    .find(|c| c._name == map_point.name)
                    .unwrap()
                    .clone();
                Selectable::TargetStrike(item)
            }
            "Squadron" => {
                let item = instance
                    .oob_air
                    .blue
                    .iter()
                    .chain(instance.oob_air.red.iter())
                    .find(|c| c.name == map_point.name)
                    .unwrap()
                    .clone();
                Selectable::Squadron(item)
            }
            "FixedAirBase" => {
                let item = instance
                    .airbases
                    .fixed
                    .iter()
                    .find(|item| item._name == map_point.name)
                    .unwrap()
                    .clone();
                Selectable::FixedAirBase(item)
            }
            _ => Selectable::None,
        }
    }
}

pub trait ToSelectable {
    fn to_selectable(&self) -> Selectable;

    fn get_mut_by_name<'a>(instance: &'a mut DCEInstance, name: &str) -> &'a mut Self;

    fn from_selectable(sel: &Selectable) -> Option<Self>
    where
        Self: Sized;

    fn get_name(&self) -> String;

    fn validate(&self, instance: &DCEInstance) -> ValidationResult;
}

impl ToSelectable for Squadron {
    fn to_selectable(&self) -> Selectable {
        Selectable::Squadron(self.clone())
    }

    fn get_mut_by_name<'a>(instance: &'a mut DCEInstance, name: &str) -> &'a mut Squadron {
        instance
            .oob_air
            .red
            .iter_mut()
            .chain(instance.oob_air.blue.iter_mut())
            .find(|s| s.name == name)
            .unwrap()
    }

    fn from_selectable(sel: &Selectable) -> Option<Self> {
        if let Selectable::Squadron(squad) = sel {
            return Some(squad.clone());
        }
        None
    }

    fn get_name(&self) -> String {
        self.name.to_string()
    }

    fn validate(&self, instance: &DCEInstance) -> ValidationResult {
        let mut errors = Vec::default();

        if !instance.airbases.airbase_exists(&self.base) {
            errors.push(ValidationError::new(
                "base",
                "Airbase Name",
                "Airbase must be a fixed airbase, ship, farp, reserve or airstart",
            ));
        }

        if errors.is_empty() {
            return ValidationResult::Pass;
        }
        ValidationResult::Fail(errors)
    }
}

impl ToSelectable for Strike {
    fn to_selectable(&self) -> Selectable {
        Selectable::TargetStrike(self.clone())
    }

    fn get_mut_by_name<'a>(instance: &'a mut DCEInstance, name: &str) -> &'a mut Self {
        instance
            .target_list
            .strike
            .iter_mut()
            .find(|s| s._name == name)
            .unwrap()
    }

    fn from_selectable(sel: &Selectable) -> Option<Self>
    where
        Self: Sized,
    {
        if let Selectable::TargetStrike(t) = sel {
            return Some(t.clone());
        }
        None
    }

    fn get_name(&self) -> String {
        self.text.to_string()
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
        if let Some(vg_name) = self.class_template.clone() {
            match self.class.as_str() {
                "vehicle" => {
                    if let None = instance
                        .mission
                        .get_vehicle_groups()
                        .iter()
                        .find(|g| g.name == vg_name)
                    {
                        errors.push(ValidationError::new(
                            "class_template",
                            "Target group name",
                            "Target group must be a vehicle group name defined in base_mission.miz",
                        ));
                    }
                }
                "ship" => {
                    if let None = instance
                        .mission
                        .get_ship_groups()
                        .iter()
                        .find(|g| g.name == vg_name)
                    {
                        errors.push(ValidationError::new(
                            "class_template",
                            "Target group name",
                            "Target group must be a ship group name defined in base_mission.miz",
                        ));
                    }
                }
                _ => {
                    errors.push(ValidationError::new(
                        "class",
                        "Target Class",
                        "Target class must be vehicle or ship",
                    ));
                }
            }
        }

        if errors.is_empty() {
            return ValidationResult::Pass;
        }
        ValidationResult::Fail(errors)
    }
}

impl ToSelectable for CAP {
    fn to_selectable(&self) -> Selectable {
        Selectable::TargetCAP(self.clone())
    }

    fn get_mut_by_name<'a>(instance: &'a mut DCEInstance, name: &str) -> &'a mut Self {
        instance
            .target_list
            .cap
            .iter_mut()
            .find(|s| s._name == name)
            .unwrap()
    }

    fn from_selectable(sel: &Selectable) -> Option<Self>
    where
        Self: Sized,
    {
        if let Selectable::TargetCAP(t) = sel {
            return Some(t.clone());
        }
        None
    }

    fn get_name(&self) -> String {
        self.text.to_string()
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
        if let Err(_) = instance.mission.get_zone_by_name(&self.ref_point) {
            errors.push(ValidationError::new(
                "ref_point",
                "CAP Reference Zone",
                "CAP reference zone must exist in base_mission.miz",
            ));
        }
        if errors.is_empty() {
            return ValidationResult::Pass;
        }
        ValidationResult::Fail(errors)
    }
}

impl ToSelectable for FixedAirBase {
    fn to_selectable(&self) -> Selectable {
        Selectable::FixedAirBase(self.clone())
    }

    fn get_mut_by_name<'a>(instance: &'a mut DCEInstance, name: &str) -> &'a mut Self {
        instance
            .airbases
            .fixed
            .iter_mut()
            .find(|item| item._name == name)
            .unwrap()
    }

    fn from_selectable(sel: &Selectable) -> Option<Self>
    where
        Self: Sized,
    {
        if let Selectable::FixedAirBase(t) = sel {
            return Some(t.clone());
        }
        None
    }

    fn get_name(&self) -> String {
        self._name.to_owned()
    }

    fn validate(&self, _: &DCEInstance) -> ValidationResult {
        let mut errors = Vec::default();

        if self.side != "blue" && self._name == "red" {
            errors.push(ValidationError::new(
                "_side",
                "Airbase Side",
                "Side must be blue or red",
            ));
        }

        if errors.is_empty() {
            return ValidationResult::Pass;
        }
        ValidationResult::Fail(errors)
    }
}

impl ToSelectable for Header {
    fn to_selectable(&self) -> Selectable {
        Selectable::CampaignSettings(self.clone())
    }

    fn get_mut_by_name<'a>(instance: &'a mut DCEInstance, _: &str) -> &'a mut Self {
        &mut instance.campaign_header
    }

    fn from_selectable(sel: &Selectable) -> Option<Self>
    where
        Self: Sized,
    {
        if let Selectable::CampaignSettings(header) = sel {
            return Some(header.clone());
        }
        None
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

        if errors.is_empty() {
            return ValidationResult::Pass;
        }
        ValidationResult::Fail(errors)
    }
}

impl ToSelectable for CAPLoadout {
    fn to_selectable(&self) -> Selectable {
        Selectable::LoadoutCAP(self.to_owned())
    }

    fn get_mut_by_name<'a>(instance: &'a mut DCEInstance, name: &str) -> &'a mut Self {
        instance
            .loadouts
            .cap
            .iter_mut()
            .find(|item| item._name == name)
            .unwrap()
    }

    fn from_selectable(sel: &Selectable) -> Option<Self>
    where
        Self: Sized,
    {
        if let Selectable::LoadoutCAP(cap) = sel {
            return Some(cap.clone());
        }
        None
    }

    fn get_name(&self) -> String {
        self._name.to_owned()
    }

    fn validate(&self, _: &DCEInstance) -> ValidationResult {
        let errors = Vec::default();

        // todo: Probably want to put some limits on speeds/altitudes

        if errors.is_empty() {
            return ValidationResult::Pass;
        }
        ValidationResult::Fail(errors)
    }
}

impl ToSelectable for StrikeLoadout {
    fn to_selectable(&self) -> Selectable {
        Selectable::LoadoutStrike(self.to_owned())
    }

    fn get_mut_by_name<'a>(instance: &'a mut DCEInstance, name: &str) -> &'a mut Self {
        instance
            .loadouts
            .strike
            .iter_mut()
            .find(|item| item._name == name)
            .unwrap()
    }

    fn from_selectable(sel: &Selectable) -> Option<Self>
    where
        Self: Sized,
    {
        if let Selectable::LoadoutStrike(strike) = sel {
            return Some(strike.clone());
        }
        None
    }

    fn get_name(&self) -> String {
        self._name.to_owned()
    }

    fn validate(&self, _: &DCEInstance) -> ValidationResult {
        let errors = Vec::default();

        // todo: Probably want to put some limits on speeds/altitudes
        if errors.is_empty() {
            return ValidationResult::Pass;
        }
        ValidationResult::Fail(errors)
    }
}
