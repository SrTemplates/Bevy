use bevy::prelude::*;
use bevy::render::render_resource::{
    Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
};
use bevy::window::PrimaryWindow;
use bevy_egui::EguiUserTextures;
use bevy_infinite_grid::{GridShadowCamera, InfiniteGridBundle};

use super::camera_movement::FlycamControls;
use super::ui::UiState;

#[derive(Component)]
pub struct MainSceneCamera;

#[derive(Component)]
pub struct MainGameCamera;

#[derive(Component)]
pub struct InspectorEntity;

fn create_image() -> Image {
    let size = Extent3d {
        width: 512,
        height: 512,
        ..default()
    };

    // This is the texture that will be rendered scene.
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };
    // fill image.data with zeroes
    image.resize(size);
    image
}

pub fn setup(
    mut commands: Commands,
    windows: Query<Entity, With<Window>>,
    mut egui_user_textures: ResMut<EguiUserTextures>,
    mut images: ResMut<Assets<Image>>,
    mut ui_state: ResMut<UiState>,
) {
    //
    // Prepare render Scene image
    //
    let image_scene_handle = images.add(create_image());

    commands.spawn((InfiniteGridBundle::default(), InspectorEntity));

    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0., 3., 10.))
                .with_rotation(Quat::from_rotation_x(-0.2)),
            camera: Camera {
                target: bevy::render::camera::RenderTarget::Image(image_scene_handle.clone()),
                ..default()
            },
            ..default()
        },
        InspectorEntity,
        FlycamControls::default(),
        MainSceneCamera,
        GridShadowCamera,
    ));

    for e in windows.iter() {
        let mut entity = commands.entity(e);
        entity.insert(InspectorEntity);
    }

    //
    // Prepare render images
    //
    ui_state.scene_texture_id = Some(egui_user_textures.add_image(image_scene_handle.clone()));

    let image_game_handle = images.add(create_image());
    ui_state.game_texture_id = Some(egui_user_textures.add_image(image_game_handle.clone()));

    ui_state.scene_render = Some(image_scene_handle);
    ui_state.game_render = Some(image_game_handle);
}

// make camera only render to view not obstructed by UI
pub fn set_camera_viewport(
    mut ui_state: ResMut<UiState>,
    primary_window: Query<&mut Window, With<PrimaryWindow>>,
    _egui_settings: Res<bevy_egui::EguiSettings>,
    // mut cameras: Query<&mut Camera, With<MainSceneCamera>>,
    mut game_camera: Query<&mut Camera, (With<MainGameCamera>, Without<MainSceneCamera>)>,
) {
    let Ok(_window) = primary_window.get_single() else {
        return;
    };

    // Game Camera
    for mut cam in game_camera.iter_mut() {
        if !cam.is_active {
            ui_state.exist_game_camera = false;
            continue;
        }
        if let Some(render) = &ui_state.game_render {
            cam.target = bevy::render::camera::RenderTarget::Image(render.clone());
        }
        ui_state.exist_game_camera = true;
        break;
    }
}
