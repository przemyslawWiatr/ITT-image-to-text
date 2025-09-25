use std::{error::Error, path::Path};

use colored::Colorize;
use image::{DynamicImage, ImageReader, Pixel};

pub struct ImageConverter {
    image: DynamicImage,
    colored: bool,
}

impl ImageConverter {
    pub fn new<T: AsRef<Path>>(
        image_path: T,
        colored: bool,
        width: u16,
        height: u16,
        scaling_type: ScalingType,
    ) -> Result<Self, Box<dyn Error>> {
        let mut image = ImageReader::open(image_path)?
            .with_guessed_format()?
            .decode()?;

        match scaling_type {
            ScalingType::PreserveRatio => {
                image = image.resize(
                    width as u32,
                    height as u32,
                    image::imageops::FilterType::Nearest,
                )
            }
            ScalingType::Stretch => {
                image = image.resize_exact(
                    width as u32,
                    height as u32,
                    image::imageops::FilterType::Nearest,
                )
            }
        }

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
        let image_height = prepared_image.height() as usize;
        let palette_length = character_palette.len();
        let palette_step = 1. / palette_length as f32;

        let mut converted_image = String::with_capacity((image_width * 23 + 1) * image_height); // + 1 is for '\n' at the end of each line, * 23 because each ANSI truecolor escape can take up to 23 bytes

        for (index, pixel) in prepared_image.pixels().enumerate() {
            let [red, green, blue] = pixel.0;
            let luma = pixel.to_luma().0[0] as f32 / 255.;

            let character_index = (luma / palette_step)
                .round()
                .clamp(0., (palette_length - 1) as f32) as usize;

            let colored_string = String::from(character_palette[character_index].clone())
                .truecolor(red, green, blue);

            converted_image += format!("{colored_string}").as_str();

            if (index + 1) % image_width == 0 {
                converted_image.push('\n');
            }
        }

        return converted_image;
    }

    fn convert_grayscaled(&self, character_palette: Vec<char>) -> String {
        let prepared_image = self.image.to_luma32f();
        let image_width = prepared_image.width() as usize;
        let image_height = prepared_image.height() as usize;
        let palette_length = character_palette.len();
        let palette_step = 1. / palette_length as f32;

        let mut converted_image = String::with_capacity((image_width * 4 + 1) * image_height); // + 1 is for '\n' at the end of each line, * 4 because each utf-8 character can take up to 4 bytes;

        for (index, pixel) in prepared_image.pixels().enumerate() {
            let luma = pixel.0[0];

            let character_index = (luma / palette_step)
                .round()
                .clamp(0., (palette_length - 1) as f32) as usize;

            converted_image.push(character_palette[character_index].clone());

            if (index + 1) % image_width == 0 {
                converted_image.push('\n');
            }
        }

        return converted_image;
    }
}

pub enum ScalingType {
    PreserveRatio,
    Stretch,
}
