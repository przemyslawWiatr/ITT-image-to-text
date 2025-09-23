use std::{error::Error, path::Path};

use colored::Colorize;
use image::{DynamicImage, ImageReader};

const LUMINENCE_R: f32 = 0.2126;
const LUMINENCE_G: f32 = 0.7152;
const LUMINENCE_B: f32 = 0.0722;

pub struct ImageConverter {
    image: DynamicImage,
    colored: bool,
}

impl ImageConverter {
    pub fn new<T: AsRef<Path>>(image_path: T, colored: bool) -> Result<Self, Box<dyn Error>> {
        let image = ImageReader::open(image_path)?
            .with_guessed_format()?
            .decode()?;

        Ok(Self { image, colored })
    }

    pub fn convert(&self, character_palette: Vec<char>) -> String {
        return if self.colored {
            self.convert_colored(character_palette)
        } else {
            self.convert_grayscaled(character_palette)
        };
    }

    fn convert_colored(&self, character_palette: Vec<char>) -> String {
        let prepared_image = self.image.to_rgb8();
        let image_width = prepared_image.width() as usize;
        let palette_length = character_palette.len();
        let palette_step = 1. / palette_length as f32;

        let mut converted_image = String::new();

        for (index, pixel) in prepared_image.pixels().enumerate() {
            let [red, green, blue] = pixel.0;
            let luma = red as f32 / 255. * LUMINENCE_R
                + green as f32 / 255. * LUMINENCE_G
                + blue as f32 / 255. * LUMINENCE_B;

            let character_index = (luma / palette_step)
                .round()
                .clamp(0., (palette_length - 1) as f32) as usize;

            let colored_string = String::from(character_palette[character_index].clone())
                .truecolor(red, green, blue);

            converted_image += format!("{colored_string}").as_str();

            if index % image_width == 0 {
                converted_image.push('\n');
            }
        }

        return converted_image;
    }

    fn convert_grayscaled(&self, character_palette: Vec<char>) -> String {
        let prepared_image = self.image.to_luma32f();
        let image_width = prepared_image.width() as usize;
        let palette_length = character_palette.len();
        let palette_step = 1. / palette_length as f32;

        let mut converted_image = String::new();

        for (index, pixel) in prepared_image.pixels().enumerate() {
            let luma = pixel.0[0];

            let character_index = (luma / palette_step)
                .round()
                .clamp(0., (palette_length - 1) as f32) as usize;

            converted_image.push(character_palette[character_index].clone());

            if index % image_width == 0 {
                converted_image.push('\n');
            }
        }

        return converted_image;
    }
}
