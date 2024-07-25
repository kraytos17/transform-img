mod conversion;
mod ppm;

use clap::{Args as ClapArgs, Parser, Subcommand};
use std::io;

/// Command line args for image conversion
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Convert from one image type to another
    Convert(ConvertArgs),
}

#[derive(ClapArgs, Debug)]
struct ConvertArgs {
    /// The input file
    #[clap(short, long)]
    input: String,
    /// The output file
    #[clap(short, long)]
    output: String,
    /// Format of some image types (mostly required by ppm image headers (P3 / P6))
    #[clap(short, long)]
    format: Option<String>,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    match args.command {
        Commands::Convert(convert_args) => {
            let input = convert_args.input;
            let output = convert_args.output;
            let format = convert_args.format.unwrap_or_default();

            let input_ext = input.split('.').last().unwrap_or_default();
            let output_ext = output.split('.').last().unwrap_or_default();

            match (input_ext, output_ext, format.as_str()) {
                ("ppm", "png", _) => conversion::ppm_to_png(&input, &output).unwrap(),
                ("png", "ppm", fmt) => {
                    let bin = match fmt {
                        "P3" => false,
                        "P6" => true,
                        _ => {
                            eprintln!("Invalid format for PPM. Use P3 for ASCII or P6 for binary.");
                            std::process::exit(1);
                        }
                    };
                    conversion::png_to_ppm(&input, &output, bin).unwrap();
                }
                ("ppm", "jpg", _) | ("ppm", "jpeg", _) => {
                    conversion::ppm_to_jpeg(&input, &output).unwrap()
                }
                _ => {
                    eprintln!("Unsupported conversion direction or format.");
                    std::process::exit(1);
                }
            }
        }
    }

    Ok(())
}
