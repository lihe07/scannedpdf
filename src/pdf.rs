use std::path::Path;
use std::io::Write;
use std::fs::File;

use crate::{PageConfig, Config};

// PDF related
pub struct PDF<W: Write> {
    config: Config,
    writer: W
}

impl PDF<File> {
    pub fn create_file<P: AsRef<Path>>(path: P, config: Config) -> std::io::Result<Self> {
        Ok(PDF {
            config,
            writer: File::create(path)?
        })
    }
}

impl<W: Write> PDF<W> {

    pub fn add_page_by_path<P: AsRef<Path>>(&mut self, image_path: P, page_config: Option<PageConfig>) {

    }
}