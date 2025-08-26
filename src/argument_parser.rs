use std::{
    env::args,
    error::Error,
    fmt::{Debug, Display},
};

pub struct ArgumentParser {
    pub image_path: String,
    // list of characters that will be used to convert the image, going from darkest (left) to brightest pixel range value
    pub character_palette: Vec<char>,
}

impl ArgumentParser {
    pub fn new() -> Result<Self, ImagePathError> {
        let mut image_path: Option<String> = None;

        let arguments: Vec<String> = args().collect();
        let mut previous_argument = String::new();

        for argument in arguments {
            match previous_argument.as_str() {
                "--image" => image_path = Some(argument.clone()),
                "-i" => image_path = Some(argument.clone()),
                _ => {}
            }
            previous_argument = argument
        }

        return match image_path {
            Some(path) => Ok(Self {
                image_path: path,
                character_palette: vec![' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'],
            }),
            None => Err(ImagePathError),
        };
    }
}

pub struct ImagePathError;

impl ImagePathError {
    fn write(formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "No image path was supplied, please provide a path to an image with either:\n -i {{image_path}} or --image {{image_path}}"
        )
    }
}

impl Error for ImagePathError {}

impl Display for ImagePathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Self::write(f)
    }
}

impl Debug for ImagePathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Self::write(f)
    }
}
