use bevy::prelude::*;
use bevy::render::render_resource::Extent3d;
use bevy_egui::egui::{self, TextureId};
use bevy_inspector_egui::bevy_inspector::hierarchy::SelectedEntities;
use bevy_inspector_egui::bevy_inspector::{
    self, ui_for_entities_shared_components, ui_for_entity_with_children,
};
use egui_gizmo::{GizmoMode, GizmoOrientation};

use crate::inspector::inspect_log::Logs;

use super::{
    add, draw_console_logs, draw_gizmo, hierarchy_ui, select_asset, select_resource,
    AddWindowState, InspectorSelection,
};

#[derive(Debug)]
pub enum EguiWindow {
    SceneView,
    GameView,
    Hierarchy,
    Resources,
    RuntimeAssets,
    Assets,
    Inspector,
    Console,
}

pub struct TabViewer<'a> {
    pub world: &'a mut World,
    pub hierarchy_state: &'a AddWindowState<'a>,
    pub selected_entities: &'a mut SelectedEntities,
    pub selection: &'a mut InspectorSelection,
    pub scene_rect: &'a mut Option<egui::Rect>,
    pub scene_render: Option<Handle<Image>>,
    pub scene_texture_id: Option<TextureId>,
    pub game_render: Option<Handle<Image>>,
    pub game_texture_id: Option<TextureId>,
    pub gizmo_mode: &'a mut GizmoMode,
    pub gizmo_orientation: &'a mut GizmoOrientation,
    pub filter_level_log: &'a mut log::Level,
    pub exist_game_camera: bool,
}

impl egui_dock::TabViewer for TabViewer<'_> {
    type Tab = EguiWindow;

    fn ui(&mut self, ui: &mut egui_dock::egui::Ui, window: &mut Self::Tab) {
        let type_registry = self.world.resource::<AppTypeRegistry>().0.clone();
        let type_registry = type_registry.read();

        match window {
            EguiWindow::SceneView => {
                let rect = ui.clip_rect();
                *self.scene_rect = Some(rect);

                if let Some(img) = self.scene_texture_id {
                    if let Some(render) = &self.scene_render {
                        let mut assets = self.world.resource_mut::<Assets<Image>>();
                        let img = assets.get_mut(&render).unwrap();
                        img.resize(Extent3d {
                            width: rect.size().x as u32,
                            height: rect.size().y as u32 - 32,
                            ..default()
                        })
                    }
                    ui.image(img, [rect.size().x - 10., rect.size().y - 32.]);
                }

                draw_gizmo(
                    ui,
                    self.world,
                    self.selected_entities,
                    *self.gizmo_mode,
                    *self.gizmo_orientation,
                );
            }
            EguiWindow::GameView => {
                let rect = ui.clip_rect();
                if let Some(img) = self.game_texture_id {
                    if let Some(render) = &self.game_render {
                        let mut assets = self.world.resource_mut::<Assets<Image>>();
                        let img = assets.get_mut(&render).unwrap();
                        img.resize(Extent3d {
                            width: rect.size().x as u32,
                            height: rect.size().y as u32 - 32,
                            ..default()
                        })
                    }
                    ui.image(img, [rect.size().x - 10., rect.size().y - 32.]);
                }

                if !self.exist_game_camera {
                    ui.horizontal_centered(|ui| {
                        ui.label("This Scene not have any Camera enabled");
                    });
                }
            }
            EguiWindow::Hierarchy => {
                let selected =
                    hierarchy_ui(self.world, ui, self.selected_entities, self.hierarchy_state);
                if selected {
                    *self.selection = InspectorSelection::Entities;
                }
            }
            EguiWindow::Resources => select_resource(ui, &type_registry, self.selection),
            EguiWindow::RuntimeAssets => {
                select_asset(ui, &type_registry, self.world, self.selection)
            }
            EguiWindow::Assets => {
                // TODO: implement show assets files
                ui.label("TODO: implement show assets files");
            }
            EguiWindow::Console => {
                let logs = self.world.resource::<Logs>();
                draw_console_logs(ui, self.filter_level_log, logs.clone());
            }
            EguiWindow::Inspector => match *self.selection {
                InspectorSelection::Entities => match self.selected_entities.as_slice() {
                    &[] => {}
                    &[entity] => ui_for_entity_with_children(self.world, entity, ui),
                    entities => ui_for_entities_shared_components(self.world, entities, ui),
                },
                InspectorSelection::Resource(type_id, ref name) => {
                    ui.label(name);
                    bevy_inspector::by_type_id::ui_for_resource(
                        self.world,
                        type_id,
                        ui,
                        name,
                        &type_registry,
                    )
                }
                InspectorSelection::Asset(type_id, ref name, handle) => {
                    ui.label(name);
                    bevy_inspector::by_type_id::ui_for_asset(
                        self.world,
                        type_id,
                        handle,
                        ui,
                        &type_registry,
                    );
                }
            },
        }
    }

    fn title(&mut self, window: &mut Self::Tab) -> egui_dock::egui::WidgetText {
        format!("{window:?}").into()
    }

    fn clear_background(&self, window: &Self::Tab) -> bool {
        !matches!(window, EguiWindow::SceneView | EguiWindow::GameView)
    }
}

//
// TODO: add component to entity
// TODO: get components from world registry and add to AddWindowState
//
#[allow(unused)]
fn add_ui(
    ui: &mut egui::Ui,
    entities: &[Entity],
    world: &mut World,
    add_window_state: Option<&AddWindowState>,
) {
    if let Some(add_window_state) = add_window_state {
        let layout = egui::Layout::top_down(egui::Align::Center).with_cross_justify(true);
        ui.with_layout(layout, |ui| {
            ui.menu_button("+", |ui| {
                if let Some(add_item) = add::add_ui(ui, add_window_state) {
                    for entity in entities {
                        add_item.add_to_entity(world, *entity);
                    }
                }
            });
        });
    }
}
