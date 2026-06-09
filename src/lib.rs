pub mod conversions;
pub mod errors;
pub mod formats;
pub mod operations;

use regex::Regex;

use errors::InvalidColourFormat;
use formats::{Hex, HSV, RGB};

const RGB_REGEX_STRING: &str = r"^rgb\((\d{1,3}), (\d{1,3}), (\d{1,3})\)$";
const HSV_REGEX_STRING: &str = r"^hsv\((\d+(?:\.\d+)?), (\d+(?:\.\d+)?), (\d+(?:\.\d+)?)\)$";
const HEX_REGEX_STRING: &str = r"^#(\d|\w){6}$";

#[derive(Clone)]
pub struct Colour {
    rgb: RGB,
    hsv: HSV,
    hex: Hex,
}

impl Colour {
    pub fn new(string: &str) -> Result<Colour, InvalidColourFormat> {
        let rgb_regex: Regex = Regex::new(RGB_REGEX_STRING).unwrap();
        let hsv_regex: Regex = Regex::new(HSV_REGEX_STRING).unwrap();
        let hex_regex: Regex = Regex::new(HEX_REGEX_STRING).unwrap();

        if rgb_regex.is_match(&string) {
            let captures = rgb_regex.captures(&string).unwrap();

            let r = captures
                .get(1)
                .unwrap()
                .as_str()
                .parse::<u8>()
                .map_err(|_err| InvalidColourFormat::FormatNotRecognisedError)?;
            let g = captures
                .get(2)
                .unwrap()
                .as_str()
                .parse::<u8>()
                .map_err(|_err| InvalidColourFormat::FormatNotRecognisedError)?;
            let b = captures
                .get(3)
                .unwrap()
                .as_str()
                .parse::<u8>()
                .map_err(|_err| InvalidColourFormat::FormatNotRecognisedError)?;
            return Colour::new_from_rgb(r, g, b);
        }

        if hsv_regex.is_match(&string) {
            let captures = hsv_regex.captures(&string).unwrap();

            let h = captures
                .get(1)
                .unwrap()
                .as_str()
                .parse::<f32>()
                .map_err(|_err| InvalidColourFormat::FormatNotRecognisedError)?;
            let s = captures
                .get(2)
                .unwrap()
                .as_str()
                .parse::<f32>()
                .map_err(|_err| InvalidColourFormat::FormatNotRecognisedError)?;
            let v = captures
                .get(3)
                .unwrap()
                .as_str()
                .parse::<f32>()
                .map_err(|_err| InvalidColourFormat::FormatNotRecognisedError)?;

            return Colour::new_from_hsv(h, s, v);
        }

        if hex_regex.is_match(&string) {
            return Colour::new_from_hex(&string);
        }

        Err(InvalidColourFormat::FormatNotRecognisedError)
    }

    pub fn new_from_rgb(r: u8, g: u8, b: u8) -> Result<Colour, InvalidColourFormat> {
        let rgb = RGB::new(r, g, b)?;

        Ok(Colour {
            rgb: rgb.clone(),
            hsv: conversions::rgb_to_hsv(&rgb)?,
            hex: conversions::rgb_to_hex(&rgb)?,
        })
    }

    pub fn new_from_hsv(h: f32, s: f32, v: f32) -> Result<Colour, InvalidColourFormat> {
        let hsv = HSV::new(h, s, v)?;

        Ok(Colour {
            rgb: conversions::hsv_to_rgb(&hsv)?,
            hsv: hsv.clone(),
            hex: conversions::hsv_to_hex(&hsv)?,
        })
    }

    pub fn new_from_hex(hex: &str) -> Result<Colour, InvalidColourFormat> {
        let hex = Hex::new(hex)?;

        Ok(Colour {
            rgb: conversions::hex_to_rgb(&hex)?,
            hsv: conversions::hex_to_hsv(&hex)?,
            hex: hex.clone(),
        })
    }

    pub fn rgb(&self) -> &RGB {
        &self.rgb
    }

    pub fn hsv(&self) -> &HSV {
        &self.hsv
    }

    pub fn hex(&self) -> &Hex {
        &self.hex
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rgb_regex_works() {
        let rgb_regex: Regex = Regex::new(RGB_REGEX_STRING).unwrap();
        assert!(rgb_regex.is_match("rgb(255, 255, 255)"));
    }

    #[test]
    fn rgb_regex_fails_correctly() {
        let rgb_regex: Regex = Regex::new(RGB_REGEX_STRING).unwrap();
        assert!(rgb_regex.is_match("rgb(255.0, -2, 9999)") == false);
    }

    #[test]
    fn hsv_regex_works() {
        let hsv_regex: Regex = Regex::new(HSV_REGEX_STRING).unwrap();
        assert!(hsv_regex.is_match("hsv(0.0, 0.0, 100.0)"));
    }

    #[test]
    fn hex_regex_works() {
        let hex_regex: Regex = Regex::new(HEX_REGEX_STRING).unwrap();
        assert!(hex_regex.is_match("#ffffff"));
    }

    #[test]
    fn new_with_rgb_input_works() {
        let colour = Colour::new("rgb(255, 255, 255)").unwrap();
        let white_rgb = RGB::new(255, 255, 255).unwrap();
        assert_eq!(colour.rgb(), &white_rgb);
    }

    #[test]
    fn new_with_hsv_input_works() {
        let colour = Colour::new("hsv(0.0, 0.0, 100.0)").unwrap();
        let white_hsv = HSV::new(0.0, 0.0, 100.0).unwrap();
        assert_eq!(colour.hsv(), &white_hsv);
    }

    #[test]
    fn new_with_hex_input_works() {
        let colour = Colour::new("#ffffff").unwrap();
        let white_hex = Hex::new("#ffffff").unwrap();
        assert_eq!(colour.hex(), &white_hex);
    }
}
