#![allow(
    unused_variables,
    unused_mut,
    unused_imports,
    dead_code,
    unused_assignments
)]

mod cli;

use clap::Parser;
use cli::Cli;
use image::buffer::EnumeratePixels;
use image::io::Reader as ImageReader;
use image::{
    DynamicImage, GenericImage, GenericImageView, ImageBuffer, ImageFormat,
    Rgb, RgbImage, Rgba, RgbaImage
};
use std::cmp::Ordering;
use std::error::Error;
use std::ops::{Index, IndexMut};
use std::slice::Iter;

fn sort_by_ilog(
    bytes: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    width: u32,
    buffer: &mut Vec<u8>,
    divide_width_by: usize,
    ilog: u8
) {
    for element in bytes.chunks(width as usize / divide_width_by) {
        let mut el = element.to_owned();
        el.sort_by_key(|key| key.checked_ilog(ilog));
        buffer.extend(el);
    }
}

fn create_image(
    width: u32,
    height: u32,
    buffer: Vec<u8>
) -> anyhow::Result<ImageBuffer<Rgb<u8>, Vec<u8>>> {
    let mut new_image = RgbImage::from_vec(width, height, buffer)
        .ok_or(anyhow::anyhow!("Failed to create image from buffer"))?;
    Ok(new_image)
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let mut image = ImageReader::open(cli.filename)?
        .with_guessed_format()?
        .decode()?;
    let (width, height) = image.dimensions();
    let mut bytes = image.as_mut_rgb8().ok_or(anyhow::anyhow!("Fail"))?;
    let mut buffer = bytes.clone();

    // sort_by_ilog(bytes, width, &mut buffer, 10, 20);

    for (y, row) in bytes.enumerate_rows() {
        for (w, h, Rgb(rgb)) in row {
            // let sum =
            //     rgb.iter().fold(0 as u32, |curr, next| curr + *next as u32);

            // if sum > 50 {
            //     *buffer.index_mut((w, h)) = Rgb::from(*rgb);
            // }
        }
    }

    buffer.save(cli.output)?;

    Ok(())
}
