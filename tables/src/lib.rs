use bevy_reflect::Struct;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub trait Table
where
    Self: Struct + std::fmt::Debug,
{
    fn get_header() -> Vec<HeaderField>;

    fn get_field(&self, header: &HeaderField) -> String {
        match header.type_ {
            FieldType::String => self
                .field(&header.field)
                .unwrap()
                .downcast_ref::<String>()
                .unwrap()
                .to_string(),
            FieldType::Float => self
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
                let v = self
                    .field(&header.field)
                    .unwrap();
                format!("{:?}", v)
            }
        }
    }
}

pub struct HeaderField {
    pub field: String,
    pub display: String,
    pub type_: FieldType,
}

pub enum FieldType {
    String,
    Float,
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
