use bevy::{log::LogPlugin, prelude::*, window::WindowMode};
#[cfg(feature = "inspect")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub const LAUNCHER_TITLE: &str = "{{game_name}}";

//
// App library entrypoint from launchers
//
pub fn app(fullscreen: bool) -> App {
    let mode = if fullscreen {
        WindowMode::BorderlessFullscreen
    } else {
        WindowMode::Windowed
    };

    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    mode,
                    title: LAUNCHER_TITLE.to_string(),
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: true,
                    present_mode: bevy::window::PresentMode::AutoVsync,
                    decorations: false,
                    ..default()
                }),
                ..default()
            })
            .disable::<LogPlugin>(),
    );
    #[cfg(feature = "inspect")]
    app.add_plugins(WorldInspectorPlugin::new());

    app
}
