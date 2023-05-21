use std::path::Path;

use anyhow::anyhow;
use image::DynamicImage;

/// Loads an image into memory from `path`
pub fn load_image<T: AsRef<Path>>(path: T) -> anyhow::Result<DynamicImage> {
    image::io::Reader::open(path.as_ref())?
        .decode()
        .map_err(|error| anyhow!("error decoding image: {}", error))
}
