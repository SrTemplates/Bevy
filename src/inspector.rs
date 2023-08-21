use bevy::prelude::*;
use bevy_egui::EguiSet;
use bevy_inspector_egui::DefaultInspectorConfigPlugin;
use default_scene::{set_camera_viewport, setup as setup_scene};
use camera_movement::*;
use inspect_log::*;
use ui::*;

mod camera_movement;
mod default_scene;
mod inspect_log;
mod ui;

pub use default_scene::MainGameCamera;

pub struct InspectorPlugin;

impl Plugin for InspectorPlugin {
    fn build(&self, app: &mut App) {
        let logger = Logs::default();
        app.init_resource::<UiState>()
            .insert_resource(logger.clone())
            .add_plugins(DefaultInspectorConfigPlugin)
            .add_plugins(bevy_egui::EguiPlugin)
            .add_plugins(bevy_infinite_grid::InfiniteGridPlugin);
        app.add_systems(Startup, setup_scene)
            .add_systems(
                PostUpdate,
                show_ui_system
                    .before(EguiSet::ProcessOutput)
                    .before(bevy::transform::TransformSystem::TransformPropagate),
            )
            .add_systems(PostUpdate, set_camera_viewport.after(show_ui_system))
            .add_systems(PostUpdate, (camera_movement, camera_look))
            .register_type::<Option<Handle<Image>>>()
            .register_type::<AlphaMode>();

        log::set_max_level(log::LevelFilter::Trace);
        log::set_boxed_logger(Box::from(logger)).unwrap();
    }
}
