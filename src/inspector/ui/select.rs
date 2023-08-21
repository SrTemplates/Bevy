use std::any::TypeId;

use bevy::asset::{HandleId, ReflectAsset};
use bevy::prelude::*;
use bevy_reflect::TypeRegistry;
use bevy_egui::egui;

#[derive(Eq, PartialEq)]
pub enum InspectorSelection {
    Entities,
    Resource(TypeId, String),
    Asset(TypeId, String, HandleId),
}

pub fn select_resource(
    ui: &mut egui::Ui,
    type_registry: &TypeRegistry,
    selection: &mut InspectorSelection,
) {
    let mut resources: Vec<_> = type_registry
        .iter()
        .filter(|registration| registration.data::<ReflectResource>().is_some())
        .map(|registration| (registration.short_name().to_owned(), registration.type_id()))
        .collect();
    resources.sort_by(|(name_a, _), (name_b, _)| name_a.cmp(name_b));

    for (resource_name, type_id) in resources {
        let selected = match *selection {
            InspectorSelection::Resource(selected, _) => selected == type_id,
            _ => false,
        };

        if ui.selectable_label(selected, &resource_name).clicked() {
            *selection = InspectorSelection::Resource(type_id, resource_name);
        }
    }
}

pub fn select_asset(
    ui: &mut egui::Ui,
    type_registry: &TypeRegistry,
    world: &World,
    selection: &mut InspectorSelection,
) {
    let mut assets: Vec<_> = type_registry
        .iter()
        .filter_map(|registration| {
            let reflect_asset = registration.data::<ReflectAsset>()?;
            Some((
                registration.short_name().to_owned(),
                registration.type_id(),
                reflect_asset,
            ))
        })
        .collect();
    assets.sort_by(|(name_a, ..), (name_b, ..)| name_a.cmp(name_b));

    for (asset_name, asset_type_id, reflect_asset) in assets {
        let mut handles: Vec<_> = reflect_asset.ids(world).collect();
        handles.sort();

        ui.collapsing(format!("{asset_name} ({})", handles.len()), |ui| {
            for handle in handles {
                let selected = match *selection {
                    InspectorSelection::Asset(_, _, selected_id) => selected_id == handle,
                    _ => false,
                };

                if ui
                    .selectable_label(selected, format!("{:?}", handle))
                    .clicked()
                {
                    *selection =
                        InspectorSelection::Asset(asset_type_id, asset_name.clone(), handle);
                }
            }
        });
    }
}
