use image::{DynamicImage, GenericImageView, Rgb, RgbImage};
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};

#[derive(Debug)]
struct PpmHeader {
    magic_number: String,
    width: u32,
    height: u32,
    _max_color_val: u32,
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
        _max_color_val: max_color_value
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing max color value"))?,
    })
}

fn parse_ppm<R: BufRead>(header: &PpmHeader, reader: &mut R) -> io::Result<Vec<u8>> {
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

fn write_ppm_header<W: Write>(
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

fn write_ppm_data<W: Write>(writer: &mut W, img: &DynamicImage, bin: bool) -> io::Result<()> {
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

fn png_to_ppm(png_file: &str, ppm_file: &str, bin: bool) -> Result<(), image::ImageError> {
    let input = image::open(png_file)?;
    let max_color_val: u32 = 255;
    let output = File::create(ppm_file)?;
    let mut writer = BufWriter::new(output);

    write_ppm_header(&mut writer, &input, max_color_val, bin)?;
    write_ppm_data(&mut writer, &input, bin)?;

    Ok(())
}

fn main() -> io::Result<()> {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 4 {
        eprintln!("Usage: {} <input> <output> <format>", args[0]);
        std::process::exit(1);
    }

    let input = &args[1];
    let output = &args[2];
    let format = &args[3];

    match (
        input.ends_with(".ppm"),
        output.ends_with(".png"),
        format.as_str(),
    ) {
        (true, true, _) => ppm_to_png(input, output)?,
        (true, false, _) | (false, true, _) => {
            eprintln!("Unsupported conversion direction.");
            std::process::exit(1);
        }
        (false, false, fmt) => {
            let bin = match fmt {
                ASCII_PPM => false,
                BIN_PPM => true,
                _ => {
                    eprintln!("Invalid format. Use P3 for ASCII or P6 for binary.");
                    std::process::exit(1);
                }
            };
            png_to_ppm(input, output, bin).unwrap();
        }
    }

    Ok(())
}
