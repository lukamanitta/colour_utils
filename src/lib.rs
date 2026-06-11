pub mod conversions;
pub mod errors;
pub mod formats;
pub mod operations;

use regex::Regex;

use errors::InvalidColourFormat;
use formats::{Alpha, Hex, RGBFormat, HSV, RGB, RGBA};

const RGB_REGEX_STRING: &str = r"^rgb\((\d{1,3}), (\d{1,3}), (\d{1,3})\)$";
const RGBA_REGEX_STRING: &str = r"^rgba\((\d{1,3}), (\d{1,3}), (\d{1,3}), (0|1|0?\.\d+)\)$";
const RGBA_U8_ALPHA_REGEX_STRING: &str = r"^rgba\((\d{1,3}), (\d{1,3}), (\d{1,3}), (\d{1,3})\)$";
const HSV_REGEX_STRING: &str = r"^hsv\((\d+(?:\.\d+)?), (\d+(?:\.\d+)?), (\d+(?:\.\d+)?)\)$";
const HEX_REGEX_STRING: &str = r"^#(\d|\w{6})$";
const HEX_ALPHA_REGEX_STRING: &str = r"^#(\d|\w{6})(\d|\w{2})$";

#[derive(Clone, Debug, PartialEq)]
pub struct Colour {
    rgb: RGB,
    rgba: RGBA,
    hsv: HSV,
    hex: Hex,
}

