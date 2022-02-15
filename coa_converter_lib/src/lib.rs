use image::{DynamicImage, GenericImageView};
use multimap::MultiMap;
use rscolorq::{self, Matrix2d, Params};
use std::fmt;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

/// The root coat of arms struct
pub struct Coa {
    pub pattern: Pattern,
    pub colors: Vec<Rgb>,
    pub emblems: Vec<Emblem>,
    pub is_title: bool,
}

/// The emblems used in a coa
pub struct Emblem {
    pub colors: Vec<Rgb>,
    pub texture: Texture,
    pub instances: Vec<Instance>,
}

/// The individual instances of a emblem
pub struct Instance {
    pub position: Option<(f32, f32)>,
    pub scale: Option<(f32, f32)>,
    pub depth: Option<f32>,
    pub rotation: Option<u32>,
}

/// The possible patterns in a coat of arms
pub enum Pattern {
    Solid,
}

/// The possible textures for a emblem
pub enum Texture {
    Block,
}

/// The vanilla coa designer colors
#[derive(EnumIter, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    Red,
    Orange,
    Yellow,
    YellowLight,
    White,
    Grey,
    Black,
    Brown,
    Green,
    GreenLight,
    BlueLight,
    Blue,
    Purple,
}

/// Represents a rgb u8 value
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Rgb {
    red: u8,
    green: u8,
    blue: u8,
}

impl Rgb {
    fn new(r: u8, g: u8, b: u8) -> Self {
        Rgb {
            red: r,
            green: g,
            blue: b,
        }
    }

    /// Calculates the 3d distance between two rgb values
    fn get_distance(&self, other: &Rgb) -> f32 {
        (((self.red as f32 - other.red as f32) * (self.red as f32 - other.red as f32)
            + (self.green as f32 - other.green as f32) * (self.green as f32 - other.green as f32)
            + (self.blue as f32 - other.blue as f32) * (self.blue as f32 - other.blue as f32))
            as f32)
            .sqrt()
    }
}

impl Color {
    /// Returns the rgb codes of the vanilla colors
    fn get_rgb(&self) -> Rgb {
        match self {
            Color::Red => Rgb::new(112, 33, 22),
            Color::Orange => Rgb::new(150, 57, 1),
            Color::Yellow => Rgb::new(187, 131, 47),
            Color::YellowLight => Rgb::new(150, 70, 50),
            Color::White => Rgb::new(200, 197, 195),
            Color::Grey => Rgb::new(125, 125, 125),
            Color::Black => Rgb::new(25, 22, 19),
            Color::Brown => Rgb::new(112, 58, 29),
            Color::Green => Rgb::new(31, 75, 35),
            Color::GreenLight => Rgb::new(50, 100, 55),
            Color::BlueLight => Rgb::new(42, 91, 137),
            Color::Blue => Rgb::new(20, 61, 100),
            Color::Purple => Rgb::new(87, 26, 63),
        }
    }

    /// Gets the closes matching color from the vanilla colors
    fn get_closest_match(sample: Rgb) -> Self {
        let v = Color::iter()
            .map(|c| (c, c.get_rgb().get_distance(&sample)))
            .min_by(|(_, dist1), (_, dist2)| dist1.partial_cmp(dist2).unwrap());
        v.unwrap().0
    }
}

/// Generates a coa with a limited color palette
pub fn from_image_custom_colors(img: DynamicImage, title: bool, color_count: u8) -> String {
    let width = img.width();
    let height = img.height();

    let mut imgbuf = Vec::with_capacity((width * height * 3).try_into().unwrap());
    let mut quantized_image = Matrix2d::new(width as usize, height as usize);

    // Build the quantization parameters
    let mut conditions = Params::new();
    conditions.palette_size(color_count);
    conditions.verify_parameters().unwrap();

    // Convert the input image buffer from Rgb<u8> to Rgb<f64>

    let image = Matrix2d::from_vec(
        img.pixels()
            .into_iter()
            .map(|(_, _, c)| rscolorq::color::Rgb {
                red: c[0] as f64 / 255.0,
                green: c[1] as f64 / 255.0,
                blue: c[2] as f64 / 255.0,
            })
            .collect(),
        width as usize,
        height as usize,
    );

    let mut palette = Vec::with_capacity(color_count as usize);

    // Reduce the colors
    rscolorq::spatial_color_quant(&image, &mut quantized_image, &mut palette, &conditions).unwrap();

    // Convert the Rgb<f64> palette to Rgb<u8>
    let palette = palette
        .iter()
        .map(|&c| {
            let color = 255.0 * c;
            [
                color.red.round() as u8,
                color.green.round() as u8,
                color.blue.round() as u8,
            ]
        })
        .collect::<Vec<[u8; 3]>>();

    // Create the final image by color lookup from the palette
    for &c in quantized_image.iter() {
        let color = palette
            .get(c as usize)
            .ok_or("Could not retrieve color from palette")
            .unwrap();
        imgbuf.extend_from_slice(color);
    }

    // Map all the pixels based on their colors
    let mut colored_blocks = MultiMap::new();
    for (p, (x, y, _)) in imgbuf.chunks(3).zip(img.pixels().into_iter()) {
        let color = Rgb::new(p[0], p[1], p[2]);
        colored_blocks.insert(color, (x, y));
    }

    parse_map(&mut colored_blocks, title, width)
}

