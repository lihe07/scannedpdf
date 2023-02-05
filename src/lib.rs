use std::{fs::File, path::Path};
mod config;
mod error;
mod image;
mod pdf;

pub use config::*;
pub use error::Error;
pub use pdf::PDF;

/// An alias of scannedpdf::PDF::create_file
pub fn create<P: AsRef<Path>>(
    path: P,
    default_page_config: PageConfig,
    total_pages: usize,
) -> std::io::Result<pdf::PDF<std::fs::File>> {
    pdf::PDF::create(File::create(path)?, default_page_config, total_pages)
}