impl Colour {
    pub fn new(string: &str) -> Result<Colour, InvalidColourFormat> {
        let rgb_regex: Regex = Regex::new(RGB_REGEX_STRING).unwrap();
        let rgba_regex: Regex = Regex::new(RGBA_REGEX_STRING).unwrap();
        let rgba_u8_alpha_regex: Regex = Regex::new(RGBA_U8_ALPHA_REGEX_STRING).unwrap();
        let hsv_regex: Regex = Regex::new(HSV_REGEX_STRING).unwrap();
        let hex_regex: Regex = Regex::new(HEX_REGEX_STRING).unwrap();
        let hex_alpha_regex: Regex = Regex::new(HEX_ALPHA_REGEX_STRING).unwrap();

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

        if rgba_regex.is_match(&string) {
            let captures = rgba_regex.captures(&string).unwrap();

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
            let a = captures
                .get(4)
                .unwrap()
                .as_str()
                .parse::<f32>()
                .map_err(|_err| InvalidColourFormat::FormatNotRecognisedError)?;

            return Colour::new_from_rgba(r, g, b, Some(a), None);
        }

        if rgba_u8_alpha_regex.is_match(&string) {
            let captures = rgba_u8_alpha_regex.captures(&string).unwrap();

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
            let a_u8 = captures
                .get(4)
                .unwrap()
                .as_str()
                .parse::<u8>()
                .map_err(|_err| InvalidColourFormat::FormatNotRecognisedError)?;

            return Colour::new_from_rgba(r, g, b, None, Some(a_u8));
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

        if hex_alpha_regex.is_match(&string) {
            let captures = hex_alpha_regex.captures(&string).unwrap();
            let hex_part = captures.get(1).unwrap().as_str();
            let alpha_part = captures.get(2).unwrap().as_str();

            let colour_from_hex = Colour::new_from_hex(hex_part)?;

            let a_u8 = u8::from_str_radix(&alpha_part, 16)
                .map_err(|_err| InvalidColourFormat::FormatNotRecognisedError)?;

            return Colour::new_from_rgba(
                *colour_from_hex.rgb().r(),
                *colour_from_hex.rgb().g(),
                *colour_from_hex.rgb().b(),
                None,
                Some(a_u8),
            );
        }

        Err(InvalidColourFormat::FormatNotRecognisedError)
    }

    pub fn new_from_rgb(r: u8, g: u8, b: u8) -> Result<Colour, InvalidColourFormat> {
        let rgb = RGB::new(r, g, b)?;

        Ok(Colour {
            rgb: rgb.clone(),
            rgba: RGBA::from(rgb),
            hsv: conversions::rgb_to_hsv(&rgb)?,
            hex: conversions::rgb_to_hex(&rgb)?,
        })
    }

    pub fn new_from_rgba(
        r: u8,
        g: u8,
        b: u8,
        a: Option<f32>,
        a_u8: Option<u8>,
    ) -> Result<Colour, InvalidColourFormat> {
        let rgba = RGBA::new(r, g, b, a, a_u8)?;
        let rgb = RGB::from(rgba);

        Ok(Colour {
            rgb,
            rgba,
            hsv: conversions::rgb_to_hsv(&rgb)?,
            hex: conversions::rgb_to_hex(&rgb)?,
        })
    }

    pub fn new_from_hsv(h: f32, s: f32, v: f32) -> Result<Colour, InvalidColourFormat> {
        let hsv = HSV::new(h, s, v)?;
        let rgb = conversions::hsv_to_rgb(&hsv)?;

        Ok(Colour {
            rgb,
            rgba: RGBA::from(rgb),
            hsv: hsv.clone(),
            hex: conversions::hsv_to_hex(&hsv)?,
        })
    }

    pub fn new_from_hex(hex: &str) -> Result<Colour, InvalidColourFormat> {
        let hex = Hex::new(hex)?;
        let rgb = conversions::hex_to_rgb(&hex)?;

        Ok(Colour {
            rgb: conversions::hex_to_rgb(&hex)?,
            rgba: RGBA::from(rgb),
            hsv: conversions::hex_to_hsv(&hex)?,
            hex: hex.clone(),
        })
    }

    pub fn rgb(&self) -> &RGB {
        &self.rgb
    }

    pub fn rgba(&self) -> &RGBA {
        &self.rgba
    }

    pub fn hsv(&self) -> &HSV {
        &self.hsv
    }

    pub fn hex(&self) -> &Hex {
        &self.hex
    }

    pub fn alpha(&self) -> &f32 {
        self.rgba.a()
    }

    pub fn alpha_u8(&self) -> &u8 {
        self.rgba.a_u8()
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
    fn rgba_regex_works() {
        let rgba_regex: Regex = Regex::new(RGBA_REGEX_STRING).unwrap();
        assert!(rgba_regex.is_match("rgba(255, 255, 255, 0.5)"));
    }

    #[test]
    fn rgba_u8_alpha_regex_works() {
        let rgba_u8_alpha_regex: Regex = Regex::new(RGBA_U8_ALPHA_REGEX_STRING).unwrap();
        assert!(rgba_u8_alpha_regex.is_match("rgba(255, 255, 255, 128)"));
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
    fn hex_alpha_regex_works() {
        let hex_alpha_regex: Regex = Regex::new(HEX_ALPHA_REGEX_STRING).unwrap();
        assert!(hex_alpha_regex.is_match("#ffffffff"));
    }

    #[test]
    fn new_with_rgb_input_works() {
        let colour = Colour::new("rgb(255, 255, 255)").unwrap();
        let white_rgb = RGB::new(255, 255, 255).unwrap();
        assert_eq!(colour.rgb(), &white_rgb);
    }

    #[test]
    fn new_with_rgba_input_works() {
        let colour = Colour::new("rgba(255, 255, 255, 0.5)").unwrap();
        let white_rgba = RGBA::new(255, 255, 255, Some(0.5), None).unwrap();
        assert_eq!(colour.rgba(), &white_rgba);
    }

    #[test]
    fn new_with_rgba_u8_alpha_input_works() {
        let colour = Colour::new("rgba(255, 255, 255, 128)").unwrap();
        let white_rgba = RGBA::new(255, 255, 255, None, Some(128)).unwrap();
        assert_eq!(colour.rgba(), &white_rgba);
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

    #[test]
    fn new_with_hex_alpha_input_works() {
        let colour = Colour::new("#ffffffff").unwrap();
        let white_hex = Hex::new("#ffffff").unwrap();
        assert_eq!(colour.hex(), &white_hex);
        let white_rgba = RGBA::new(255, 255, 255, None, Some(255)).unwrap();
        assert_eq!(colour.rgba(), &white_rgba);
    }
}
