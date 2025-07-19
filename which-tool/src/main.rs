use clap::Parser;
use crate::cli::Which;

mod cli;
mod commands;

fn main() {
    let args: Which = cli::Which::parse();

    commands::show::show(args.programs)
}
