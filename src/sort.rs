use std::{path::PathBuf, sync::mpsc::channel};

use image::{Rgb, RgbImage, ImageBuffer};
use rand::{seq::SliceRandom, thread_rng};
use rayon::prelude::*;
use crate::{SortingAlgorithm, WalkPath, ColorChannel, AnimateParams, Cli, Coefficients};

/// Sort the pixels of an `RGB8` image
///
/// Configurable with [`SortOptions`]
fn rgb8_pixel_sort(image: &mut RgbImage, options: SortOptions) {
    let sorter = options.by.into_rgb_sorter();

    let (width, height) = image.dimensions();
    let (outer_limit, inner_limit) = match options.direction {
        WalkPath::Horizontal => (height, width),
        WalkPath::Vertical => (width, height)
    };

    let interval = (1..=options.interval).collect::<Vec<_>>();

    let block_size = options.discretize;

    let (tx, rx) = channel();

    (0..outer_limit)
        .into_par_iter()
        .for_each_with(tx, |tx, outer| {
            let mut pixels = (0..inner_limit)
                .step_by(*interval.choose(&mut thread_rng()).unwrap())
                .map(|inner| match options.direction {
                    WalkPath::Horizontal => *image.get_pixel(inner, outer),
                    WalkPath::Vertical => *image.get_pixel(outer, inner)
                })
                .collect::<Vec<_>>();

            if !options.shuffle {
                if options.reverse {
                    pixels.reverse();
                    pixels.par_sort_unstable_by_key(|pixel| sorter(pixel, &options));
                    pixels.reverse();
                } else {
                    pixels.par_sort_unstable_by_key(|pixel| sorter(pixel, &options));
                }
            } else {
                pixels.par_iter_mut().for_each(|Rgb(pixel)| {
                    pixel.shuffle(&mut thread_rng());
                })
            }

            tx.send((outer, pixels)).unwrap();
        });

    match options.direction {
        WalkPath::Horizontal => rx.iter().for_each(|(y, sorted_row)| {
            sorted_row
                .into_iter()
                .enumerate()
                .step_by(*interval.choose(&mut thread_rng()).unwrap())
                .for_each(|(x, pixel)| image.put_pixel(x as u32, y, pixel))
        }),
        WalkPath::Vertical => rx.iter().for_each(|(y, sorted_row)| {
            sorted_row
                .into_iter()
                .enumerate()
                .step_by(*interval.choose(&mut thread_rng()).unwrap())
                .for_each(|(x, pixel)| image.put_pixel(y, x as u32, pixel))
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct SortOptions {
    pub by: SortingAlgorithm,
    pub interval: usize,
    pub reverse: bool,
    pub coefficients: Coefficients,
    pub discretize: u64,
    pub direction: WalkPath,
    pub splice: Option<f64>,
    pub edge_threshold: Option<u64>,
    pub image_threshold: Option<u64>,
    pub image_mask: Option<PathBuf>,
    pub channel: Option<ColorChannel>,
    pub animate: Option<AnimateParams>,
    pub shuffle: bool
}

impl From<Cli> for SortOptions {
    fn from(value: Cli) -> Self {
        Self {
            interval: value.interval,
            by: value.by,
            reverse: value.reverse,
            coefficients: Coefficients::from(&value),
            discretize: value.discretize,
            direction: value.direction,
            splice: value.splice,
            edge_threshold: value.edge_threshold,
            image_threshold: value.image_threshold,
            image_mask: value.image_mask,
            channel: value.channel,
            animate: value.animate,
            shuffle: value.shuffle
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
            coefficients: Coefficients::from(value),
            direction: value.direction,
            splice: value.splice,
            edge_threshold: value.edge_threshold,
            image_threshold: value.image_threshold,
            image_mask: value.image_mask.clone(),
            channel: value.channel,
            animate: value.animate.clone(),
            shuffle: value.shuffle
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////

/// Extension trait for an `RgbImage` to provide pixel sorting functionality
pub trait PixelSort {
    /// Sort the pixels by a key extraction function with options
    fn sort_rgb8_pixels(&mut self, options: SortOptions);
}

impl PixelSort for ImageBuffer<Rgb<u8>, Vec<u8>> {
    /// Sort the pixels by a key extraction function with options
    fn sort_rgb8_pixels(&mut self, options: SortOptions) {
        rgb8_pixel_sort(self, options);
    }
}
