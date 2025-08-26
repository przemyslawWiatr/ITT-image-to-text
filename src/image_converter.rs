use std::{error::Error, path::Path};

use image::{DynamicImage, ImageReader};

pub struct ImageConverter {
    image: DynamicImage,
}

impl ImageConverter {
    pub fn new<T: AsRef<Path>>(image_path: T) -> Result<Self, Box<dyn Error>> {
        let image = ImageReader::open(image_path)?
            .with_guessed_format()?
            .decode()?;

        Ok(Self { image })
    }

    pub fn convert(&self, character_palette: Vec<char>) -> String {
        let prepared_image = self.image.grayscale().to_luma32f();
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
