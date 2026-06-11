use crate::errors::InvalidColourFormat;
use crate::formats::{Alpha, Hex, RGBFormat, HSV, RGB, RGBA};

pub fn rgb_to_hsv(rgb: &RGB) -> Result<HSV, InvalidColourFormat> {
    // Adapted from https://www.geeksforgeeks.org/program-change-rgb-color-model-hsv-color-model/
    let r: f32 = *rgb.r() as f32 / 255.0;
    let g: f32 = *rgb.g() as f32 / 255.0;
    let b: f32 = *rgb.b() as f32 / 255.0;

    let max: f32 = *[r, g, b].iter().max_by(|a, b| a.total_cmp(b)).unwrap();

    let min: f32 = *[r, g, b]
        .iter()
        .max_by(|a, b| a.total_cmp(b).reverse())
        .unwrap();

    let d: f32 = max - min;

    let h: f32;
    let s: f32;
    let v: f32;

    if max == min {
        h = 0.0; // achromatic
    } else if max == r {
        h = (60.0 * ((g - b) / d) + 360.0) % 360.0;
    } else if max == g {
        h = (60.0 * ((b - r) / d) + 120.0) % 360.0;
    } else if max == b {
        h = (60.0 * ((r - g) / d) + 240.0) % 360.0;
    } else {
        return Err(InvalidColourFormat::FailedConversionError);
    }

    if max == 0.0 {
        s = 0.0;
    } else {
        s = (d / max) * 100.0;
    }

    v = max * 100.0;

    HSV::new(h, s, v)
}

pub fn rgb_to_hex(rgb: &RGB) -> Result<Hex, InvalidColourFormat> {
    Hex::new(&format!("{:02x}{:02x}{:02x}", rgb.r(), rgb.g(), rgb.b()))
}

pub fn hsv_to_rgb(hsv: &HSV) -> Result<RGB, InvalidColourFormat> {
    // Adapted from https://www.rapidtables.com/convert/color/hsv-to-rgb.html

    let hp: f32 = hsv.h() / 60.0;
    let s: f32 = hsv.s() / 100.0;
    let v: f32 = hsv.v() / 100.0;

    let c: f32 = v * s;
    let x: f32 = c * (1.0 - (hp % 2.0 - 1.0).abs());
    let m: f32 = v - c;

    let (r_prime, g_prime, b_prime) = if (0.0..1.0).contains(&hp) {
        (c, x, 0.0)
    } else if (1.0..2.0).contains(&hp) {
        (x, c, 0.0)
    } else if (2.0..3.0).contains(&hp) {
        (0.0, c, x)
    } else if (3.0..4.0).contains(&hp) {
        (0.0, x, c)
    } else if (4.0..5.0).contains(&hp) {
        (x, 0.0, c)
    } else if (5.0..6.0).contains(&hp) {
        (c, 0.0, x)
    } else {
        return Err(InvalidColourFormat::FailedConversionError);
    };

    RGB::new(
        ((r_prime + m) * 255.0).round() as u8,
        ((g_prime + m) * 255.0).round() as u8,
        ((b_prime + m) * 255.0).round() as u8,
    )
}

pub fn hsv_to_hex(hsv: &HSV) -> Result<Hex, InvalidColourFormat> {
    rgb_to_hex(&hsv_to_rgb(hsv).unwrap()) // This nested conversion seems like the fastest way - rgb to
                                          // hex is trivial
}

pub fn hex_to_rgb(hex: &Hex) -> Result<RGB, InvalidColourFormat> {
    let r = u8::from_str_radix(hex.r(), 16).unwrap();
    let g = u8::from_str_radix(hex.g(), 16).unwrap();
    let b = u8::from_str_radix(hex.b(), 16).unwrap();
    RGB::new(r, g, b)
}

pub fn hex_to_hsv(hex: &Hex) -> Result<HSV, InvalidColourFormat> {
    rgb_to_hsv(&hex_to_rgb(hex).unwrap()) // This nested conversion seems like the fastest way - hex to
                                          // rgb is trivial
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rgb_format() -> RGB {
        RGB::new(255, 0, 255).unwrap()
    }

    fn hsv_format() -> HSV {
        HSV::new(300.0, 100.0, 100.0).unwrap()
    }

    fn hex_format() -> Hex {
        Hex::new("#ff00ff").unwrap()
    }

    #[test]
    fn rgb_to_hsv_works() {
        assert_eq!(rgb_to_hsv(&rgb_format()), Ok(hsv_format()));
    }

    #[test]
    fn rgb_to_hex_works() {
        assert_eq!(rgb_to_hex(&rgb_format()), Ok(hex_format()));
    }

    #[test]
    fn hsv_to_rgb_works() {
        assert_eq!(hsv_to_rgb(&hsv_format()), Ok(rgb_format()));
    }

    #[test]
    fn hsv_to_hex_works() {
        assert_eq!(hsv_to_hex(&hsv_format()), Ok(hex_format()));
    }

    #[test]
    fn hex_to_rgb_works() {
        assert_eq!(hex_to_rgb(&hex_format()), Ok(rgb_format()));
    }

    #[test]
    fn hex_to_hsv_works() {
        assert_eq!(hex_to_hsv(&hex_format()), Ok(hsv_format()));
    }
}
