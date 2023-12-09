use std::{cell::RefMut, fmt::Debug};

use bevy_reflect::Struct;

use dce_lib::{
    editable::{Editable, FieldType, HeaderField, ValidationResult},
    DCEInstance,
};
use dioxus::prelude::*;
use fermi::{use_atom_ref, use_atom_state, AtomState, UseAtomRef};
use log::{trace, warn};

use crate::{
    helpers::edit_items::apply_to_item,
    selectable::{Selectable, ToSelectable},
    INSTANCE, INSTANCE_DIRTY, SELECTED,
};

fn fieldtype_to_input(field: &FieldType) -> String {
    match field {
        FieldType::String => "text".into(),
        FieldType::OptionString => "text".into(),
        FieldType::Float(_) => "number".into(),
        FieldType::Int => "number".into(),
        FieldType::Enum => "text".into(),
        FieldType::Debug => "text".into(),
        FieldType::IntTime => "time".into(),
        FieldType::Bool => "checkbox".into(),
        FieldType::AltitudeFeet => "number".into(),
        FieldType::SpeedKnotsTAS => "number".into(),
        FieldType::DistanceNM => "number".into(),
        FieldType::DurationMin => "number".into(),
        FieldType::TriggerActions => "text".into(),
        FieldType::FixedEnum(_) => "select".into(),
        FieldType::VecString => "text".into(),
        FieldType::VecStringOptions(_) => "select".into(),
        FieldType::DateStr => "date".into(),
        FieldType::NestedEditable(_) => "text".into(),
    }
}

fn fieldtype_editable(field: &FieldType) -> bool {
    match field {
        FieldType::String => true,
        FieldType::OptionString => true,
        FieldType::Float(_) => true,
        FieldType::Int => true,
        FieldType::Enum => false,
        FieldType::Debug => false,
        FieldType::IntTime => true,
        FieldType::Bool => true,
        FieldType::AltitudeFeet => true,
        FieldType::SpeedKnotsTAS => true,
        FieldType::DistanceNM => true,
        FieldType::DurationMin => true,
        FieldType::TriggerActions => true,
        FieldType::FixedEnum(_) => true,
        FieldType::VecString => true,
        FieldType::VecStringOptions(_) => true,
        FieldType::DateStr => true,
        FieldType::NestedEditable(_) => true,
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
    T: Struct + ToSelectable + std::fmt::Debug + Clone + Editable,
{
    trace!("render edit form");
    let atom_instance = use_atom_ref(cx, INSTANCE);
    let atom_dirty = use_atom_state(cx, INSTANCE_DIRTY);
    let atom_selected = use_atom_ref(cx, SELECTED);

    let selectable_from_props = T::from_selectable(&cx.props.item);

    if selectable_from_props.is_none() {
        return cx.render(rsx! {
            "Nothing to edit"
        });
    }

    let item_from_props = selectable_from_props.unwrap();

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

        trace!("edit_form submit: {:?}", ev);

        // apply the values from the form (to local state only)
        apply_to_item(&mut current_item, &ev.values);

        // validate and if passes apply to wider instance
        validate_and_apply(
            current_item,
            atom_instance,
            atom_selectable,
            atom_dirty,
            orig_name,
            validation_state,
        );
    };
    let headers = T::get_header();
    let usable_headers = headers
        .iter()
        .filter(|h| fieldtype_editable(&h.type_))
        .collect::<Vec<_>>();

    let entity_actions = T::actions_one_entity();

    let related_items = item_state
        .get()
        .related(&atom_instance.read().as_ref().expect("An instance exists"));

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
            form { autocomplete: "off", oninput: on_submit,
                for h in usable_headers {
                    match &h.type_ {
                        // Trigger actions have to render as one input per action
                        FieldType::TriggerActions | FieldType::VecString => rsx!{
                            render_triggeractions {
                                header: h.clone(),
                                item: item_state.get().to_owned(),
                                onclick_delete: move |(h_local, i)|{
                                    let mut mut_item = item_state.make_mut();
                                    delete_action_from_item(&mut mut_item, h_local, i);
                                    validate_and_apply(
                                        mut_item,
                                        atom_instance,
                                        use_atom_ref(cx, SELECTED),
                                        atom_dirty,
                                        orig_name,
                                        validation_state,
                                    );
                             },
                                onclick_addnew: move |h_local| {
                                    let mut mut_item = item_state.make_mut();

                                    add_action_to_item(&mut mut_item, h_local);
                                    validate_and_apply(
                                        mut_item,
                                        atom_instance,
                                        use_atom_ref(cx, SELECTED),
                                        atom_dirty,
                                        orig_name,
                                        validation_state,
                                    );
                                }
                            }
                        },
                        FieldType::VecStringOptions(options_func) => rsx!{
                            render_imagepickers {
                                header: h.clone(),
                                options_func: *options_func,
                                item: item_state.get().to_owned(),
                                onclick_delete: move |(h_local, i)|{
                                    let mut mut_item = item_state.make_mut();
                                    delete_action_from_item(&mut mut_item, h_local, i);
                                    validate_and_apply(
                                        mut_item,
                                        atom_instance,
                                        use_atom_ref(cx, SELECTED),
                                        atom_dirty,
                                        orig_name,
                                        validation_state,
                                    );
                             },
                                onclick_addnew: move |h_local| {
                                    let mut mut_item = item_state.make_mut();

                                    add_action_to_item(&mut mut_item, h_local);
                                    validate_and_apply(
                                        mut_item,
                                        atom_instance,
                                        use_atom_ref(cx, SELECTED),
                                        atom_dirty,
                                        orig_name,
                                        validation_state,
                                    );
                                }
                            }
                        },
                        FieldType::FixedEnum(allowed_values) => {
                            rsx! {
                                div { class: "flex w-full mt-1 mb-1",
                                label { class: "flex-grow p-1", r#for: "{h.display}", "{h.display}" }
                                select {
                                    class: "rounded p-1",
                                    autocomplete: "off",
                                    name: "{h.display}",
                                    value: "{h.get_value_string(item_state.get())}",
                                    disabled: "{!h.editable}",
                                    for v in allowed_values {
                                        rsx!{
                                            option {
                                                value: "{v}",
                                                "{v}"
                                            }
                                        }
                                    }
                                }
                            }
                            }
                        }
                        FieldType::NestedEditable(_) => rsx!{render_nested{
                            header: h.clone()
                            item: item_state.get().to_owned()
                        }},
                        // all other fields are one input per field
                        _ => rsx!{
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
                    }
                }
                render_errors { result: validation_state.get().to_owned() }
            }
            if !entity_actions.is_empty() {
                rsx!{
                    for a in entity_actions {
                        rsx!{
                            button {
                                class: "p-1 bg-slate-100 hover:bg-slate-300 rounded border-slate-500 border-2 ml-2 tooltip",
                                onclick: move |_| {
                                    let mut atom_instance = atom_instance.write();
                                    let mut_instance = atom_instance.as_mut().expect("DCE instance is loaded");
                                    let mut mut_item = item_state.make_mut();
                                    match (a.function)(&mut mut_item, mut_instance) {
                                        Ok(()) => {},
                                        Err(_) => {}
                                    };
                                },
                                span {
                                    class: "tooltiptext",
                                    "{a.description}"
                                }
                                "{a.name}"
                            }
                        }
                    }
                }
            }
            if !related_items.is_empty() {
                rsx! {
                    h4 {class: "font-semibold", "Related Items"},
                    for r in related_items {
                        rsx! {
                            span {
                                class: "hover:bg-slate-200",
                                onclick: move |_| {
                                // make a selectable from name and type
                                let atom_instance = atom_instance.read();
                                let instance = atom_instance.as_ref().expect("DCE instance is loaded");
                                let selectable = Selectable::from_type_name(&r.type_name(), &r.get_name(), &instance);
                                let mut selected = atom_selected.write();
                                *selected = selectable;
                            },"{r.get_name()}"}
                        }
                    }
                }
            }
        }
    })
}

