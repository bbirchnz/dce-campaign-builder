/// Collection of simple methods for getting first item out of the instance, and settings
/// it as the SELECTED. Has the effect of bringing up the edit form and table for that type
use dioxus::prelude::*;
use fermi::use_atom_ref;

use crate::{selectable::Selectable, INSTANCE, SELECTED};

pub fn select_first_fixed_airbase(cx: Scope) {
    let atom_instance = use_atom_ref(cx, INSTANCE);
    let atom_selected = use_atom_ref(cx, SELECTED);
    let mut writable = atom_selected.write();

    if let Some(fixed) = atom_instance
        .read()
        .as_ref()
        .unwrap()
        .airbases
        .fixed
        .first()
    {
        *writable = Selectable::FixedAirBase(Some(fixed.clone()));
    } else {
        *writable = Selectable::FixedAirBase(None);
    }
}
pub fn select_first_ship_airbase(cx: Scope) {
    let atom_instance = use_atom_ref(cx, INSTANCE);
    let atom_selected = use_atom_ref(cx, SELECTED);
    let mut writable = atom_selected.write();

    if let Some(item) = atom_instance.read().as_ref().unwrap().airbases.ship.first() {
        *writable = Selectable::ShipAirBase(Some(item.clone()));
    } else {
        *writable = Selectable::ShipAirBase(None);
    }
}
pub fn select_first_airstart_airbase(cx: Scope) {
    let atom_instance = use_atom_ref(cx, INSTANCE);
    let atom_selected = use_atom_ref(cx, SELECTED);
    let mut writable = atom_selected.write();

    if let Some(item) = atom_instance
        .read()
        .as_ref()
        .unwrap()
        .airbases
        .air_start
        .first()
    {
        *writable = Selectable::AirstartBase(Some(item.clone()));
    } else {
        *writable = Selectable::AirstartBase(None);
    }
}

pub fn select_first_farp_airbase(cx: Scope) {
    let atom_instance = use_atom_ref(cx, INSTANCE);
    let atom_selected = use_atom_ref(cx, SELECTED);
    let mut writable = atom_selected.write();

    if let Some(item) = atom_instance.read().as_ref().unwrap().airbases.farp.first() {
        *writable = Selectable::FARPBase(Some(item.clone()));
    } else {
        *writable = Selectable::FARPBase(None);
    }
}

pub fn select_first_strike_target(cx: Scope) {
    let atom_instance = use_atom_ref(cx, INSTANCE);
    let atom_selected = use_atom_ref(cx, SELECTED);
    let mut writable = atom_selected.write();

    if let Some(item) = atom_instance
        .read()
        .as_ref()
        .unwrap()
        .target_list
        .strike
        .first()
    {
        *writable = Selectable::TargetStrike(Some(item.clone()));
    } else {
        *writable = Selectable::TargetStrike(None);
    }
}

pub fn select_first_cap_target(cx: Scope) {
    let atom_instance = use_atom_ref(cx, INSTANCE);
    let atom_selected = use_atom_ref(cx, SELECTED);
    let mut writable = atom_selected.write();

    if let Some(item) = atom_instance
        .read()
        .as_ref()
        .unwrap()
        .target_list
        .cap
        .first()
    {
        *writable = Selectable::TargetCAP(Some(item.clone()));
    } else {
        *writable = Selectable::TargetCAP(None);
    }
}

pub fn select_first_intercept_target(cx: Scope) {
    let atom_instance = use_atom_ref(cx, INSTANCE);
    let atom_selected = use_atom_ref(cx, SELECTED);
    let mut writable = atom_selected.write();

    if let Some(item) = atom_instance
        .read()
        .as_ref()
        .unwrap()
        .target_list
        .intercept
        .first()
    {
        *writable = Selectable::TargetIntercept(Some(item.clone()));
    } else {
        *writable = Selectable::TargetIntercept(None);
    }
}

pub fn select_first_ship_target(cx: Scope) {
    let atom_instance = use_atom_ref(cx, INSTANCE);
    let atom_selected = use_atom_ref(cx, SELECTED);
    let mut writable = atom_selected.write();

    if let Some(item) = atom_instance
        .read()
        .as_ref()
        .unwrap()
        .target_list
        .antiship
        .first()
    {
        *writable = Selectable::TargetAntiShip(Some(item.clone()));
    } else {
        *writable = Selectable::TargetAntiShip(None);
    }
}

pub fn select_first_aar_target(cx: Scope) {
    let atom_instance = use_atom_ref(cx, INSTANCE);
    let atom_selected = use_atom_ref(cx, SELECTED);
    let mut writable = atom_selected.write();

    if let Some(item) = atom_instance
        .read()
        .as_ref()
        .unwrap()
        .target_list
        .refuel
        .first()
    {
        *writable = Selectable::TargetAAR(Some(item.clone()));
    } else {
        *writable = Selectable::TargetAAR(None);
    }
}

pub fn select_first_awacs_target(cx: Scope) {
    let atom_instance = use_atom_ref(cx, INSTANCE);
    let atom_selected = use_atom_ref(cx, SELECTED);
    let mut writable = atom_selected.write();

    if let Some(item) = atom_instance
        .read()
        .as_ref()
        .unwrap()
        .target_list
        .awacs
        .first()
    {
        *writable = Selectable::TargetAWACS(Some(item.clone()));
    } else {
        *writable = Selectable::TargetAWACS(None);
    }
}

