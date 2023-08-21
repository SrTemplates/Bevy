use bevy::prelude::*;
use bevy::render::camera::CameraProjection;
use bevy_egui::egui;
use bevy_inspector_egui::bevy_inspector::hierarchy::SelectedEntities;
use egui_gizmo::{Gizmo, GizmoMode, GizmoOrientation};

use crate::inspector::default_scene::MainSceneCamera;

#[derive(Clone, Copy, Default)]
pub struct GizmoSnapValues {
    pub enable: bool,
    pub distance: f32,
    pub angle: f32,
    pub scale: f32,
}

pub fn draw_gizmo(
    ui: &mut egui::Ui,
    world: &mut World,
    selected_entities: &SelectedEntities,
    gizmo_mode: GizmoMode,
    gizmo_orientation: GizmoOrientation,
    gizmo_snap: GizmoSnapValues,
) {
    let (cam_transform, projection) = world
        .query_filtered::<(&GlobalTransform, &Projection), With<MainSceneCamera>>()
        .single(world);
    let view_matrix = Mat4::from(cam_transform.affine().inverse());
    let projection_matrix = projection.get_projection_matrix();

    if selected_entities.len() != 1 {
        return;
    }

    for selected in selected_entities.iter() {
        let Some(transform) = world.get::<Transform>(selected) else {
            continue;
        };
        let model_matrix = transform.compute_matrix();

        let Some(result) = Gizmo::new(selected)
            .model_matrix(model_matrix.to_cols_array_2d())
            .view_matrix(view_matrix.to_cols_array_2d())
            .projection_matrix(projection_matrix.to_cols_array_2d())
            .orientation(gizmo_orientation)
            .snapping(gizmo_snap.enable && gizmo_snap.distance > 0. && gizmo_snap.angle > 0. && gizmo_snap.scale > 0.)
            .snap_distance(gizmo_snap.distance)
            .snap_angle(gizmo_snap.angle)
            .snap_scale(gizmo_snap.scale)
            .mode(gizmo_mode)
            .interact(ui)
        else {
            continue;
        };

        let mut transform = world.get_mut::<Transform>(selected).unwrap();
        *transform = Transform {
            translation: Vec3::from(<[f32; 3]>::from(result.translation)),
            rotation: Quat::from_array(<[f32; 4]>::from(result.rotation)),
            scale: Vec3::from(<[f32; 3]>::from(result.scale)),
        };
    }
}
