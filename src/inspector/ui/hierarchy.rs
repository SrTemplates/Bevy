use bevy::ecs::query::ReadOnlyWorldQuery;
use bevy::prelude::*;
use bevy::utils::HashSet;
use bevy_egui::egui::collapsing_header::paint_default_icon;
use bevy_egui::egui::{self, CollapsingHeader, RichText};
use bevy_inspector_egui::bevy_inspector::guess_entity_name;
use bevy_inspector_egui::bevy_inspector::hierarchy::{SelectedEntities, SelectionMode};
use bevy_reflect::TypeRegistry;

use crate::inspector::default_scene::InspectorEntity;

use super::{add_ui, AddWindowState};

/// Display UI of the entity hierarchy.
///
/// Returns `true` if a new entity was selected.
pub fn hierarchy_ui(
    world: &mut World,
    ui: &mut egui::Ui,
    selected: &mut SelectedEntities,
    state: &AddWindowState,
) -> bool {
    let type_registry = world.resource::<AppTypeRegistry>().clone();
    let type_registry = type_registry.read();

    Hierarchy {
        world,
        type_registry: &type_registry,
        selected,
        context_menu: None,
        shortcircuit_entity: None,
        extra_state: state,
    }
    .show::<()>(ui)
}

pub struct Hierarchy<'a> {
    pub world: &'a mut World,
    pub type_registry: &'a TypeRegistry,
    pub selected: &'a mut SelectedEntities,
    pub context_menu: Option<&'a dyn FnMut(&mut egui::Ui, Entity, &mut World, &AddWindowState)>,
    pub shortcircuit_entity:
        Option<&'a mut dyn FnMut(&mut egui::Ui, Entity, &mut World, &AddWindowState) -> bool>,
    pub extra_state: &'a AddWindowState<'a>,
}

impl Hierarchy<'_> {
    pub fn show<F: ReadOnlyWorldQuery>(&mut self, ui: &mut egui::Ui) -> bool {
        let mut root_query = self
            .world
            .query_filtered::<Entity, (Without<Parent>, Without<InspectorEntity>, F)>();

        let always_open: HashSet<Entity> = self
            .selected
            .iter()
            .flat_map(|selected| {
                std::iter::successors(Some(selected), |&entity| {
                    self.world.get::<Parent>(entity).map(|parent| parent.get())
                })
                .skip(1)
            })
            .collect();

        let mut entities: Vec<_> = root_query.iter(self.world).collect();
        entities.sort();

        let mut selected = false;
        ui.vertical(|ui| {
            for &entity in &entities {
                selected |= self.entity_ui(ui, entity, &always_open, &entities);
            }
        })
        .response
        .context_menu(|ui| {
            add_ui(ui, self.extra_state).map(|add_item| {
                selected |= true;
                let entity = self.world.spawn_empty().id();
                add_item.add_to_entity(self.world, entity);
                self.selected.select_replace(entity);
            });
        });
        selected
    }

    fn entity_ui(
        &mut self,
        ui: &mut egui::Ui,
        entity: Entity,
        always_open: &HashSet<Entity>,
        at_same_level: &[Entity],
    ) -> bool {
        let mut new_selection = false;
        let selected = self.selected.contains(entity);

        let entity_name = guess_entity_name(self.world, entity);
        let mut name = RichText::new(entity_name);
        if selected {
            name = name.strong();
        }

        let has_children = self
            .world
            .get::<Children>(entity)
            .map_or(false, |children| children.len() > 0);

        let open = if !has_children {
            Some(false)
        } else if always_open.contains(&entity) {
            Some(true)
        } else {
            None
        };

        if let Some(shortcircuit_entity) = self.shortcircuit_entity.as_mut() {
            if shortcircuit_entity(ui, entity, self.world, self.extra_state) {
                return false;
            }
        }

        #[allow(deprecated)] // the suggested replacement doesn't really work
        let response = CollapsingHeader::new(name)
            .id_source(entity)
            .icon(move |ui, openness, response| {
                if !has_children {
                    return;
                }
                paint_default_icon(ui, openness, response);
            })
            .selectable(true)
            .selected(selected)
            .open(open)
            .show(ui, |ui| {
                let children = self.world.get::<Children>(entity);
                if let Some(children) = children {
                    let children = children.to_vec();
                    for &child in children.iter() {
                        self.entity_ui(ui, child, always_open, &children);
                    }
                } else {
                    ui.label("No children");
                }
            });
        let header_response = response.header_response;

        if header_response.clicked() {
            let selection_mode = ui.input(|input| {
                SelectionMode::from_ctrl_shift(input.modifiers.ctrl, input.modifiers.shift)
            });
            let extend_with = |from, to| {
                // PERF: this could be done in one scan
                let from_position = at_same_level.iter().position(|&entity| entity == from);
                let to_position = at_same_level.iter().position(|&entity| entity == to);
                from_position
                    .zip(to_position)
                    .map(|(from, to)| {
                        let (min, max) = if from < to { (from, to) } else { (to, from) };
                        at_same_level[min..=max].iter().copied()
                    })
                    .into_iter()
                    .flatten()
            };
            self.selected.select(selection_mode, entity, extend_with);
            new_selection = true;
        }

        // if let Some(context_menu) = self.context_menu.as_mut() {
        header_response.context_menu(|ui| {
            if let Some(add_item) = add_ui(ui, self.extra_state) {
                let entity = self.world.spawn_empty().set_parent(entity).id();
                add_item.add_to_entity(self.world, entity);
            }
            // (context_menu)(ui, entity, self.world, self.extra_state)
        });
        // }

        new_selection
    }
}
