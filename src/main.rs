mod args;
mod conversion;
mod ppm;

use args::{Args, Commands};
use clap::Parser;
use std::io;

fn main() -> io::Result<()> {
    let args = Args::parse();
    match args.command {
        Commands::Convert(convert_args) => {
            args::handle_conversion(
                &convert_args.input,
                &convert_args.output,
                convert_args.format.as_deref(),
            );
        }
    }

    Ok(())
}
