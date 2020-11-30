use std::{fmt, num::ParseIntError};

#[derive(Debug)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

pub enum HexError {
    OddLength,
    Empty,
    Invalid,
}

impl fmt::Display for HexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HexError::OddLength => {
                "invalid hexcode representation. hexcode has odd number of bytes".fmt(f)
            }
            HexError::Empty => "hexcode is empty.".fmt(f),
            HexError::Invalid => "invalid hexcode representation.".fmt(f),
        }
    }
}

impl Color {
    pub fn new(hex_code: &str) -> Result<Color, String> {
        if hex_code.is_empty() {
            return Err(HexError::Empty.to_string());
        }

        let hex_code_without_hash = if hex_code.starts_with('#') {
            crop_letters(hex_code, 1)
        } else {
            hex_code
        };

        if hex_code_without_hash.len() % 2 != 0 {
            return Err(HexError::Invalid.to_string());
        }

        let decoded_values = decode_hex(hex_code_without_hash).unwrap_or(vec![]);
        if decoded_values.is_empty() || decoded_values.len() > 4 {
            return Err(HexError::Invalid.to_string());
        }

        let color = Color {
            red: decoded_values[0],
            green: decoded_values[1],
            blue: decoded_values[2],
        };

        Ok(color)
    }
}

/// Crops letters from 0 to pos-1 and returns the rest of the string slice.
fn crop_letters(s: &str, pos: usize) -> &str {
    match s.char_indices().skip(pos).next() {
        Some((pos, _)) => &s[pos..],
        None => "",
    }
}

/// Converts hexcode into a vector of rgb values
fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

/// Converts a hexcode to an rgb string
pub fn convert_hexcode_to_rgb(hex_code: String) -> Result<String, String> {
    match Color::new(&hex_code) {
        Ok(color) => Ok(format!(
            "rgb({}, {}, {})",
            color.red, color.green, color.blue
        )),
        Err(e) => Err(e),
    }
}

#[test]
fn should_convert_hexcode_to_rgb() {
    let hex = String::from("#ffffff");
    assert_eq!(
        "rgb(255, 255, 255)".to_owned(),
        convert_hexcode_to_rgb(hex).unwrap()
    )
}

#[test]

fn should_throw_error_when_hexcode_is_invalid() {
    let hex = String::from("#fasfsfff");
    let err = convert_hexcode_to_rgb(hex).unwrap_err();
    assert_eq!(err, HexError::Invalid.to_string());
}
