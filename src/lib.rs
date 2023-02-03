use std::path::Path;
mod pdf;
mod config;
mod image;

pub use config::*;

/// An alias of scannedpdf::PDF::create_file
pub fn create<P: AsRef<Path>>(path: P, config: Config) -> std::io::Result<pdf::PDF<std::fs::File>> {
    pdf::PDF::create_file(path, config)
}