use std::{cell::RefMut, collections::HashMap};

use bevy_reflect::Struct;
use dce_lib::editable::{Editable, FieldType};
use itertools::Itertools;

pub fn apply_to_item<T>(item: &mut RefMut<T>, values: &HashMap<String, String>)
where
    T: Struct + Editable,
{
    let headers = T::get_header();
    for h in headers.iter().filter(|h| h.editable) {
        match h.type_ {
            FieldType::TriggerActions
            | FieldType::VecString
            | FieldType::VecStringOptions(_)
            | FieldType::NestedEditable(_) => {
                let values = stringvec_for_field(values, &h.display);

                if let Err(e) = h.set_value_from_stringvec(&mut **item, values) {
                    panic!("Failed to set field: {}. Error: {}", h.field, e);
                }
            }
            _ => {
                let v = values.get(&h.display).unwrap_or_else(|| {
                    panic!(
                        "There must be a value for field {:?} in formevent",
                        &h.field
                    )
                });
                if let Err(e) = h.set_value_fromstr(&mut **item, v) {
                    panic!("Failed to set field: {} with {}. Error: {}", h.field, v, e);
                }
            }
        };
    }
}

fn stringvec_for_field(values: &HashMap<String, String>, display_name: &str) -> Vec<String> {
    values
        .iter()
        .filter_map(|(k, v)| {
            let splits = k.split('.').map(|s| s.to_string()).collect::<Vec<String>>();

            if splits.len() == 2 && splits[0] == display_name && splits[1].parse::<usize>().is_ok()
            {
                return Some((splits[1].parse::<usize>().unwrap(), v.to_owned()));
            }
            None
        })
        .sorted()
        .map(|(_, v)| v)
        .collect::<Vec<String>>()
}