/// Generates a coa with a full color palette
pub fn from_image_all_colors(img: DynamicImage, title: bool) -> String {
    let width = img.width();

    let mut colored_blocks = MultiMap::new();
    for (x, y, p) in img.pixels().into_iter() {
        let color = Rgb::new(p[0], p[1], p[2]);
        colored_blocks.insert(color, (x, y));
    }
    parse_map(&mut colored_blocks, title, width)
}

/// Generates a coa with only the vanilla designer colors
pub fn from_image_vanilla_colors(img: DynamicImage, title: bool) -> String {
    let width = img.width();

    let mut colored_blocks = MultiMap::new();
    for (x, y, p) in img.pixels().into_iter() {
        let color = Rgb::new(p[0], p[1], p[2]);
        let closest_match = Color::get_closest_match(color);
        colored_blocks.insert(closest_match.get_rgb(), (x, y));
    }
    parse_map(&mut colored_blocks, title, width)
}

/// Parses the mapped pixels into a coa
fn parse_map(colored_blocks: &mut MultiMap<Rgb, (u32, u32)>, is_title: bool, width: u32) -> String {
    let mut max = 0;
    let mut color = Rgb::new(0, 0, 0);
    for key_color in colored_blocks.keys() {
        let size = colored_blocks.get_vec(key_color).unwrap().len();
        if size > max {
            max = size;
            color = *key_color;
        }
    }

    colored_blocks.remove(&color);

    let mut co = Coa {
        pattern: Pattern::Solid,
        colors: vec![color],
        emblems: Vec::new(),
        is_title,
    };

    for key_color in colored_blocks.keys() {
        let mut emblem = Emblem {
            colors: vec![*key_color],
            texture: Texture::Block,
            instances: Vec::new(),
        };

        let coords = colored_blocks.get_vec(key_color).unwrap();
        for (x, y) in coords {
            let instance = Instance {
                position: Some((*x as f32 / width as f32, *y as f32 / width as f32)),
                scale: Some((1.0 / width as f32, 1.0 / width as f32)),
                depth: None,
                rotation: None,
            };
            emblem.instances.push(instance);
        }
        co.emblems.push(emblem);
    }
    co.to_string()
}

impl fmt::Display for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ {r} {g} {b} }}",
            r = self.red,
            g = self.green,
            b = self.blue
        )
    }
}

impl fmt::Display for Coa {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = "coa".to_string();
        if self.is_title {
            result.push_str("coa_rd_title={\n");
        } else {
            result.push_str("_rd_dynasty_0000000000={\n");
        }

        result.push_str(&format!("\tpattern=\"{}\"\n", self.pattern));
        let mut count = 1;
        for color in &self.colors {
            result.push_str(&format!("\tcolor{n}={c}\n", n = count, c = color));
            count += 1;
        }
        for emblem in &self.emblems {
            result.push_str(&format!("{}", emblem));
        }
        result.push_str("}\n");
        write!(f, "{}", result)
    }
}

impl fmt::Display for Emblem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = "\tcolored_emblem={\n".to_string();
        let mut count = 1;
        for color in &self.colors {
            result.push_str(&format!("\t\tcolor{n}={c}\n", n = count, c = color));
            count += 1;
        }
        result.push_str(&format!("\t\ttexture=\"{}\"\n", self.texture));
        for instance in &self.instances {
            result.push_str(&format!("{}", instance));
        }
        result.push_str("\t}\n");
        write!(f, "{}", result)
    }
}

impl fmt::Display for Instance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = "\t\tinstance={\n".to_string();
        if let Some(position) = self.position {
            result.push_str(&format!(
                "\t\t\tposition={{ {x:.6} {y:.6} }}\n",
                x = position.0,
                y = position.1
            ));
        }
        if let Some(scale) = self.scale {
            result.push_str(&format!(
                "\t\t\tscale={{ {x:.6} {y:.6} }}\n",
                x = scale.0,
                y = scale.1
            ));
        };
        if let Some(depth) = self.depth {
            result.push_str(&format!("\t\t\tdepth={:.6}\n", depth));
        };
        if let Some(rotation) = self.rotation {
            result.push_str(&format!("\t\t\trotation={}\n", rotation));
        };
        result.push_str("\t\t}\n");
        write!(f, "{}", result)
    }
}

impl fmt::Display for Texture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Texture::Block => write!(f, "ce_block_02.dds"),
        }
    }
}

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Pattern::Solid => write!(f, "pattern_solid.dds"),
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::Red => write!(f, "red"),
            Color::Orange => write!(f, "orange"),
            Color::Yellow => write!(f, "yellow"),
            Color::YellowLight => write!(f, "yellow_light"),
            Color::White => write!(f, "white"),
            Color::Grey => write!(f, "grey"),
            Color::Black => write!(f, "black"),
            Color::Brown => write!(f, "brown"),
            Color::Green => write!(f, "green"),
            Color::GreenLight => write!(f, "green_light"),
            Color::BlueLight => write!(f, "blue_light"),
            Color::Blue => write!(f, "blue"),
            Color::Purple => write!(f, "purple"),
        }
    }
}
