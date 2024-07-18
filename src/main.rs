use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

use image::{Rgb, RgbImage};

#[derive(Debug)]
#[allow(dead_code)]
struct PpmHeader {
    magic_number: String,
    width: u32,
    height: u32,
    max_color_value: u32,
}

const ASCII_PPM: &str = "P3";
const BIN_PPM: &str = "P6";

fn read_ppm_header<R: BufRead>(reader: &mut R) -> io::Result<PpmHeader> {
    let mut lines = reader.lines();
    let magic_number = lines
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing magic number"))??;

    let (mut width, mut height, mut max_color_value) = (None, None, None);

    for line in lines {
        let line = line?;
        let trimmed = line.trim();

        if trimmed.starts_with('#') {
            continue;
        }

        if width.is_none() && height.is_none() {
            let mut dims = trimmed.split_whitespace();
            if let (Some(w), Some(h)) = (dims.next(), dims.next()) {
                width =
                    Some(w.parse().map_err(|_| {
                        io::Error::new(io::ErrorKind::InvalidData, "Invalid width")
                    })?);
                height =
                    Some(h.parse().map_err(|_| {
                        io::Error::new(io::ErrorKind::InvalidData, "Invalid height")
                    })?);
                continue;
            }
        }

        if max_color_value.is_none() {
            max_color_value = Some(trimmed.parse().map_err(|_| {
                io::Error::new(io::ErrorKind::InvalidData, "Invalid max color value")
            })?);
            break;
        }
    }

    Ok(PpmHeader {
        magic_number,
        width: width.ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing width"))?,
        height: height
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing height"))?,
        max_color_value: max_color_value
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing max color value"))?,
    })
}

fn parse_ppm<R: Read + BufRead>(header: &PpmHeader, reader: &mut R) -> io::Result<Vec<u8>> {
    let mut result = vec![];

    if header.magic_number == ASCII_PPM {
        let lines = reader.lines();
        for line in lines {
            let line = line?;
            let trimmed_line = line.trim();
            if trimmed_line.starts_with('#') {
                continue;
            }
            for val in trimmed_line.split_whitespace() {
                let px_val = val.parse::<u8>().map_err(|_| {
                    io::Error::new(io::ErrorKind::InvalidData, "Invalid pixel value")
                })?;
                result.push(px_val);
            }
        }
    } else if header.magic_number == BIN_PPM {
        let size = (header.width * header.height * 3) as usize;
        result.resize(size, 0);
        reader.read_exact(&mut result)?;
    } else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Unsupported PPM format",
        ));
    }

    Ok(result)
}

fn ppm_to_png(ppm_file: &str, png_file: &str) -> io::Result<()> {
    let file = File::open(ppm_file)?;
    let mut reader = BufReader::new(file);
    let header = read_ppm_header(&mut reader)?;
    let data = parse_ppm(&header, &mut reader)?;
    let mut rgb_img = RgbImage::new(header.width, header.height);

    for (i, chunk) in data.chunks(3).enumerate() {
        let x = (i as u32 % header.width) as u32;
        let y = (i as u32 / header.width) as u32;
        rgb_img.put_pixel(x, y, Rgb([chunk[0], chunk[1], chunk[2]]));
    }

    rgb_img.save(png_file).unwrap();
    Ok(())
}

fn _write_ppm_header() {}

fn main() -> io::Result<()> {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 3 {
        eprintln!("Usage: {} <input.ppm> <output.png>", args[0]);
        std::process::exit(1);
    }
    ppm_to_png(&args[1], &args[2])?;
    Ok(())
}
