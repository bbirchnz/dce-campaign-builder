use anyhow::anyhow;
use bevy_reflect::Struct;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub trait TableHeader {
    fn get_header() -> Vec<HeaderField>;
}

pub trait Table
where
    Self: Struct + std::fmt::Debug + PartialEq,
{
    fn get_field(&self, header: &HeaderField) -> String {
        match header.type_ {
            FieldType::String => self
                .field(&header.field)
                .unwrap()
                .downcast_ref::<String>()
                .unwrap()
                .to_string(),
            FieldType::Float(_) => self
                .field(&header.field)
                .unwrap()
                .downcast_ref::<f64>()
                .unwrap()
                .to_string(),
            FieldType::Int => self
                .field(&header.field)
                .unwrap()
                .downcast_ref::<u32>()
                .expect(&format!("Failed to get field {} as u32", &header.field))
                .to_string(),
            FieldType::Enum => "".into(),
            FieldType::VecString => self
                .field(&header.field)
                .unwrap()
                .downcast_ref::<Vec<String>>()
                .expect(&format!(
                    "Failed to get field {} as Vec<String>",
                    &header.field
                ))
                .join(", "),
            FieldType::Debug => {
                let v = self.field(&header.field).unwrap();
                format!("{:?}", v)
            }
        }
    }
}

#[derive(PartialEq)]
pub struct HeaderField {
    pub field: String,
    pub display: String,
    pub type_: FieldType,
    pub editable: bool,
}

impl HeaderField {
    pub fn get_value_string(&self, item: &dyn Struct) -> String {
        match self.type_ {
            FieldType::String => item
                .field(&self.field)
                .unwrap()
                .downcast_ref::<String>()
                .unwrap()
                .to_string(),
            FieldType::Float(func) => {
                let value = item
                    .field(&self.field)
                    .unwrap()
                    .downcast_ref::<f64>()
                    .expect(&format!("Failed to get field {} as f64", &self.field));
                func(*value)
            } // .to_string(),
            FieldType::Int => item
                .field(&self.field)
                .unwrap()
                .downcast_ref::<u32>()
                .expect(&format!("Failed to get field {} as u32", &self.field))
                .to_string(),
            FieldType::Enum => "".into(),
            FieldType::VecString => item
                .field(&self.field)
                .unwrap()
                .downcast_ref::<Vec<String>>()
                .expect(&format!(
                    "Failed to get field {} as Vec<String>",
                    &self.field
                ))
                .join(", "),
            FieldType::Debug => {
                let v = item.field(&self.field).unwrap();
                format!("{:?}", v)
            }
        }
    }

    pub fn set_value_fromstr(
        &self,
        item: &mut dyn Struct,
        value: &str,
    ) -> Result<(), anyhow::Error> {
        match self.type_ {
            FieldType::String => {
                item.field_mut(&self.field)
                    .ok_or(anyhow!("Couldn't get field {}", &self.field))?
                    .apply(&value.to_owned());
            }
            FieldType::Float(_) => {
                item.field_mut(&self.field)
                    .ok_or(anyhow!("Couldn't get field {}", &self.field))?
                    .apply(&value.parse::<f64>()?);
            }
            FieldType::Int => {
                item.field_mut(&self.field)
                    .ok_or(anyhow!("Couldn't get field {}", &self.field))?
                    .apply(&value.parse::<u32>()?);
            }
            FieldType::Enum => todo!(),
            FieldType::VecString => todo!(),
            FieldType::Debug => todo!(),
        };
        Ok(())
    }
}

#[derive(PartialEq)]
pub enum FieldType {
    String,
    Float(fn(f64) -> String),
    Int,
    Enum,
    VecString,
    Debug,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
