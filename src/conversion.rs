use image::{codecs::jpeg::JpegEncoder, io::Reader, ImageError, RgbImage};
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;

use crate::ppm::{self, parse_ppm, read_ppm_header, BIN_PPM};

pub fn ppm_to_png<P: AsRef<Path>>(ppm_file: P, png_file: P) -> Result<(), ImageError> {
    let file = File::open(ppm_file)?;
    let mut reader = BufReader::new(file);
    let header = read_ppm_header(&mut reader)?;
    let data = parse_ppm(&header, &mut reader)?;
    let rgb_img = RgbImage::from_raw(header.width, header.height, data)
        .expect("unable to produce rgb image from ppm data");
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

pub fn ppm_to_jpeg<P: AsRef<Path>>(ppm_file: P, jpeg_file: P) -> Result<(), ImageError> {
    let file = File::open(ppm_file)?;
    let mut reader = BufReader::new(file);
    let header = read_ppm_header(&mut reader)?;
    let data = parse_ppm(&header, &mut reader)?;

    let mut rgb_img = RgbImage::new(header.width, header.height);
    rgb_img.copy_from_slice(&data);

    let output = File::create(jpeg_file)?;
    let mut encoder = JpegEncoder::new(output);
    encoder.encode(
        &rgb_img,
        header.width,
        header.height,
        image::ExtendedColorType::Rgb8,
    )?;

    Ok(())
}

pub fn jpeg_to_ppm<P: AsRef<Path>>(jpeg_file: P, ppm_file: P) -> Result<(), ImageError> {
    let input = Reader::open(jpeg_file)?.decode()?;
    let img_buf = input.to_rgb8();
    let output = File::create(ppm_file)?;
    let mut writer = BufWriter::new(output);

    writeln!(writer, "{BIN_PPM}")?;
    writeln!(writer, "{} {}", img_buf.width(), img_buf.height())?;
    writeln!(writer, "255")?;

    writer.write_all(&img_buf)?;

    Ok(())
}

pub fn png_to_jpeg<P: AsRef<Path>>(png_file: P, jpeg_file: P) -> Result<(), ImageError> {
    let input = image::open(png_file)?;
    let img_buf = input.to_rgb8();
    let output = File::create(jpeg_file)?;
    img_buf.write_to(&mut BufWriter::new(output), image::ImageFormat::Jpeg)?;

    Ok(())
}

pub fn jpeg_to_png<P: AsRef<Path>>(jpeg_file: P, png_file: P) -> Result<(), ImageError> {
    let input = image::open(jpeg_file)?;
    let img_buf = input.to_rgb8();
    let output = File::create(png_file)?;
    img_buf.write_to(&mut BufWriter::new(output), image::ImageFormat::Png)?;

    Ok(())
}
