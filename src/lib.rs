//! `scannedpdf` is a simple images to pdf library.
//!
//! [GitHub](https://github.com/lihe07/scannedpdf)
//!
//! ## An example
//!
//! ```rust
//! let default_config = scannedpdf::PageConfig::new()
//!     .quality(50)
//!     .vertical_alignment(scannedpdf::Alignment::Center)
//!     .margin(scannedpdf::Margin::Custom(20, 20));
//!
//! let images = vec!["./1.jpg", "2.jpg", "3.jpg"];
//! let mut file = scannedpdf::create("./test.pdf", default_config, images.len()).unwrap();
//! for image in images {
//!    file.add_page_from_path(&image, Some(format!("Image: {}", image)), None)
//!        .unwrap();
//! }
//! file.finish().unwrap();
//! ```

use std::{fs::File, path::Path};

mod config;
mod error;
mod image;
mod pdf;

// Re-export
pub use config::*;
pub use error::Error;
pub use pdf::PDF;

/// An alias of `scannedpdf::PDF::create_file`
pub fn create<P: AsRef<Path>>(
    path: P,
    default_page_config: PageConfig,
    total_pages: usize,
) -> std::io::Result<pdf::PDF<std::fs::File>> {
    pdf::PDF::create(File::create(path)?, default_page_config, total_pages)
}
