use image::codecs::jpeg::JpegEncoder;
use image::io::Reader;
use image::{GenericImageView, RgbImage};
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;

use crate::ppm::{self, parse_ppm, read_ppm_header, BIN_PPM};

pub fn ppm_to_png<P: AsRef<Path>>(ppm_file: P, png_file: P) -> Result<(), image::ImageError> {
    let file = File::open(ppm_file)?;
    let mut reader = BufReader::new(file);
    let header = read_ppm_header(&mut reader)?;
    let data = parse_ppm(&header, &mut reader)?;
    let mut rgb_img = RgbImage::new(header.width, header.height);
    rgb_img.copy_from_slice(&data);
    rgb_img.save(png_file)?;

    Ok(())
}

pub fn png_to_ppm<P: AsRef<Path>>(
    png_file: P,
    ppm_file: P,
    bin: bool,
) -> Result<(), image::ImageError> {
    let input = image::open(png_file)?;
    let max_color_val = 255;
    let output = File::create(ppm_file)?;
    let mut writer = BufWriter::new(output);
    ppm::write_ppm_header(&mut writer, &input, max_color_val, bin)?;
    ppm::write_ppm_data(&mut writer, &input, bin)?;

    Ok(())
}

pub fn ppm_to_jpeg<P: AsRef<Path>>(ppm_file: P, jpeg_file: P) -> Result<(), image::ImageError> {
    let input = image::open(ppm_file)?;
    let output = File::create(jpeg_file)?;
    let mut encoder = JpegEncoder::new(output);
    let (w, h) = input.dimensions();
    let img_buf = input.to_rgb8();
    encoder.encode(&img_buf, w, h, image::ExtendedColorType::Rgb8)?;

    Ok(())
}

pub fn jpeg_to_ppm<P: AsRef<Path>>(jpeg_file: P, ppm_file: P) -> Result<(), image::ImageError> {
    let input = Reader::open(jpeg_file)?.decode()?;
    let img_buf = input.to_rgb8();
    let output = File::create(ppm_file)?;
    let mut writer = BufWriter::new(output);

    writeln!(writer, "{BIN_PPM}")?;
    writeln!(writer, "{} {}", img_buf.width(), img_buf.height())?;
    writeln!(writer, "255")?;

    let data = img_buf.as_raw();
    writer.write_all(data)?;

    Ok(())
}

// fn png_to_jpeg<P: AsRef<Path>>(png_file: P, jpeg_file: P) -> Result<(), image::ImageError> {
//     let input = image::open(png_file)?;
//     let img_buf = input.to_rgb8();
//     let output = File::create(jpeg_file)?;
// }
