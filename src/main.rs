use anyhow::Result;
use clap::Parser;
use nb::cli::{CLI, Commands};
use nb::handlers::handle_add;

fn main() -> Result<()> {
    let cli = CLI::parse();

    match cli.command {
        Commands::New(new) => handle_add(new),
        _ => todo!(),
    }
}
