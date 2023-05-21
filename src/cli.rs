use anyhow::{anyhow, bail, Context};
use clap::ValueEnum;
use std::{path::PathBuf, fmt::Display};

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ArgumentList {
    Interval,
    Reverse,
    Discretize,
    Path,
    Mirror,
    Splice,
    EdgeThreshold,
    ImageThreshold,
    ImageMask,
    UseTiles,
    TileX,
    TileY,
    Channel
}

impl TryFrom<&str> for ArgumentList {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> anyhow::Result<Self> {
        match value.to_lowercase().as_str() {
            "interval" => Ok(Self::Interval),
            "reverse" => Ok(Self::Reverse),
            "discretize" => Ok(Self::Discretize),
            "path" => Ok(Self::Path),
            "mirror" => Ok(Self::Mirror),
            "splice" => Ok(Self::Splice),
            "edge_threshold" => Ok(Self::EdgeThreshold),
            "image_threshold" => Ok(Self::ImageThreshold),
            "image_mask" => Ok(Self::ImageMask),
            "use_tiles" => Ok(Self::UseTiles),
            "tile_x" => Ok(Self::TileX),
            "tile_y" => Ok(Self::TileY),
            "channel" => Ok(Self::Channel),
            _ => bail!("'param' has to be one of: interval, reverse, discretize, path, mirror, splice, edge_threshold, image_threshold, image_mask, use_tiles, tile_x, tile_y, channel")
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy, ValueEnum, Default)]
pub enum WalkPath {
    #[default]
    Horizontal,
    Vertical,
    Concentric,
    Diagonal
}

impl Display for WalkPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WalkPath::Horizontal => write!(f, "horizontal"),
            WalkPath::Vertical => write!(f, "vertical"),
            WalkPath::Concentric => write!(f, "concentric"),
            WalkPath::Diagonal => write!(f, "diagonal")
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ColorChannel {
    Red,
    Green,
    Blue
}

////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum SortingAlgorithm {
    Luma,
    Chroma,
    Saturation,
    Hue,
    Brightness
}

////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct AnimateParams {
    pub param: ArgumentList,
    pub start: u64,
    pub stop: u64,
    pub step: u64
}

fn into_animate_params(value: &str) -> anyhow::Result<AnimateParams> {
    let mut divided = value.split_whitespace();

    let param = divided
        .next()
        .ok_or_else(|| anyhow!("'param' is missing"))?
        .try_into()?;
    let start = divided
        .next()
        .ok_or_else(|| anyhow!("'start' is missing"))?
        .parse::<u64>()
        .with_context(|| "failed to parse 'start' to a number")?;
    let stop = divided
        .next()
        .ok_or_else(|| anyhow!("'stop' is missing"))?
        .parse::<u64>()
        .with_context(|| "failed to parse 'stop' to a number")?;
    let step = divided
        .next()
        .ok_or_else(|| anyhow!("'step' is missing"))?
        .parse::<u64>()
        .with_context(|| "failed to parse 'step' to a number")?;

    Ok(AnimateParams { param, start, stop, step })
}

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about, arg_required_else_help = true)]
pub struct Cli {
    /// Input file
    pub input: PathBuf,

    /// Output file
    pub output: PathBuf,

    /// Use a predefined sorting algorithm
    #[arg(long = "by")]
    pub by: SortingAlgorithm,

    /// interval
    #[arg(short = 'i', long = "interval", default_value_t = 1)]
    pub interval: usize,

    /// Sort in reverse
    #[arg(short = 'r', long = "reverse", default_value_t = false)]
    pub reverse: bool,

    #[arg(short = 'd', long = "discretize")]
    pub discretize: Option<u64>,

    #[arg(long = "direction", default_value_t = WalkPath::default())]
    pub direction: WalkPath,

    #[arg(short = 'm', long = "mirror")]
    pub mirror: Option<f64>,

    #[arg(short = 's', long = "splice")]
    pub splice: Option<f64>,

    #[arg(short = 'e', long = "edge-threshold")]
    pub edge_threshold: Option<u64>,

    #[arg(long = "image-threshold")]
    pub image_threshold: Option<u64>,

    #[arg(long = "image-mask")]
    pub image_mask: Option<PathBuf>,

    #[arg(short = 'c', long = "channel")]
    pub channel: Option<ColorChannel>,

    /// Parameters for animation.
    /// PARAM must be one of:
    /// interval, reverse, discretize, direction,
    /// mirror, splice, edge_threshold, image_threshold,
    /// image_mask, channel
    /// and START STOP STEP must be positive integers
    #[arg(
        long = "animate",
        value_parser(into_animate_params),
        name = "PARAM START STOP STEP",
        verbatim_doc_comment
    )]
    pub animate: Option<AnimateParams>
}
