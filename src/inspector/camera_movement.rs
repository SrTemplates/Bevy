use bevy::input::mouse::MouseWheel;
use bevy::window::PrimaryWindow;
use bevy::{input::mouse::MouseMotion, prelude::*};
use bevy_egui::egui::Pos2;

use super::ui::UiState;

#[derive(Component)]
pub struct FlycamControls {
    pub yaw: f32,
    pub pitch: f32,
    pub sensitivity: f32,
    pub enable_movement: bool,
    pub enable_look: bool,

    pub key_forward: KeyCode,
    pub key_back: KeyCode,
    pub key_left: KeyCode,
    pub key_right: KeyCode,
    pub key_up: KeyCode,
    pub key_down: KeyCode,
    pub key_boost: KeyCode,
}
impl Default for FlycamControls {
    fn default() -> Self {
        Self {
            yaw: Default::default(),
            pitch: Default::default(),
            sensitivity: 1.0,
            enable_movement: true,
            enable_look: true,
            key_forward: KeyCode::W,
            key_back: KeyCode::S,
            key_left: KeyCode::A,
            key_right: KeyCode::D,
            key_up: KeyCode::Space,
            key_down: KeyCode::ControlLeft,
            key_boost: KeyCode::ShiftLeft,
        }
    }
}

pub fn camera_movement(
    time: Res<Time>,
    ui_state: Res<UiState>,
    keyboard_input: Res<Input<KeyCode>>,
    mut scroll_events: EventReader<MouseWheel>,
    mut cam: Query<(&FlycamControls, &mut Transform)>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let Some(rect) = ui_state.scene_rect else { return; };
    let Ok(window) = windows.get_single() else { return; };
    let pos = window.cursor_position().unwrap_or(Vec2::ZERO);

    let (flycam, mut cam_transform) = cam.single_mut();

    let distance = rect.distance_to_pos(Pos2::new(pos.x, pos.y));

    if !flycam.enable_movement || ui_state.scene_rect.is_none() || distance > 0. {
        return;
    }

    let scroll = scroll_events
        .iter()
        .map(|ev| ev.y)
        .sum::<f32>()
        .clamp(-1., 1.);

    let if_then_1 = |b| if b { 1.0 } else { 0.0 };
    let forward = if_then_1(keyboard_input.pressed(flycam.key_forward))
        - if_then_1(keyboard_input.pressed(flycam.key_back));
    let sideways = if_then_1(keyboard_input.pressed(flycam.key_right))
        - if_then_1(keyboard_input.pressed(flycam.key_left));
    let up = if_then_1(keyboard_input.pressed(flycam.key_up))
        - if_then_1(keyboard_input.pressed(flycam.key_down));

    if forward == 0.0 && sideways == 0.0 && up == 0.0 {
        return;
    }

    let speed = if keyboard_input.pressed(flycam.key_boost) {
        20.0
    } else {
        5.0
    };

    let movement = Vec3::new(sideways, forward + scroll, up).normalize_or_zero()
        * speed
        * time.raw_delta_seconds();

    let diff = cam_transform.forward() * movement.y
        + cam_transform.right() * movement.x
        + cam_transform.up() * movement.z;
    cam_transform.translation += diff;
}

pub fn camera_look(
    ui_state: Res<UiState>,
    mouse_input: Res<Input<MouseButton>>,
    mut mouse_motion_event_reader: EventReader<MouseMotion>,
    mut query: Query<(&mut FlycamControls, &mut Transform)>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let Some(rect) = ui_state.scene_rect else { return; };
    let Ok(window) = windows.get_single() else { return; };
    let pos = window.cursor_position().unwrap_or(Vec2::ZERO);
    let distance = rect.distance_to_pos(Pos2::new(pos.x, pos.y));

    let (mut flycam, mut transform) = query.single_mut();
    if !mouse_input.pressed(MouseButton::Right) || distance > 0. {
        return;
    }
    if !flycam.enable_look {
        return;
    }
    let mut delta: Vec2 = Vec2::ZERO;
    for event in mouse_motion_event_reader.iter() {
        delta += event.delta;
    }
    if delta.is_nan() || delta.abs_diff_eq(Vec2::ZERO, f32::EPSILON) {
        return;
    }

    flycam.yaw -= delta.x / 180.0 * flycam.sensitivity;
    flycam.pitch -= delta.y / 180.0 * flycam.sensitivity;

    flycam.pitch = flycam
        .pitch
        .clamp(-std::f32::consts::PI / 2.0, std::f32::consts::PI / 2.0);

    transform.rotation = Quat::from_euler(EulerRot::YXZ, flycam.yaw, flycam.pitch, 0.0);
}
