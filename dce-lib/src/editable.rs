use crate::DCEInstance;

pub trait Editable {
    fn get_name(&self) -> String;

    fn validate(&self, instance: &DCEInstance) -> ValidationResult;

    fn get_mut_by_name<'a>(instance: &'a mut DCEInstance, name: &str) -> &'a mut Self;
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
