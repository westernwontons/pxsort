use std::{process, path::PathBuf};

use image::{Rgb, RgbImage, ImageBuffer};
use rayon::{
    prelude::{IntoParallelIterator, ParallelIterator},
    slice::ParallelSliceMut
};
use crate::{
    extractor::{luma, chroma, saturation, intensity, brightness, hue},
    SortingAlgorithm, WalkPath, ColorChannel, AnimateParams, Cli
};

fn pixel_sort_with_options<F>(image: &mut RgbImage, sort_options: SortOptions, sorter: F)
where
    F: Fn(&Rgb<u8>) -> u8 + Sync + Copy
{
    let (outer_limit, inner_limit) = {
        let (width, height) = image.dimensions();
        match sort_options.direction {
            WalkPath::Horizontal => (height, width),
            WalkPath::Vertical => (width, height),
            WalkPath::Concentric => {
                eprintln!("Not implemented");
                process::exit(0);
            }
            WalkPath::Diagonal => {
                eprintln!("Not implemented");
                process::exit(0);
            }
        }
    };

    // Sort each row in parallel and collect the results
    let sorted_rows = (0..outer_limit)
        .into_par_iter()
        .map(|outer| {
            let mut pixels = (0..inner_limit)
                .step_by(sort_options.interval)
                .map(|inner| match sort_options.direction {
                    WalkPath::Horizontal => *image.get_pixel(inner, outer),
                    WalkPath::Vertical => *image.get_pixel(outer, inner),
                    _ => unimplemented!()
                })
                .collect::<Vec<_>>();

            if sort_options.reverse {
                pixels.reverse();
                pixels.par_sort_unstable_by_key(sorter);
                pixels.reverse();
            } else {
                pixels.par_sort_unstable_by_key(sorter);
            }

            pixels
        })
        .collect::<Vec<Vec<_>>>();

    for (x, y, pixel) in image.enumerate_pixels_mut() {
        match sort_options.direction {
            WalkPath::Horizontal => *pixel = sorted_rows[y as usize][x as usize],
            WalkPath::Vertical => *pixel = sorted_rows[x as usize][y as usize],
            _ => unimplemented!()
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct SortOptions {
    pub interval: usize,
    pub by: SortingAlgorithm,
    pub reverse: bool,
    pub discretize: Option<u64>,
    pub direction: WalkPath,
    pub mirror: Option<f64>,
    pub splice: Option<f64>,
    pub edge_threshold: Option<u64>,
    pub image_threshold: Option<u64>,
    pub image_mask: Option<PathBuf>,
    pub channel: Option<ColorChannel>,
    pub animate: Option<AnimateParams>
}

impl From<Cli> for SortOptions {
    fn from(value: Cli) -> Self {
        Self {
            interval: value.interval,
            by: value.by,
            reverse: value.reverse,
            discretize: value.discretize,
            direction: value.direction,
            mirror: value.mirror,
            splice: value.splice,
            edge_threshold: value.edge_threshold,
            image_threshold: value.image_threshold,
            image_mask: value.image_mask,
            channel: value.channel,
            animate: value.animate
        }
    }
}

impl From<&Cli> for SortOptions {
    fn from(value: &Cli) -> Self {
        Self {
            interval: value.interval,
            by: value.by,
            reverse: value.reverse,
            discretize: value.discretize,
            direction: value.direction,
            mirror: value.mirror,
            splice: value.splice,
            edge_threshold: value.edge_threshold,
            image_threshold: value.image_threshold,
            image_mask: value.image_mask.clone(),
            channel: value.channel,
            animate: value.animate.clone()
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////

/// Extension trait for an `RgbImage` to provide pixel sorting functionality
pub trait PixelSort {
    /// Sort the pixels by `luma` with options
    fn luma_sort_with_options(&mut self, sort_options: SortOptions);

    /// Sort the pixels by `chroma` with options
    fn chroma_sort_with_options(&mut self, sort_options: SortOptions);

    /// Sort the pixels by `saturation` with options
    fn saturation_sort_with_options(&mut self, sort_options: SortOptions);

    /// Sort the pixels by `hue` with options
    fn hue_sort_with_options(&mut self, sort_options: SortOptions);

    /// Sort the pixels by `brightness` with options
    fn brightness_sort_with_options(&mut self, sort_options: SortOptions);

    /// Sort the pixels by `intensity` with options
    fn intensity_sort_with_options(&mut self, sort_options: SortOptions);
}

impl PixelSort for ImageBuffer<Rgb<u8>, Vec<u8>> {
    /// Sort the pixels by `luma` with options
    fn luma_sort_with_options(&mut self, sort_options: SortOptions) {
        pixel_sort_with_options(self, sort_options, luma);
    }

    /// Sort the pixels by `chroma` with options
    fn chroma_sort_with_options(&mut self, sort_options: SortOptions) {
        pixel_sort_with_options(self, sort_options, chroma);
    }

    /// Sort the pixels by `saturation` with options
    fn saturation_sort_with_options(&mut self, sort_options: SortOptions) {
        pixel_sort_with_options(self, sort_options, saturation);
    }

    /// Sort the pixels by `hue` with options
    fn hue_sort_with_options(&mut self, sort_options: SortOptions) {
        pixel_sort_with_options(self, sort_options, hue);
    }

    /// Sort the pixels by `brightness` with options
    fn brightness_sort_with_options(&mut self, sort_options: SortOptions) {
        pixel_sort_with_options(self, sort_options, brightness);
    }

    /// Sort the pixels by `intensity` with options
    fn intensity_sort_with_options(&mut self, sort_options: SortOptions) {
        pixel_sort_with_options(self, sort_options, intensity);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////

/// Sort by a key extraction function
pub fn sort_by(image: &mut RgbImage, sort_options: SortOptions) {
    match sort_options.by {
        SortingAlgorithm::Luma => image.luma_sort_with_options(sort_options),
        SortingAlgorithm::Chroma => image.chroma_sort_with_options(sort_options),
        SortingAlgorithm::Saturation => image.saturation_sort_with_options(sort_options),
        SortingAlgorithm::Hue => image.hue_sort_with_options(sort_options),
        SortingAlgorithm::Brightness => image.brightness_sort_with_options(sort_options)
    }
}
