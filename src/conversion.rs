use image::codecs::jpeg::JpegEncoder;
use image::{GenericImageView, Rgb, RgbImage};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

use crate::ppm;

pub fn ppm_to_png<P: AsRef<Path>>(ppm_file: P, png_file: P) -> Result<(), image::ImageError> {
    let file = File::open(ppm_file)?;
    let mut reader = BufReader::new(file);
    let header = ppm::read_ppm_header(&mut reader)?;
    let data = ppm::parse_ppm(&header, &mut reader)?;

    let mut rgb_img = RgbImage::new(header.width, header.height);
    for (i, chunk) in data.chunks(3).enumerate() {
        let x = (i as u32 % header.width) as u32;
        let y = (i as u32 / header.width) as u32;
        rgb_img.put_pixel(x, y, Rgb([chunk[0], chunk[1], chunk[2]]));
    }

    rgb_img.save(png_file)?;

    Ok(())
}

pub fn png_to_ppm<P: AsRef<Path>>(
    png_file: P,
    ppm_file: P,
    bin: bool,
) -> Result<(), image::ImageError> {
    let input = image::open(png_file)?;
    let max_color_val: u32 = 255;
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
