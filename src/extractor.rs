use image::Rgb;
use itertools::Itertools;
use crate::sort::SortOptions;

/// Update the RGB8 pixel with the coefficients
///
/// Only used in `intensity`, `brightness`, `chroma` and `saturation`
fn update_pixel(pixel: &[u8; 3], options: &SortOptions) -> [u8; 3] {
    let red = if options.coefficients.red != 0.0 {
        (pixel[0] as f32 * options.coefficients.red) as u8
    } else {
        pixel[0]
    };

    let green = if options.coefficients.green != 0.0 {
        (pixel[1] as f32 * options.coefficients.green) as u8
    } else {
        pixel[1]
    };

    let blue = if options.coefficients.blue != 0.0 {
        (pixel[2] as f32 * options.coefficients.blue) as u8
    } else {
        pixel[2]
    };

    [red, green, blue]
}

/// Calculate the intensity of an `RGB` pixel
pub fn intensity(Rgb(pixel): &Rgb<u8>, options: &SortOptions) -> u8 {
    let pixel = update_pixel(pixel, options);
    (pixel
        .iter()
        .map(|i| *i as u16)
        .sum::<u16>()
        .wrapping_div(3)) as u8
}

/// Calculcate the brightness of an `RGB` pixel
pub fn brightness(Rgb(pixel): &Rgb<u8>, options: &SortOptions) -> u8 {
    let pixel = update_pixel(pixel, options);
    let (&min, &max) = pixel.iter().minmax().into_option().unwrap();
    (max.wrapping_add(min).wrapping_div(2)) as u8
}

/// Calculate the luma value of an `RGB` pixel
pub fn luma(Rgb([r, g, b]): &Rgb<u8>, options: &SortOptions) -> u8 {
    (options.coefficients.red * (*r as f32)
        + options.coefficients.green * (*g as f32)
        + options.coefficients.blue * (*b as f32)) as u8
}

/// Calculate the chroma value of an `RGB` pixel
pub fn chroma(Rgb(pixel): &Rgb<u8>, options: &SortOptions) -> u8 {
    let pixel = update_pixel(pixel, options);
    let (&min, &max) = pixel.iter().minmax().into_option().unwrap();
    max.wrapping_sub(min)
}

/// Calculate the hue value of an `Rgb` pixel
pub fn hue(Rgb(pixel): &Rgb<u8>, options: &SortOptions) -> u8 {
    let [red, green, blue] = pixel.map(|channel| channel as f32 / 255.0);
    let (&min, &max) = pixel.iter().minmax().into_option().unwrap();

    if max == min {
        // hue is undefined for grayscale colors, return arbitrary value
        return 0;
    }

    let diff = (max - min) as f32;
    let mut hue = match max {
        r if r == max => options.coefficients.red + (green - blue) / diff,
        g if g == max => options.coefficients.green + (blue - red) / diff,
        _ => options.coefficients.blue + (red - green) / diff
    };

    hue *= 60.0;
    if hue < 0.0 {
        hue += 360.0;
    }

    hue as u8
}

/// Calculate the saturation of an `RGB` pixel
pub fn saturation(Rgb(pixel): &Rgb<u8>, options: &SortOptions) -> u8 {
    let pixel = update_pixel(pixel, options);
    let (&min, &max) = pixel.iter().minmax().into_option().unwrap();
    if max != 0 {
        max.wrapping_sub(min) / max
    } else {
        0
    }
}
