use crate::errors::InvalidColourFormat;

fn within_bounds<T: PartialOrd<T>>(val: T, min: T, max: T) -> bool {
    min <= val && val <= max
}

#[derive(Clone, Debug, PartialEq)]
pub struct RGB {
    r: u8,
    g: u8,
    b: u8,
}

impl RGB {
    pub fn new(r: u8, g: u8, b: u8) -> Result<Self, InvalidColourFormat> {
        // TODO: work out how to properly handle range error, because it will arise at function
        // call
        // u8 can only be 0-255 anyway
        // if !within_bounds(r, 0, 255)
        //     || !within_bounds(g, 0, 255)
        //     || !within_bounds(b, 0, 255) {
        //     Err(InvalidColourFormat::ArgOutOfBoundsError)
        // } else {
        Ok(Self { r, g, b })
        // }
    }

    pub fn to_string(&self) -> String {
        format!("rgb({}, {}, {})", self.r, self.g, self.b)
    }

    pub fn r(&self) -> &u8 {
        &self.r
    }

    pub fn g(&self) -> &u8 {
        &self.g
    }

    pub fn b(&self) -> &u8 {
        &self.b
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct HSV {
    h: f32,
    s: f32,
    v: f32,
}

impl HSV {
    pub fn new(h: f32, s: f32, v: f32) -> Result<Self, InvalidColourFormat> {
        if !within_bounds(h, 0.0, 360.0)
            || !within_bounds(s, 0.0, 100.0)
            || !within_bounds(v, 0.0, 100.0)
        {
            Err(InvalidColourFormat::ArgOutOfBoundsError)
        } else {
            Ok(Self { h, s, v })
        }
    }

    // TODO: need to know more about commonly expected hsv formats to make these useful
    pub fn to_string(&self) -> String {
        format!("hsv({}, {}, {})", self.h, self.s, self.v)
    }

    pub fn to_string_as_percent(&self) -> String {
        format!("hsv({}, {}%, {}%)", self.h, self.s, self.v)
    }

    pub fn h(&self) -> &f32 {
        &self.h
    }

    pub fn s(&self) -> &f32 {
        &self.s
    }

    pub fn v(&self) -> &f32 {
        &self.v
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Hex {
    hashed_str: String,
    unhashed_str: String,
    r: String,
    g: String,
    b: String,
}

impl Hex {
    pub fn new(hex: &str) -> Result<Self, InvalidColourFormat> {
        let hashed_str: String;
        let unhashed_str: String;

        if hex.starts_with('#') {
            hashed_str = hex.to_string();
            unhashed_str = hex[1..].to_string();
        } else {
            hashed_str = format!("#{hex}");
            unhashed_str = hex.to_string();
        };

        let r_slice: String = unhashed_str[0..2].to_string();
        let g_slice: String = unhashed_str[2..4].to_string();
        let b_slice: String = unhashed_str[4..6].to_string();

        let r = u8::from_str_radix(&r_slice, 16)?;
        let g = u8::from_str_radix(&g_slice, 16)?;
        let b = u8::from_str_radix(&b_slice, 16)?;

        if !within_bounds(r, 0, 255) || !within_bounds(g, 0, 255) || !within_bounds(b, 0, 255) {
            Err(InvalidColourFormat::ArgOutOfBoundsError)
        } else {
            Ok(Self {
                hashed_str,
                unhashed_str,
                r: r_slice,
                g: g_slice,
                b: b_slice,
            })
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}", self.unhashed_str)
    }

    pub fn to_string_with_hash(&self) -> String {
        format!("{}", self.hashed_str)
    }

    pub fn r(&self) -> &String {
        &self.r
    }

    pub fn g(&self) -> &String {
        &self.g
    }

    pub fn b(&self) -> &String {
        &self.b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rgb_new_works() {
        let rgb = RGB::new(255, 0, 255);
        assert_eq!(
            rgb,
            Ok(RGB {
                r: 255,
                g: 0,
                b: 255
            })
        );
    }

    // #[test]
    // fn rgb_new_fails_with_invalid_input() {
    //     let rgb = RGB::new(999, 999, 999);
    //     assert_eq!(rgb, Err(InvalidColourFormat::ArgOutOfBoundsError));
    // }

    #[test]
    fn rgb_to_string_works() {
        let rgb = RGB::new(255, 0, 255).unwrap();
        let rgb_string = rgb.to_string();
        assert_eq!(rgb_string, "rgb(255, 0, 255)");
    }

    #[test]
    fn hsv_new_works() {
        let hsv = HSV::new(300.0, 100.0, 100.0);
        assert_eq!(
            hsv,
            Ok(HSV {
                h: 300.0,
                s: 100.0,
                v: 100.0
            })
        );
    }

    #[test]
    fn hsv_new_fails_with_invalid_input() {
        let hsv = HSV::new(999.0, 999.0, 999.0);
        assert_eq!(hsv, Err(InvalidColourFormat::ArgOutOfBoundsError));
    }

    #[test]
    fn hsv_to_string_works() {
        let hsv = HSV::new(300.0, 100.0, 100.0).unwrap();
        let hsv_string = hsv.to_string();
        assert_eq!(hsv_string, "hsv(300, 100, 100)");
    }

    #[test]
    fn hsv_to_string_as_percent_works() {
        let hsv = HSV::new(300.0, 100.0, 100.0).unwrap();
        let hsv_string = hsv.to_string_as_percent();
        assert_eq!(hsv_string, "hsv(300, 100%, 100%)");
    }

    #[test]
    fn hex_new_works() {
        let hex = Hex::new("#ff00ff");
        assert_eq!(
            hex,
            Ok(Hex {
                hashed_str: "#ff00ff".to_string(),
                unhashed_str: "ff00ff".to_string(),
                r: "ff".to_string(),
                g: "00".to_string(),
                b: "ff".to_string(),
            })
        );
    }

    #[test]
    fn hex_new_fails_with_invalid_input() {
        let hex = Hex::new("#gggggg");
        assert!(hex.is_err());
    }

    #[test]
    fn hex_to_string_works() {
        let hex = Hex::new("ff00ff").unwrap();
        let hex_string = hex.to_string();
        let hex_string_with_hash = hex.to_string_with_hash();

        assert_eq!(hex_string, "ff00ff");
        assert_eq!(hex_string_with_hash, "#ff00ff");
    }
}
