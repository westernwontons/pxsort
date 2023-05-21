#![allow(unused_variables, unused_mut, unused_imports, dead_code, unused_assignments)]

use image::{Rgb};
use itertools::Itertools;

/// Calculate the intensity of an `RGB` pixel
pub fn intensity(Rgb(pixel): &Rgb<u8>) -> u8 {
    (pixel
        .iter()
        .map(|i| *i as u16)
        .sum::<u16>()
        .wrapping_div(3)) as u8
}

/// Calculcate the brightness of an `RGB` pixel
pub fn brightness(Rgb(pixel): &Rgb<u8>) -> u8 {
    (pixel
        .iter()
        .max()
        .unwrap()
        .wrapping_add(*pixel.iter().min().unwrap())
        .wrapping_div(2)) as u8
}

/// Calculate the luma value of an `RGB` pixel
pub fn luma(Rgb([r, g, b]): &Rgb<u8>) -> u8 {
    (0.2126 * (*r as f32) + 0.7152 * (*g as f32) + 0.0722 * (*b as f32)) as u8
}

/// Calculate the chroma value of an `RGB` pixel
pub fn chroma(Rgb(pixel): &Rgb<u8>) -> u8 {
    let (&min, &max) = pixel.iter().minmax().into_option().unwrap();
    max.wrapping_sub(min)
}

/// Calculate the hue value of an `Rgb` pixel
pub fn hue(Rgb(pixel): &Rgb<u8>) -> u8 {
    let [red, green, blue] = pixel.map(|channel| channel as f32 / 255.0);
    let min = pixel.iter().min().unwrap();
    let max = pixel.iter().max().unwrap();

    if max == min {
        // hue is undefined for grayscale colors, return arbitrary value
        return 0;
    }

    let diff = (max - min) as f32;
    let mut hue = match max {
        r if r == max => (green - blue) / diff,
        g if g == max => 2.0 + (blue - red) / diff,
        _ => 4.0 + (red - green) / diff
    };

    hue *= 60.0;
    if hue < 0.0 {
        hue += 360.0;
    }

    hue as u8
}

/// Calculate the saturation of an `RGB` pixel
pub fn saturation(Rgb(pixel): &Rgb<u8>) -> u8 {
    let (&min, &max) = pixel.iter().minmax().into_option().unwrap();
    if max != 0 {
        max.wrapping_sub(min) / max
    } else {
        0
    }
}
