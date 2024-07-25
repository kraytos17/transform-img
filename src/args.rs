use clap::{Args as ClapArgs, Parser, Subcommand};

use crate::{
    conversion,
    ppm::{ASCII_PPM, BIN_PPM},
};

/// Command line args for image conversion
#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Convert from one image type to another
    Convert(ConvertArgs),
}

#[derive(ClapArgs, Debug)]
pub struct ConvertArgs {
    /// The input file
    #[clap(short, long)]
    pub input: String,
    /// The output file
    #[clap(short, long)]
    pub output: String,
    /// Format of some image types (mostly required by ppm image headers (P3 / P6))
    #[clap(short, long)]
    pub format: Option<String>,
}

pub fn handle_conversion(input: &str, output: &str, format: Option<&str>) {
    let format = format.unwrap_or_default();
    let input_ext = input.split('.').last().unwrap_or_default();
    let output_ext = output.split('.').last().unwrap_or_default();

    match (input_ext, output_ext) {
        ("ppm", "png") => conversion::ppm_to_png(input, output).unwrap(),
        ("png", "ppm") => handle_png_to_ppm(input, output, &format),
        ("ppm", "jpg") | ("ppm", "jpeg") => conversion::ppm_to_jpeg(input, output).unwrap(),
        ("jpg", "ppm") | ("jpeg", "ppm") => conversion::jpeg_to_ppm(input, output).unwrap(),
        // ("png", "jpg") | ("png", "jpeg") => conversion::png_to_jpeg(input, output).unwrap(),
        // ("jpg", "png") | ("jpeg", "png") => conversion::jpeg_to_png(input, output).unwrap(),
        _ => {
            eprintln!("Unsupported conversion direction or format.");
            std::process::exit(1);
        }
    }
}

fn handle_png_to_ppm(input: &str, output: &str, format: &str) {
    let bin = validate_ppm_format(format);
    conversion::png_to_ppm(input, output, bin).unwrap();
}

fn validate_ppm_format(format: &str) -> bool {
    match format {
        ASCII_PPM => false,
        BIN_PPM => true,
        _ => {
            eprintln!("Invalid format for PPM. Use P3 for ASCII or P6 for binary.");
            std::process::exit(1);
        }
    }
}
