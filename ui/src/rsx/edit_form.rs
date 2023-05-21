use bevy_reflect::Struct;

use dioxus::prelude::*;
use fermi::use_atom_ref;
use log::warn;
use tables::{FieldType, HeaderField, TableHeader};

use crate::{
    selectable::{Selectable, ToSelectable},
    INSTANCE, SELECTED,
};

fn fieldtype_to_input(field: &FieldType) -> String {
    match field {
        FieldType::String => "text".into(),
        FieldType::Float(_) => "number".into(),
        FieldType::Int => "number".into(),
        FieldType::Enum => "text".into(),
        FieldType::VecString => "text".into(),
        FieldType::Debug => "text".into(),
        FieldType::IntTime => "time".into(),
        FieldType::Bool => "checkbox".into(),
        FieldType::AltitudeFeet => "number".into(),
        FieldType::SpeedKnotsTAS => "number".into(),
        FieldType::DistanceNM => "number".into(),
        FieldType::DurationMin => "number".into(),
    }
}

fn fieldtype_editable(field: &FieldType) -> bool {
    match field {
        FieldType::String => true,
        FieldType::Float(_) => true,
        FieldType::Int => true,
        FieldType::Enum => false,
        FieldType::VecString => false,
        FieldType::Debug => false,
        FieldType::IntTime => true,
        FieldType::Bool => true,
        FieldType::AltitudeFeet => true,
        FieldType::SpeedKnotsTAS => true,
        FieldType::DistanceNM => true,
        FieldType::DurationMin => true,
    }
}

#[derive(PartialEq, Props)]
pub struct EditProps {
    pub headers: Vec<HeaderField>,
    pub item: Selectable,
    pub title: String,
}

pub fn edit_form<T>(cx: Scope<EditProps>) -> Element
where
    T: Struct + ToSelectable + std::fmt::Debug + TableHeader,
{
    let atom_instance = use_atom_ref(cx, INSTANCE);
    let atom_selectable = use_atom_ref(cx, SELECTED);

    let item = T::from_selectable(&cx.props.item).unwrap();
    let orig_name = item.get_name();

    let on_submit = move |ev: FormEvent| {
        let mut selectable = atom_selectable.write();
        let mut instance_refmut = atom_instance.write();
        let w_instance = instance_refmut.as_mut().unwrap();
        let item_to_change = T::get_mut_by_name(w_instance, &orig_name);

        for (k, v) in ev.values.iter() {
            // find header that matches key:
            let h = cx.props.headers.iter().find(|h| h.display == *k).unwrap();
            if let Err(e) = h.set_value_fromstr(item_to_change, v) {
                warn!("Failed to set field: {} with {}. Error: {}", h.field, v, e);
            }
        }
        // update selectable:
        *selectable = item_to_change.to_selectable();
    };

    cx.render(rsx!{
        div { class: "p-2 m-2 rounded bg-sky-200",
            h4 { class: "font-semibold flex",
                div { class: "flex-grow", "{cx.props.title}" }
                div {
                    class: "flex items-center font-thin rounded px-1 hover:bg-sky-300 hover:text-black icon",
                    onclick: move |_| {
                        let mut selected = use_atom_ref(cx, SELECTED).write();
                        *selected = Selectable::None;
                    },
                    ""
                }
            }
            form {
                autocomplete: "off",
                onsubmit: on_submit,
                oninput: move |ev| println!("Input {:?}", ev.values),
                for h in T::get_header().iter().filter(|h| fieldtype_editable(&h.type_)) {
                    div { class: "flex w-full mt-1 mb-1",
                        label { class: "flex-grow p-1", r#for: "{h.display}", "{h.display}" }
                        input {
                            class: "rounded p-1",
                            autocomplete: "off",
                            r#type: "{fieldtype_to_input(&h.type_)}",
                            name: "{h.display}",
                            value: "{h.get_value_string(&item)}",
                            readonly: "{!h.editable}",
                            disabled: "{!h.editable}",
                            step: "any",
                            checked: "{h.get_value_string(&item) == \"true\"}"
                        }
                    }
                }
                div { class: "flex",
                    div { class: "flex-grow" }
                    button {
                        class: "rounded p-2 mt-1 mb-1 bg-sky-300 border-1",
                        r#type: "submit",
                        value: "Submit",
                        "Submit changes"
                    }
                }
            }
        }
    })
}
