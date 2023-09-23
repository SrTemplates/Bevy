use std::path::PathBuf;

use bevy::prelude::{AppTypeRegistry, World};
use bevy_save::SnapshotDeserializer;
use serde::de::DeserializeSeed;

pub fn load_scene(world: &mut World) -> Option<PathBuf> {
    let curr_dir = std::env::current_dir().unwrap();
    if let Some(path) = rfd::FileDialog::new()
        .set_directory(&curr_dir)
        // .add_filter("scene", &["escene.ron"])
        .pick_file()
    {
        let scene_raw = std::fs::read_to_string(path.clone()).unwrap();
        let mut deserializer = ron::de::Deserializer::from_str(&scene_raw).unwrap();
        let registry = world.resource::<AppTypeRegistry>().clone();
        let reg = registry.read();
        let snapshot_deserializer = SnapshotDeserializer::new(&reg);
        match snapshot_deserializer.deserialize(&mut deserializer) {
            Ok(snap) => {
                if snap.into_applier(world).apply().is_ok() {
                    return Some(path);
                }
            }
            Err(e) => log::error!("Load Scene failed! {e:?}"),
        }
    }
    None
}
