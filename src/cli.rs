#![allow(unused_variables, unused_mut, unused_imports, dead_code)]

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about)]
pub struct Cli {
    #[arg(short = 'f', long = "file")]
    pub filename: String,
}
