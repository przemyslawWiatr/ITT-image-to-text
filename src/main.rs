use std::error::Error;

use crate::{argument_parser::ArgumentParser, image_converter::ImageConverter};

mod argument_parser;
mod image_converter;

fn main() -> Result<(), Box<dyn Error>> {
    let argument_parser = ArgumentParser::new()?;
    let image_converter = ImageConverter::new(argument_parser.image_path, argument_parser.colored)?;
    let converted_image = image_converter.convert(argument_parser.character_palette);

    println!("{converted_image}");

    return Ok(());
}
