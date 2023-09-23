mod load;
mod save;

use std::fs::File;
use std::io::{BufReader, BufWriter};

use bevy_save::{Backend, SaveableError};

pub use load::*;
pub use save::*;

pub struct SaveSystem;
impl Backend for SaveSystem {
    type Reader = BufReader<File>;
    type Writer = BufWriter<File>;

    fn reader(name: &str) -> Result<Self::Reader, bevy_save::SaveableError> {
        let file = std::fs::File::open(name).map_err(SaveableError::other)?;

        Ok(std::io::BufReader::new(file))
    }

    fn writer(name: &str) -> Result<Self::Writer, bevy_save::SaveableError> {
        let path = std::path::PathBuf::from(name);
        let dir = path.parent().expect("Invalid save directory");

        std::fs::create_dir_all(dir).map_err(SaveableError::other)?;

        let file = std::fs::File::create(path).map_err(SaveableError::other)?;

        Ok(std::io::BufWriter::new(file))
    }
}
