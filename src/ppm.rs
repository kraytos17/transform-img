use image::{DynamicImage, GenericImageView};
use std::io::{self, BufRead, Write};

#[derive(Debug)]
pub struct PpmHeader {
    pub magic_number: String,
    pub width: u32,
    pub height: u32,
    pub _max_color_val: u32,
}

pub const ASCII_PPM: &str = "P3";
pub const BIN_PPM: &str = "P6";

pub fn read_ppm_header<R: BufRead>(reader: &mut R) -> io::Result<PpmHeader> {
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
        _max_color_val: max_color_value
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing max color value"))?,
    })
}

pub fn parse_ppm<R: BufRead>(header: &PpmHeader, reader: &mut R) -> io::Result<Vec<u8>> {
    let mut result = vec![];

    if header.magic_number == ASCII_PPM {
        for line in reader.lines() {
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

pub fn write_ppm_header<W: Write>(
    writer: &mut W,
    img: &DynamicImage,
    max_color_val: u32,
    bin: bool,
) -> io::Result<()> {
    let (w, h) = img.dimensions();
    writeln!(writer, "{}", if bin { BIN_PPM } else { ASCII_PPM })?;
    writeln!(writer, "{} {}", w, h)?;
    writeln!(writer, "{}", max_color_val)?;

    Ok(())
}

pub fn write_ppm_data<W: Write>(writer: &mut W, img: &DynamicImage, bin: bool) -> io::Result<()> {
    let pixels = img.to_rgb8();
    if bin {
        for px in pixels.pixels() {
            writer.write_all(&[px[0], px[1], px[2]])?;
        }
    } else {
        for px in pixels.pixels() {
            writeln!(writer, "{} {} {}", px[0], px[1], px[2])?;
        }
    }

    Ok(())
}