#[derive(PartialEq, Props)]
struct RenderErrorProps {
    result: ValidationResult,
}

fn render_errors(cx: Scope<RenderErrorProps>) -> Option<VNode> {
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

#[derive(Props)]
struct TriggerActionProps<'a, T> {
    header: HeaderField,
    item: T,
    /// Delete button: (Headerfield, index)
    onclick_delete: EventHandler<'a, (&'a HeaderField, usize)>,
    /// Add new row button: ()
    onclick_addnew: EventHandler<'a, &'a HeaderField>,
}

fn render_triggeractions<'a, T>(cx: Scope<'a, TriggerActionProps<'a, T>>) -> Element<'a>
where
    T: Struct,
{
    let h = &cx.props.header;
    cx.render(rsx!{
        label { class: "p-1 w-full", "{h.display}" }
        for (i , action) in h.get_value_stringvec(&cx.props.item).iter().enumerate() {
            rsx! {
                div {
                    class: "flex w-full mt-1 mb-1",
                    label { class: "p-1", r#for: "{h.display}.{i}", "{i}" }
                    input {
                        class: "flex-grow rounded p-1",
                        autocomplete: "off",
                        r#type: "{fieldtype_to_input(&h.type_)}",
                        name: "{h.display}.{i}",
                        value: "{action}"
                    }
                    div {
                        class: "flex items-center font-thin rounded px-1 hover:bg-sky-300 hover:text-black icon",
                        onclick: move |_| cx.props.onclick_delete.call((h, i)),
                        "\u{E74D}"
                    }
                }
            }
        }
        div { class: "flex",
            div { class: "flex-grow" }
            div {
                class: "flex items-center font-thin rounded px-1 hover:bg-sky-300 hover:text-black icon",
                onclick: move |_| cx.props.onclick_addnew.call(h),
                ""
            }
        }
    })
}

#[derive(Props)]
struct ImagePickerProps<'a, T> {
    header: HeaderField,
    item: T,
    /// Delete button: (Headerfield, index)
    onclick_delete: EventHandler<'a, (&'a HeaderField, usize)>,
    /// Add new row button: ()
    onclick_addnew: EventHandler<'a, &'a HeaderField>,
    options_func: fn(&DCEInstance) -> Vec<String>,
}
fn render_imagepickers<'a, T>(cx: Scope<'a, ImagePickerProps<'a, T>>) -> Element<'a>
where
    T: Struct,
{
    let atom_instance = use_atom_ref(cx, INSTANCE).read();
    let instance = atom_instance.as_ref().expect("an instance");

    let h = &cx.props.header;

    cx.render(rsx!{
        label { class: "p-1 w-full", "{h.display}" }
        for (i , action) in h.get_value_stringvec(&cx.props.item).iter().enumerate() {
            rsx! {
                div {
                    class: "flex w-full mt-1 mb-1",
                    label { class: "p-1", r#for: "{h.display}.{i}", "{i}" }
                    select {
                        class: "rounded p-1 flex-grow",
                        autocomplete: "off",
                        name: "{h.display}.{i}",
                        value: "{action}",
                        disabled: "{!h.editable}",
                        option {
                            value: "",
                            ""
                        }
                        for v in (cx.props.options_func)(instance).clone() {
                            rsx!{
                                option {
                                    value: "{v}",
                                    "{v}"
                                }
                            }
                        }
                    }
                    div {
                        class: "flex items-center font-thin rounded px-1 hover:bg-sky-300 hover:text-black icon",
                        onclick: move |_| cx.props.onclick_delete.call((h, i)),
                        "\u{E74D}"
                    }
                }
            }
        }
        div { class: "flex",
            div { class: "flex-grow" }
            div {
                class: "flex items-center font-thin rounded px-1 hover:bg-sky-300 hover:text-black icon",
                onclick: move |_| cx.props.onclick_addnew.call(h),
                ""
            }
        }
    })
}

#[derive(Props)]
struct NestedProps<T> {
    header: HeaderField,
    item: T,
}
fn render_nested<T>(cx: Scope<NestedProps<T>>) -> Element
where
    T: Struct,
{
    let parent_header = &cx.props.header;
    let FieldType::NestedEditable(sub_headers) = &parent_header.type_ else {
        panic!("Only use this with NestedEditable")
    };
    let values = parent_header.get_value_stringvec(&cx.props.item);

    cx.render(rsx! {
        div {
            class: "bg-sky-300 rounded p-2",
            h4{ class: "flex-grow font-medium p-1", "{parent_header.display}"}
            for (i,(sub_header, value)) in sub_headers.iter().zip(values).enumerate() {
                rsx!(
                    div { class: "flex w-full mt-1 mb-1",
                label { class: "flex-grow p-1", r#for: "{sub_header.display}", "{sub_header.display}" }
                input {
                    class: "rounded p-1",
                    autocomplete: "off",
                    r#type: "{fieldtype_to_input(&sub_header.type_)}",
                    name: "{parent_header.display}.{i}",
                    value: "{value}",
                    readonly: "{!sub_header.editable}",
                    disabled: "{!sub_header.editable}",
                    step: "any",
                    checked: "{value == \"true\"}"
                }
            }
                )
            }
        }
    })
}

fn add_action_to_item<T>(item: &mut RefMut<T>, header: &HeaderField)
where
    T: Struct,
{
    let mut actions = header.get_value_stringvec(&**item);

    actions.push("".into());

    header
        .set_value_from_stringvec(&mut **item, actions)
        .unwrap_or_else(|e| panic!("Failed to add action with error: {e:?}"));
}

fn delete_action_from_item<T>(item: &mut RefMut<T>, header: &HeaderField, index: usize)
where
    T: Struct + Debug,
{
    let mut actions = header.get_value_stringvec(&**item);
    if actions.len() == 1 {
        actions[0] = "".into();
    } else {
        actions.remove(index);
    }

    header
        .set_value_from_stringvec(&mut **item, actions)
        .unwrap_or_else(|e| panic!("Failed to add action with error: {e:?}"));
}

fn validate_and_apply<T>(
    current_item: RefMut<T>,
    atom_instance: &UseAtomRef<Option<DCEInstance>>,
    atom_selectable: &UseAtomRef<Selectable>,
    atom_dirty: &AtomState<bool>,
    orig_name: &UseState<String>,
    validation_state: &UseState<ValidationResult>,
) where
    T: Editable + ToSelectable + Clone,
{
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
            let item_to_change = T::get_mut_by_name(w_instance, orig_name);
            *item_to_change = current_item.clone();

            let mut selectable = atom_selectable.write();
            *selectable = item_to_change.to_selectable();

            orig_name.modify(|_| current_item.get_name());
            atom_dirty.set(true);
            validation_state.modify(|_| ValidationResult::Pass);
        }
        ValidationResult::Fail(errors) => {
            warn!("Got Errors: {:?}", errors);
            validation_state.modify(|_| ValidationResult::Fail(errors));
        }
    }
}
