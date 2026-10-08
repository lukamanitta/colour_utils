use crate::errors::InvalidColourFormat;
use crate::formats::RGBFormat;
use crate::Colour;

fn clamp(value: f32, min: f32, max: f32) -> f32 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

pub fn multiply_brightness(
    colour: &Colour,
    multiplier: f32,
) -> Result<Colour, InvalidColourFormat> {
    Colour::new_from_hsv(
        *colour.hsv().h(),
        *colour.hsv().s(),
        clamp(*colour.hsv().v() * multiplier, 0.0, 100.0),
    )
}

pub fn lighten(colour: &Colour, multiplier: f32) -> Result<Colour, InvalidColourFormat> {
    multiply_brightness(colour, multiplier)
}

pub fn darken(colour: &Colour, multiplier: f32) -> Result<Colour, InvalidColourFormat> {
    multiply_brightness(colour, 1.0 / multiplier)
}

pub fn invert(colour: &Colour) -> Result<Colour, InvalidColourFormat> {
    Colour::new_from_rgb(
        255 - *colour.rgb().r(),
        255 - *colour.rgb().g(),
        255 - *colour.rgb().b(),
    )
}

pub fn invert_brightness(colour: &Colour) -> Result<Colour, InvalidColourFormat> {
    Colour::new_from_hsv(
        *colour.hsv().h(),
        *colour.hsv().s(),
        100.0 - *colour.hsv().v(),
    )
}

pub fn blend(
    colour1: &Colour,
    colour2: &Colour,
    ratio: f32,
) -> Result<Colour, InvalidColourFormat> {
    Colour::new_from_rgb(
        (*colour1.rgb().r() as f32 * (1.0 - ratio) + *colour2.rgb().r() as f32 * ratio).round()
            as u8,
        (*colour1.rgb().g() as f32 * (1.0 - ratio) + *colour2.rgb().g() as f32 * ratio).round()
            as u8,
        (*colour1.rgb().b() as f32 * (1.0 - ratio) + *colour2.rgb().b() as f32 * ratio).round()
            as u8,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn it_works() {}

    #[test]
    fn invert_brightness_inverts_v_within_range() {
        let white = Colour::new("#FFFFFF").unwrap();
        let black = invert_brightness(&white).unwrap();
        assert_eq!(black.hex().to_string(), "000000");

        let black_again = invert_brightness(&black).unwrap();
        assert_eq!(black_again.hex().to_string(), "ffffff");
    }

    #[test]
    fn invert_brightness_keeps_hue_and_saturation() {
        let colour = Colour::new("#FF0000").unwrap();
        let inverted = invert_brightness(&colour).unwrap();
        // Hue/saturation preserved, value flipped (100 -> 0).
        assert_eq!(inverted.hsv().h().to_string(), "0");
        assert_eq!(inverted.hsv().s().to_string(), "100");
    }

    #[test]
    fn invert_returns_complement() {
        let colour = Colour::new("#FF0000").unwrap();
        assert_eq!(invert(&colour).unwrap().hex().to_string(), "00ffff");
    }
}
