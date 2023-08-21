use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::egui::{KeyboardShortcut, Modifiers, TextureId};
use bevy_egui::{egui, EguiContext};
use bevy_inspector_egui::bevy_inspector::hierarchy::SelectedEntities;
use egui_dock::{DockArea, NodeIndex, Tree};
use egui_gizmo::{GizmoMode, GizmoOrientation};

pub use add::*;
pub use gizmos::*;
pub use hierarchy::*;
pub use select::*;
pub use tab_viewer::*;
pub use widgets::*;

mod add;
mod gizmos;
mod hierarchy;
mod select;
mod tab_viewer;
mod widgets;

#[derive(Resource)]
pub struct UiState {
    pub tree: Tree<EguiWindow>,
    pub scene_rect: Option<egui::Rect>,
    pub scene_render: Option<Handle<Image>>,
    pub scene_texture_id: Option<TextureId>,
    pub game_render: Option<Handle<Image>>,
    pub game_texture_id: Option<TextureId>,
    pub selected_entities: SelectedEntities,
    pub selection: InspectorSelection,
    pub gizmo_mode: GizmoMode,
    pub gizmo_snap: GizmoSnapValues,
    pub gizmo_orientation: GizmoOrientation,
    pub hierarchy_state: AddWindowState<'static>,
    pub filter_level_log: log::Level,
    pub exist_game_camera: bool,
}

impl UiState {
    fn ui(&mut self, world: &mut World, ctx: &mut egui::Context) {
        let mut tab_viewer = TabViewer {
            world,
            exist_game_camera: self.exist_game_camera,
            scene_render: self.scene_render.clone(),
            game_render: self.game_render.clone(),
            scene_texture_id: self.scene_texture_id.clone(),
            game_texture_id: self.game_texture_id.clone(),
            selected_entities: &mut self.selected_entities,
            scene_rect: &mut self.scene_rect,
            selection: &mut self.selection,
            gizmo_mode: &mut self.gizmo_mode,
            gizmo_snap: &mut self.gizmo_snap,
            gizmo_orientation: &mut self.gizmo_orientation,
            hierarchy_state: &self.hierarchy_state,
            filter_level_log: &mut self.filter_level_log,
        };

        egui::CentralPanel::default()
            .frame(egui::Frame::central_panel(&ctx.style()).inner_margin(0.))
            .show(ctx, |ui| {
                egui::menu::bar(ui, |ui| file_menu_button(ui, &mut self.tree));
                ui.horizontal(|ui| tools_menu(ui, &mut tab_viewer));

                DockArea::new(&mut self.tree).show_inside(ui, &mut tab_viewer);
            });
    }
}

impl Default for UiState {
    fn default() -> Self {
        let mut tree = Tree::new(vec![EguiWindow::SceneView, EguiWindow::GameView]);
        let [game, _inspector] =
            tree.split_right(NodeIndex::root(), 0.75, vec![EguiWindow::Inspector]);
        let [game, _hierarchy] = tree.split_left(
            game,
            0.2,
            vec![EguiWindow::Hierarchy, EguiWindow::RuntimeAssets],
        );
        let [_game, _bottom] = tree.split_below(
            game,
            0.8,
            vec![
                EguiWindow::Resources,
                EguiWindow::Assets,
                EguiWindow::Console,
            ],
        );

        Self {
            tree,
            exist_game_camera: false,
            selected_entities: SelectedEntities::default(),
            selection: InspectorSelection::Entities,
            scene_render: None,
            scene_rect: None,
            game_render: None,
            scene_texture_id: None,
            game_texture_id: None,
            filter_level_log: log::max_level().to_level().unwrap_or(log::Level::Trace),
            hierarchy_state: AddWindowState::default(),
            gizmo_snap: GizmoSnapValues::default(),
            gizmo_mode: GizmoMode::Translate,
            gizmo_orientation: GizmoOrientation::Local,
        }
    }
}

pub fn show_ui_system(world: &mut World) {
    let Ok(egui_context) = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .get_single(world)
    else {
        return;
    };
    let mut egui_context = egui_context.clone();

    world.resource_scope::<UiState, _>(|world, mut ui_state| {
        ui_state.ui(world, egui_context.get_mut())
    });
}

