use bevy::prelude::*;
use bevy_egui::EguiSet;
use bevy_inspector_egui::DefaultInspectorConfigPlugin;
use bevy_save::{AppBackend, AppSaveableExt, AppSaver, SavePlugins};
use camera_movement::*;
use default_scene::{set_camera_viewport, setup as setup_scene};
use inspect_log::*;
use ui::*;

mod camera_movement;
mod default_scene;
mod inspect_log;
mod scene;
mod ui;

pub use default_scene::MainGameCamera;

use self::scene::{RONSaver, SaveSystem};

pub struct InspectorPlugin;
impl Plugin for InspectorPlugin {
    fn build(&self, app: &mut App) {
        let logger = Logs::default();
        app.init_resource::<UiState>()
            .insert_resource(logger.clone())
            .insert_resource(AppSaver::new(RONSaver))
            .insert_resource(AppBackend::new(SaveSystem))
            .add_plugins(DefaultInspectorConfigPlugin)
            .add_plugins(bevy_egui::EguiPlugin)
            .add_plugins(bevy_infinite_grid::InfiniteGridPlugin)
            .add_plugins(SavePlugins);
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

        app.register_saveable::<Handle<ColorMaterial>>()
            .register_saveable::<Handle<StandardMaterial>>()
            .register_saveable::<Text>()
            .register_saveable::<UiImage>();

        log::set_max_level(log::LevelFilter::Trace);
        log::set_boxed_logger(Box::from(logger)).unwrap();
    }
}
