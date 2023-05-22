use anyhow::{anyhow, bail, Context};
use clap::ValueEnum;
use image::Rgb;
use std::{path::PathBuf, fmt::Display};

use crate::{
    extractor::{luma, chroma, saturation, hue, brightness},
    sort::SortOptions
};

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ArgumentList {
    Interval,
    Discretize,
    Direction,
    Splice,
    EdgeThreshold,
    ImageThreshold,
    ImageMask,
    Channel
}

impl TryFrom<&str> for ArgumentList {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> anyhow::Result<Self> {
        match value.to_lowercase().as_str() {
            "interval" => Ok(Self::Interval),
            "discretize" => Ok(Self::Discretize),
            "direction" => Ok(Self::Direction),
            "splice" => Ok(Self::Splice),
            "edge_threshold" => Ok(Self::EdgeThreshold),
            "image_threshold" => Ok(Self::ImageThreshold),
            "image_mask" => Ok(Self::ImageMask),
            "channel" => Ok(Self::Channel),
            _ => bail!("'param' has to be one of: interval, discretize, direction, splice, edge_threshold, image_threshold, image_mask, channel")
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy, ValueEnum, Default)]
pub enum WalkPath {
    #[default]
    Horizontal,
    Vertical
}

impl Display for WalkPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WalkPath::Horizontal => write!(f, "horizontal"),
            WalkPath::Vertical => write!(f, "vertical")
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

impl SortingAlgorithm {
    pub fn into_rgb_sorter(&self) -> impl Fn(&Rgb<u8>, &SortOptions) -> u8 + Copy {
        match self {
            SortingAlgorithm::Luma => luma,
            SortingAlgorithm::Chroma => chroma,
            SortingAlgorithm::Saturation => saturation,
            SortingAlgorithm::Hue => hue,
            SortingAlgorithm::Brightness => brightness
        }
    }
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

////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coefficients {
    pub red: f32,
    pub green: f32,
    pub blue: f32
}

impl Coefficients {
    /// Default luma coefficients
    pub fn luma() -> Self {
        Self { red: 0.2126, green: 0.7152, blue: 0.0722 }
    }

    /// Default hue coefficients
    pub fn hue() -> Self {
        Self { red: 0.0, green: 2.0, blue: 4.0 }
    }

    /// Default saturation coefficients
    pub fn saturation() -> Self {
        Self { red: 0.0, green: 0.0, blue: 0.0 }
    }

    /// Default hue coefficients
    pub fn chroma() -> Self {
        Self { red: 0.0, green: 0.0, blue: 0.0 }
    }

    /// Default hue coefficients
    pub fn brightness() -> Self {
        Self { red: 0.0, green: 0.0, blue: 0.0 }
    }
}

impl Display for Coefficients {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.red, self.green, self.blue)
    }
}

impl Default for Coefficients {
    fn default() -> Self {
        Self { red: Default::default(), green: Default::default(), blue: Default::default() }
    }
}

impl From<&Cli> for Coefficients {
    fn from(value: &Cli) -> Self {
        match value.coefficients {
            Some(coefficients) => coefficients,
            None => match value.by {
                SortingAlgorithm::Luma => Coefficients::luma(),
                SortingAlgorithm::Chroma => Coefficients::chroma(),
                SortingAlgorithm::Saturation => Coefficients::saturation(),
                SortingAlgorithm::Hue => Coefficients::hue(),
                SortingAlgorithm::Brightness => Coefficients::brightness()
            }
        }
    }
}

/// Parse the input string into [`Coefficients`]
pub fn coefficients_value_parser(input: &str) -> anyhow::Result<Coefficients> {
    let mut coefficients = Coefficients::default();

    for value in input.split_whitespace() {
        match value.split_once('=') {
            Some((color, value)) => match color {
                "red" => coefficients.red = value.parse()?,
                "green" => coefficients.green = value.parse()?,
                "blue" => coefficients.blue = value.parse()?,
                name => bail!("invalid name: {}. has to be one of: red, green, blue", name)
            },
            None => bail!("invalid format: has to be 'color=value'")
        }
    }

    Ok(coefficients)
}

////////////////////////////////////////////////////////////////////////////////////////////////////////

/// Don't allow zero values (for interval)
fn no_negative_values(input: &str) -> anyhow::Result<usize> {
    match input.parse::<usize>() {
        Ok(value) if value != 0 => Ok(value),
        Err(error) => bail!(error),
        _ => bail!("interval cannot be less than 1")
    }
}

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about, arg_required_else_help = true)]
pub struct Cli {
    /// Use a predefined sorting algorithm
    #[clap(name = "EXTRACTOR")]
    pub by: SortingAlgorithm,

    /// Input file
    pub input: PathBuf,

    /// Output file
    pub output: PathBuf,

    /// Interval to sort pixels by
    #[arg(short = 'i', long = "interval", default_value_t = 1, value_parser(no_negative_values))]
    pub interval: usize,

    /// Sort in reverse
    #[arg(short = 'r', long = "reverse", default_value_t = false)]
    pub reverse: bool,

    /// red green blue coefficients for sorting pixels by luma.
    #[arg(short = 'f', long = "coefficients", value_parser(coefficients_value_parser))]
    pub coefficients: Option<Coefficients>,

    #[arg(short = 'd', long = "discretize", default_value_t = 1)]
    pub discretize: u64,

    /// The direction to sort pixels by
    #[arg(long = "direction", default_value_t = WalkPath::default())]
    pub direction: WalkPath,

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

    /// Passing shuffle will result in shuffling the red green blue values
    #[arg(long = "shuffle", default_value_t = false)]
    pub shuffle: bool,

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