pub fn select_first_squadron(cx: Scope) {
    let atom_instance = use_atom_ref(cx, INSTANCE);
    let atom_selected = use_atom_ref(cx, SELECTED);
    let mut writable = atom_selected.write();

    let instance_read = atom_instance.read();
    let oob_air = &instance_read.as_ref().unwrap().oob_air;

    if let Some(item) = oob_air.blue.iter().chain(oob_air.red.iter()).find(|_| true) {
        *writable = Selectable::Squadron(Some(item.clone()));
    } else {
        *writable = Selectable::Squadron(None);
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
    let mut writable = atom_selected.write();

    if let Some(item) = atom_instance.read().as_ref().unwrap().loadouts.cap.first() {
        *writable = Selectable::LoadoutCAP(Some(item.clone()));
    } else {
        *writable = Selectable::LoadoutCAP(None);
    }
}

pub fn select_first_strike_loadout(cx: Scope) {
    let atom_instance = use_atom_ref(cx, INSTANCE);
    let atom_selected = use_atom_ref(cx, SELECTED);
    let mut writable = atom_selected.write();

    if let Some(item) = atom_instance
        .read()
        .as_ref()
        .unwrap()
        .loadouts
        .strike
        .first()
    {
        *writable = Selectable::LoadoutStrike(Some(item.clone()));
    } else {
        *writable = Selectable::LoadoutStrike(None);
    }
}

pub fn select_first_awacs_loadout(cx: Scope) {
    let atom_instance = use_atom_ref(cx, INSTANCE);
    let atom_selected = use_atom_ref(cx, SELECTED);
    let mut writable = atom_selected.write();

    if let Some(item) = atom_instance
        .read()
        .as_ref()
        .unwrap()
        .loadouts
        .awacs
        .first()
    {
        *writable = Selectable::LoadoutAWACS(Some(item.clone()));
    } else {
        *writable = Selectable::LoadoutAWACS(None);
    }
}

pub fn select_first_aar_loadout(cx: Scope) {
    let atom_instance = use_atom_ref(cx, INSTANCE);
    let atom_selected = use_atom_ref(cx, SELECTED);
    let mut writable = atom_selected.write();

    if let Some(item) = atom_instance.read().as_ref().unwrap().loadouts.aar.first() {
        *writable = Selectable::LoadoutAAR(Some(item.clone()));
    } else {
        *writable = Selectable::LoadoutAAR(None);
    }
}

pub fn select_first_antiship_loadout(cx: Scope) {
    let atom_instance = use_atom_ref(cx, INSTANCE);
    let atom_selected = use_atom_ref(cx, SELECTED);
    let mut writable = atom_selected.write();

    if let Some(item) = atom_instance
        .read()
        .as_ref()
        .unwrap()
        .loadouts
        .antiship
        .first()
    {
        *writable = Selectable::LoadoutAntiship(Some(item.clone()));
    } else {
        *writable = Selectable::LoadoutAntiship(None);
    }
}

pub fn select_first_escort_loadout(cx: Scope) {
    let atom_instance = use_atom_ref(cx, INSTANCE);
    let atom_selected = use_atom_ref(cx, SELECTED);
    let mut writable = atom_selected.write();

    if let Some(item) = atom_instance
        .read()
        .as_ref()
        .unwrap()
        .loadouts
        .escort
        .first()
    {
        *writable = Selectable::LoadoutEscort(Some(item.clone()));
    } else {
        *writable = Selectable::LoadoutEscort(None);
    }
}

pub fn select_first_sead_loadout(cx: Scope) {
    let atom_instance = use_atom_ref(cx, INSTANCE);
    let atom_selected = use_atom_ref(cx, SELECTED);
    let mut writable = atom_selected.write();

    if let Some(item) = atom_instance.read().as_ref().unwrap().loadouts.sead.first() {
        *writable = Selectable::LoadoutSEAD(Some(item.clone()));
    } else {
        *writable = Selectable::LoadoutSEAD(None);
    }
}

pub fn select_first_transport_loadout(cx: Scope) {
    let atom_instance = use_atom_ref(cx, INSTANCE);
    let atom_selected = use_atom_ref(cx, SELECTED);
    let mut writable = atom_selected.write();

    if let Some(item) = atom_instance
        .read()
        .as_ref()
        .unwrap()
        .loadouts
        .transport
        .first()
    {
        *writable = Selectable::LoadoutTransport(Some(item.clone()));
    } else {
        *writable = Selectable::LoadoutTransport(None);
    }
}

pub fn select_first_intercept_loadout(cx: Scope) {
    let atom_instance = use_atom_ref(cx, INSTANCE);
    let atom_selected = use_atom_ref(cx, SELECTED);
    let mut writable = atom_selected.write();

    if let Some(item) = atom_instance
        .read()
        .as_ref()
        .unwrap()
        .loadouts
        .intercept
        .first()
    {
        *writable = Selectable::LoadoutIntercept(Some(item.clone()));
    } else {
        *writable = Selectable::LoadoutIntercept(None);
    }
}

pub fn select_first_trigger(cx: Scope) {
    let atom_instance = use_atom_ref(cx, INSTANCE);
    let atom_selected = use_atom_ref(cx, SELECTED);
    let mut writable = atom_selected.write();

    if let Some(item) = atom_instance.read().as_ref().unwrap().triggers.first() {
        *writable = Selectable::Trigger(Some(item.clone()));
    } else {
        *writable = Selectable::Trigger(None);
    }
}

pub fn select_first_image(cx: Scope) {
    let atom_instance = use_atom_ref(cx, INSTANCE);
    let atom_selected = use_atom_ref(cx, SELECTED);
    let mut writable = atom_selected.write();

    if let Some(item) = atom_instance
        .read()
        .as_ref()
        .unwrap()
        .bin_data
        .images
        .first()
    {
        *writable = Selectable::Image(Some(item.clone()));
    } else {
        *writable = Selectable::Image(None);
    }
}
