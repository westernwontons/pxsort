use std::path::PathBuf;

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about)]
pub struct Cli {
    #[arg(short = 'f', long = "file")]
    pub filename: PathBuf,
    #[arg(short = 'o', long = "output")]
    pub output: PathBuf,
}
