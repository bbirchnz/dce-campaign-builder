/// Collection of simple methods for getting first item out of the instance, and settings
/// it as the SELECTED. Has the effect of bringing up the edit form and table for that type
use dioxus::prelude::*;
use fermi::use_atom_ref;

use crate::{selectable::Selectable, INSTANCE, SELECTED};

pub fn select_first_fixed_airbase(cx: Scope) {
    let atom_instance = use_atom_ref(cx, INSTANCE);
    let atom_selected = use_atom_ref(cx, SELECTED);

    if let Some(fixed) = atom_instance
        .read()
        .as_ref()
        .unwrap()
        .airbases
        .fixed
        .first()
    {
        let mut writable = atom_selected.write();
        *writable = Selectable::FixedAirBase(fixed.clone());
    }
}
pub fn select_first_ship_airbase(cx: Scope) {
    let atom_instance = use_atom_ref(cx, INSTANCE);
    let atom_selected = use_atom_ref(cx, SELECTED);

    if let Some(item) = atom_instance.read().as_ref().unwrap().airbases.ship.first() {
        let mut writable = atom_selected.write();
        *writable = Selectable::ShipAirBase(item.clone());
    }
}
pub fn select_first_airstart_airbase(cx: Scope) {
    let atom_instance = use_atom_ref(cx, INSTANCE);
    let atom_selected = use_atom_ref(cx, SELECTED);

    if let Some(item) = atom_instance
        .read()
        .as_ref()
        .unwrap()
        .airbases
        .air_start
        .first()
    {
        let mut writable = atom_selected.write();
        *writable = Selectable::AirstartBase(item.clone());
    }
}

pub fn select_first_strike_target(cx: Scope) {
    let atom_instance = use_atom_ref(cx, INSTANCE);
    let atom_selected = use_atom_ref(cx, SELECTED);

    if let Some(item) = atom_instance
        .read()
        .as_ref()
        .unwrap()
        .target_list
        .strike
        .first()
    {
        let mut writable = atom_selected.write();
        *writable = Selectable::TargetStrike(item.clone());
    }
}

pub fn select_first_cap_target(cx: Scope) {
    let atom_instance = use_atom_ref(cx, INSTANCE);
    let atom_selected = use_atom_ref(cx, SELECTED);

    if let Some(item) = atom_instance
        .read()
        .as_ref()
        .unwrap()
        .target_list
        .cap
        .first()
    {
        let mut writable = atom_selected.write();
        *writable = Selectable::TargetCAP(item.clone());
    }
}

pub fn select_first_ship_target(cx: Scope) {
    let atom_instance = use_atom_ref(cx, INSTANCE);
    let atom_selected = use_atom_ref(cx, SELECTED);

    if let Some(item) = atom_instance
        .read()
        .as_ref()
        .unwrap()
        .target_list
        .antiship
        .first()
    {
        let mut writable = atom_selected.write();
        *writable = Selectable::TargetAntiShip(item.clone());
    }
}

pub fn select_first_aar_target(cx: Scope) {
    let atom_instance = use_atom_ref(cx, INSTANCE);
    let atom_selected = use_atom_ref(cx, SELECTED);

    if let Some(item) = atom_instance
        .read()
        .as_ref()
        .unwrap()
        .target_list
        .refuel
        .first()
    {
        let mut writable = atom_selected.write();
        *writable = Selectable::TargetAAR(item.clone());
    }
}

pub fn select_first_awacs_target(cx: Scope) {
    let atom_instance = use_atom_ref(cx, INSTANCE);
    let atom_selected = use_atom_ref(cx, SELECTED);

    if let Some(item) = atom_instance
        .read()
        .as_ref()
        .unwrap()
        .target_list
        .awacs
        .first()
    {
        let mut writable = atom_selected.write();
        *writable = Selectable::TargetAWACS(item.clone());
    }
}

pub fn select_first_squadron(cx: Scope) {
    let atom_instance = use_atom_ref(cx, INSTANCE);
    let atom_selected = use_atom_ref(cx, SELECTED);

    if let Some(item) = atom_instance.read().as_ref().unwrap().oob_air.blue.first() {
        let mut writable = atom_selected.write();
        *writable = Selectable::Squadron(item.clone());
    }
}

pub fn select_campaign_settings(cx: Scope) {
    let atom_instance = use_atom_ref(cx, INSTANCE);
    let atom_selected = use_atom_ref(cx, SELECTED);

    let item = atom_instance
        .read()
        .as_ref()
        .unwrap()
        .campaign_header
        .clone();

    let mut writable = atom_selected.write();
    *writable = Selectable::CampaignSettings(item);
}

pub fn select_first_cap_loadout(cx: Scope) {
    let atom_instance = use_atom_ref(cx, INSTANCE);
    let atom_selected = use_atom_ref(cx, SELECTED);

    if let Some(item) = atom_instance.read().as_ref().unwrap().loadouts.cap.first() {
        let mut writable = atom_selected.write();
        *writable = Selectable::LoadoutCAP(item.clone());
    }
}

pub fn select_first_strike_loadout(cx: Scope) {
    let atom_instance = use_atom_ref(cx, INSTANCE);
    let atom_selected = use_atom_ref(cx, SELECTED);

    if let Some(item) = atom_instance
        .read()
        .as_ref()
        .unwrap()
        .loadouts
        .strike
        .first()
    {
        let mut writable = atom_selected.write();
        *writable = Selectable::LoadoutStrike(item.clone());
    }
}

pub fn select_first_awacs_loadout(cx: Scope) {
    let atom_instance = use_atom_ref(cx, INSTANCE);
    let atom_selected = use_atom_ref(cx, SELECTED);

    if let Some(item) = atom_instance
        .read()
        .as_ref()
        .unwrap()
        .loadouts
        .awacs
        .first()
    {
        let mut writable = atom_selected.write();
        *writable = Selectable::LoadoutAWACS(item.clone());
    }
}

pub fn select_first_aar_loadout(cx: Scope) {
    let atom_instance = use_atom_ref(cx, INSTANCE);
    let atom_selected = use_atom_ref(cx, SELECTED);

    if let Some(item) = atom_instance.read().as_ref().unwrap().loadouts.aar.first() {
        let mut writable = atom_selected.write();
        *writable = Selectable::LoadoutAAR(item.clone());
    }
}

pub fn select_first_antiship_loadout(cx: Scope) {
    let atom_instance = use_atom_ref(cx, INSTANCE);
    let atom_selected = use_atom_ref(cx, SELECTED);

    if let Some(item) = atom_instance
        .read()
        .as_ref()
        .unwrap()
        .loadouts
        .antiship
        .first()
    {
        let mut writable = atom_selected.write();
        *writable = Selectable::LoadoutAntiship(item.clone());
    }
}

pub fn select_first_trigger(cx: Scope) {
    let atom_instance = use_atom_ref(cx, INSTANCE);
    let atom_selected = use_atom_ref(cx, SELECTED);

    if let Some(item) = atom_instance.read().as_ref().unwrap().triggers.first() {
        let mut writable = atom_selected.write();
        *writable = Selectable::Trigger(item.clone());
    }
}
