use std::path::PathBuf;

use bevy::prelude::World;
use bevy_save::prelude::IntoSerializer;
use bevy_save::{Saver, Writer, WorldSaveableExt};

pub struct RONSaver;
impl Saver for RONSaver {
    fn serializer<'w>(&self, writer: Writer<'w>) -> IntoSerializer<'w> {
        IntoSerializer::erase(
            ron::ser::Serializer::new(writer, Some(ron::ser::PrettyConfig::default()))
                .expect("Failed to create Serializer"),
        )
    }
}

pub fn save_scene(curr_path: &mut Option<PathBuf>, world: &mut World) -> bool {
    let curr_dir = std::env::current_dir().unwrap();
    if let Some(path) = curr_path {
        return world
            .save(path.to_str().unwrap_or("Scene.scene.ron"))
            .is_ok();
    }
    if let Some(path) = rfd::FileDialog::new()
        .set_file_name("Scene.scene.ron")
        // .add_filter("scene", &["escene.ron"])
        .set_directory(&curr_dir)
        .save_file()
    {
        curr_path.replace(path.clone());
        return world
            .save(path.to_str().unwrap_or("Scene.scene.ron"))
            .is_ok();
    }
    false
}
