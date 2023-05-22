use bevy_reflect::Struct;

use dioxus::prelude::*;
use fermi::use_atom_ref;
use log::{trace, warn};
use tables::{FieldType, HeaderField, TableHeader};

use crate::{
    selectable::{Selectable, ToSelectable, ValidationResult},
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
    T: Struct + ToSelectable + std::fmt::Debug + TableHeader + Clone,
{
    trace!("render edit form");
    let atom_instance = use_atom_ref(cx, INSTANCE);

    let item_from_props = T::from_selectable(&cx.props.item).unwrap();

    let validation_state = use_state(cx, || ValidationResult::Pass);
    let item_state = use_state(cx, || item_from_props.to_owned());
    let orig_name = use_state(cx, || item_state.get().get_name());

    if item_from_props.get_name().as_str() != orig_name.as_str() {
        trace!("Selectable has changed, resetting form");
        validation_state.modify(|_| ValidationResult::Pass);
        orig_name.modify(|_| item_from_props.get_name());
        item_state.modify(|_| item_from_props);
    }

    let on_submit = move |ev: FormEvent| {
        let atom_selectable = use_atom_ref(cx, SELECTED);
        let mut current_item = item_state.make_mut();

        // apply the values from the form
        for (k, v) in ev.values.iter() {
            // find header that matches key:
            let h = cx.props.headers.iter().find(|h| h.display == *k).unwrap();
            if let Err(e) = h.set_value_fromstr(&mut *current_item, v) {
                warn!("Failed to set field: {} with {}. Error: {}", h.field, v, e);
            }
        }

        // validate
        let validation_result = {
            let instance_ref = atom_instance.read();
            let r_instance = instance_ref.as_ref().unwrap();
            current_item.validate(r_instance)
        };

        match validation_result {
            ValidationResult::Pass => {
                // get mutable references and pass into the various atoms
                let mut instance_refmut = atom_instance.write();
                let w_instance = instance_refmut.as_mut().unwrap();
                let item_to_change = T::get_mut_by_name(w_instance, &orig_name);
                *item_to_change = current_item.clone();

                let mut selectable = atom_selectable.write();
                *selectable = item_to_change.to_selectable();

                orig_name.modify(|_| current_item.get_name());

                validation_state.modify(|_| ValidationResult::Pass);
            }
            ValidationResult::Fail(errors) => {
                warn!("Got Errors: {:?}", errors);
                validation_state.modify(|_| ValidationResult::Fail(errors));
            }
        }
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
                oninput: on_submit,
                for h in T::get_header().iter().filter(|h| fieldtype_editable(&h.type_)) {
                    div { class: "flex w-full mt-1 mb-1",
                        label { class: "flex-grow p-1", r#for: "{h.display}", "{h.display}" }
                        input {
                            class: "rounded p-1",
                            autocomplete: "off",
                            r#type: "{fieldtype_to_input(&h.type_)}",
                            name: "{h.display}",
                            value: "{h.get_value_string(item_state.get())}",
                            readonly: "{!h.editable}",
                            disabled: "{!h.editable}",
                            step: "any",
                            checked: "{h.get_value_string(item_state.get()) == \"true\"}"
                        }
                    }
                }
                render_errors { result: validation_state.get().to_owned()}
            }
        }
    })
}

#[derive(PartialEq, Props)]
struct RenderErrorProps {
    result: ValidationResult,
}

fn render_errors<'a>(cx: Scope<RenderErrorProps>) -> Option<VNode> {
    if let ValidationResult::Fail(errors) = &cx.props.result {
        return cx.render(rsx! {
            for e in errors.iter() {
                rsx!{
                    div {
                        class: "italic text-xs p-1 text-red-700",
                        "{e.error}"
                    }
                }
            }
        });
    }
    None
}
