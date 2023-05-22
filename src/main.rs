use anyhow::anyhow;
use clap::Parser;
use pxsort::{Cli, load_image, PixelSort};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let mut image = load_image(&cli.input)?;
    let rgb8_image = image
        .as_mut_rgb8()
        .ok_or_else(|| anyhow!("failed to convert image to RGB8"))?;

    rgb8_image.sort_rgb8_pixels((&cli).into());

    rgb8_image.save(cli.output)?;

    Ok(())
}
