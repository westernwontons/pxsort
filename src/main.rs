use clap::Parser;
use pxsort::sort::sort_by;
use pxsort::{Cli, load_image};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let mut image = load_image(&cli.input)?.to_rgb8();

    sort_by(&mut image, (&cli).into());
    image.save(cli.output)?;
    return Ok(());
}
