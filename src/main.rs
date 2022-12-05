mod cli;
use clap::Parser;
use cli::Cli;

fn main() {
    let cli = Cli::parse();
    let file = cli.filename;

    dbg!(file);
}