fn file_menu_button(ui: &mut egui::Ui, tree: &mut Tree<EguiWindow>) {
    let save_shortcut = egui::KeyboardShortcut::new(egui::Modifiers::CTRL, egui::Key::S);

    ui.menu_button("File", |ui| {
        ui.set_min_width(220.0);
        ui.style_mut().wrap = Some(false);

        if ui
            .add(
                egui::Button::new("Save Scene")
                    .shortcut_text(ui.ctx().format_shortcut(&save_shortcut)),
            )
            .clicked()
        {
            // TODO: open file explorer if not have file.scn.ron
            log::info!("Scene Saved!");
            ui.close_menu();
        }
    });

    ui.menu_button("Window", |ui| {
        ui.set_min_width(220.0);
        ui.style_mut().wrap = Some(false);

        let buttons = [
            EguiWindow::SceneView,
            EguiWindow::GameView,
            EguiWindow::Hierarchy,
            EguiWindow::Inspector,
            EguiWindow::RuntimeAssets,
            EguiWindow::Resources,
            EguiWindow::Assets,
            EguiWindow::Console,
        ];

        for btn in buttons {
            if ui.button(format!("{btn:?}")).clicked() {
                tree.push_to_focused_leaf(btn);
                ui.close_menu();
            }
        }
    });
}

fn tools_menu(ui: &mut egui::Ui, tab_viewer: &mut TabViewer) {
    ui.add_space(20.);
    if let Some(gizmo_mode) = Selectable::new(
        &[
            (
                "T",
                GizmoMode::Translate,
                Some(KeyboardShortcut::new(Modifiers::NONE, egui::Key::E)),
            ),
            (
                "R",
                GizmoMode::Rotate,
                Some(KeyboardShortcut::new(Modifiers::NONE, egui::Key::R)),
            ),
            (
                "S",
                GizmoMode::Scale,
                Some(KeyboardShortcut::new(Modifiers::NONE, egui::Key::T)),
            ),
        ],
        *tab_viewer.gizmo_mode,
        18,
        0,
        egui::Color32::DARK_GRAY,
    )
    .show(ui)
    {
        *tab_viewer.gizmo_mode = gizmo_mode;
    }

    ui.add_space(10.);

    if let Some(orientation) = Selectable::new(
        &[
            (
                "G",
                GizmoOrientation::Global,
                Some(KeyboardShortcut::new(Modifiers::NONE, egui::Key::G)),
            ),
            (
                "L",
                GizmoOrientation::Local,
                Some(KeyboardShortcut::new(Modifiers::NONE, egui::Key::L)),
            ),
        ],
        *tab_viewer.gizmo_orientation,
        18,
        0,
        egui::Color32::DARK_GRAY,
    )
    .show(ui)
    {
        *tab_viewer.gizmo_orientation = orientation;
    }

    ui.add_space(10.);

    {
        ui.style_mut().visuals.button_frame = false;
        if ui
            .add(
                egui::Button::new("Snap")
                    .fill(if tab_viewer.gizmo_snap.enable {
                        egui::Color32::DARK_GRAY
                    } else {
                        egui::Color32::TRANSPARENT
                    })
                    .min_size(egui::Vec2::splat(18.0))
                    .rounding(egui::Rounding::none()),
            )
            .context_menu(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Transform Snap:");
                    ui.add(
                        egui::DragValue::new(&mut tab_viewer.gizmo_snap.distance)
                            .clamp_range(0.0..=std::f32::MAX),
                    );
                });

                ui.horizontal(|ui| {
                    ui.label("Rotation Snap:");
                    ui.add(
                        egui::DragValue::new(&mut tab_viewer.gizmo_snap.angle)
                            .clamp_range(0.0..=360.0)
                            .suffix(" °"),
                    );
                });

                ui.horizontal(|ui| {
                    ui.label("Scale Snap:");
                    ui.add(
                        egui::DragValue::new(&mut tab_viewer.gizmo_snap.scale)
                            .clamp_range(0.0..=std::f32::MAX),
                    );
                });
            })
            .clicked()
        {
            tab_viewer.gizmo_snap.enable = !tab_viewer.gizmo_snap.enable;
        }
    }
}
