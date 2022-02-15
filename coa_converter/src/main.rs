use clap::{App, Arg};
use coa_converter_lib as coa;
use image::io::Reader as ImageReader;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::str::FromStr;

fn main() {
    let matches = App::new("Coa generator")
        .author("Perondas <Pperondas@gmail.com>")
        .version("0.1.0")
        .about("Generates a coat of arms for ck3 from images")
        .arg(
            Arg::new("source")
                .short('s')
                .takes_value(true)
                .value_name("SOURCE FILE")
                .help("Path to the source image")
                .required(true),
        )
        .arg(
            Arg::new("destination")
                .short('d')
                .takes_value(true)
                .value_name("DESTINATION PATH")
                .help("Path to the destination file"),
        )
        .arg(
            Arg::new("resolution")
                .short('r')
                .takes_value(true)
                .value_name("RESOLUTION")
                .help("Sets the resolution")
                .default_value("100"),
        )
        .arg(
            Arg::new("color_mode")
                .short('c')
                .takes_value(true)
                .value_name("MODE")
                .help("Sets the desired color mode. Available: Full, Reduced, Vanilla")
                .default_value("Reduced"),
        )
        .arg(
            Arg::new("color_count")
                .short('k')
                .takes_value(true)
                .value_name("COUNT")
                .help("Sets the color limit if using the reduced mode")
                .default_value("20"),
        )
        .arg(
            Arg::new("is_title")
                .short('t')
                .help("Indicates if the coa is for a landed title or not"),
        )
        .get_matches();

    let source;
    let mut destination;
    let resolution;
    let mode;
    let color_count;

    let is_title = matches.is_present("is_title");

    source = matches.value_of("source").unwrap();

    let s_path = Path::new(source);

    if !s_path.exists() || !s_path.is_file() {
        println!("Could not find {}", source);
        return;
    }

    match matches.value_of("destination") {
        Some(d) => destination = String::from(d),
        None => match s_path.extension() {
            Some(ex) => {
                destination = String::from(s_path.to_str().unwrap());
                for _ in 0..ex.len() {
                    destination.pop();
                }
                destination.push_str("txt");
            }
            None => {
                destination = String::from(s_path.to_str().unwrap());
                destination.push_str(".txt");
            }
        },
    };

    resolution = match matches.value_of("resolution") {
        Some(s) => match s.parse::<u32>() {
            Ok(v) => v,
            Err(m) => {
                println!("Could not parse resolution");
                println!("{}", m);
                return;
            }
        },
        None => 100,
    };

    mode = match matches.value_of("color_mode") {
        Some(s) => match ColorMode::from_str(s) {
            Ok(cm) => cm,
            Err(e) => {
                println!("Could not parse color mode");
                println!("{:?}", e);
                return;
            }
        },
        None => ColorMode::Reduced,
    };

    color_count = match matches.value_of("color_count") {
        Some(s) => match s.parse::<u8>() {
            Ok(v) => v,
            Err(m) => {
                println!("Could not parse color count");
                println!("{}", m);
                return;
            }
        },
        None => 20,
    };

    let img = match ImageReader::open(s_path) {
        Ok(i) => match i.decode() {
            Ok(img) => img,
            Err(e) => {
                println!("Failed to decode file");
                println!("Error: {}", e);
                return;
            }
        },
        Err(e) => {
            println!("Failed to open file");
            println!("Error: {}", e);
            return;
        }
    };

    let img = img.resize(resolution, resolution, image::imageops::FilterType::Nearest);

    let text = match mode {
        ColorMode::All => coa::from_image_all_colors(img, is_title),
        ColorMode::Reduced => coa::from_image_custom_colors(img, is_title, color_count),
        ColorMode::Vanilla => coa::from_image_vanilla_colors(img, is_title),
    };

    let mut output = match File::create(destination.clone()) {
        Ok(f) => f,
        Err(e) => {
            println!("Failed to create output file");
            println!("Reason {}", e);
            return;
        }
    };

    if let Err(e) = write!(output, "{}", text) {
        println!("Failed to write to file");
        println!("Reason: {}", e);
        return;
    }

    println!("Created coa at {}", destination);
}

#[derive(Debug, PartialEq)]
enum ColorMode {
    All,
    Reduced,
    Vanilla,
}

impl FromStr for ColorMode {
    type Err = ();

    fn from_str(input: &str) -> Result<ColorMode, Self::Err> {
        match input {
            "All" => Ok(ColorMode::All),
            "Reduced" => Ok(ColorMode::Reduced),
            "Vanilla" => Ok(ColorMode::Vanilla),
            _ => Err(()),
        }
    }
}
