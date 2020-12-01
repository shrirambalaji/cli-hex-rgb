use std::{fmt, num::ParseIntError};

#[derive(Debug)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
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
                "Invalid hexadecimal color code representation. hexcode has odd number of bytes"
                    .fmt(f)
            }
            HexError::Empty => "hexcode is empty.".fmt(f),
            HexError::Invalid => "Invalid hexadecimal color code representation.".fmt(f),
        }
    }
}

impl Color {
    pub fn new(hex_code: &str) -> Result<Color, String> {
        if hex_code.is_empty() {
            return Err(HexError::Empty.to_string());
        }

        // remove # from hex_code
        let hex_code = if hex_code.starts_with('#') {
            crop_letters(hex_code, 1)
        } else {
            hex_code
        };

        // convert shorthand RGB hexcode to RRGGBB
        let hex_code = if hex_code.len() == 3 {
            repeat_letters(hex_code, 1)
        } else {
            hex_code.to_owned()
        };

        if hex_code.len() % 2 != 0 {
            return Err(HexError::Invalid.to_string());
        }

        let decoded_values = decode_hex(&hex_code).unwrap_or_default();
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

impl PartialEq for Color {
    fn eq(&self, other: &Color) -> bool {
        self.red == other.red && self.green == other.green && self.blue == other.blue
    }
}

/// Crops letters from 0 to pos-1 and returns the rest of the string slice.
fn crop_letters(s: &str, pos: usize) -> &str {
    match s.char_indices().nth(pos) {
        Some((pos, _)) => &s[pos..],
        None => "",
    }
}

/// Repeats every character in a string.
fn repeat_letters(s: &str, repetitions: i32) -> String {
    let mut output = String::from("");
    for char in s.chars() {
        for _ in 0..=repetitions {
            output.push(char);
        }
    }

    output
}

/// Converts hexcode into a vector of rgb values
fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

/// Converts a hexcode to an rgb string
pub fn convert_hexcode_to_rgb(hex_code: String) -> Result<Color, String> {
    match Color::new(&hex_code) {
        Ok(color) => Ok(color),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vec_compare<T: std::cmp::PartialEq>(va: &[T], vb: &[T]) -> bool {
        (va.len() == vb.len()) &&  // zip stops at the shortest
     va.iter()
       .zip(vb)
       .all(|(a,b)| *a == *b)
    }

    #[test]
    fn should_convert_hexcode_to_rgb() {
        let hex = String::from("#ffffff");
        let expected_color: Color = Color {
            red: 255,
            green: 255,
            blue: 255,
        };

        assert_eq!(expected_color, convert_hexcode_to_rgb(hex).unwrap())
    }

    #[test]
    fn should_convert_shorthand_hexcode_to_rgb() {
        let hex = String::from("#fff");
        let expected_color: Color = Color {
            red: 255,
            green: 255,
            blue: 255,
        };

        assert_eq!(expected_color, convert_hexcode_to_rgb(hex).unwrap())
    }

    #[test]

    fn should_throw_error_when_hexcode_is_invalid() {
        let hex = String::from("#fasfsfff");
        let err = convert_hexcode_to_rgb(hex).unwrap_err();
        assert_eq!(err, HexError::Invalid.to_string());
    }

    #[test]

    fn should_throw_error_when_shorthand_hexcode_is_invalid() {
        let hex = String::from("#a0g");
        let err = convert_hexcode_to_rgb(hex).unwrap_err();
        assert_eq!(err, HexError::Invalid.to_string());
    }

    #[test]

    fn should_decode_hex() {
        let decoded = decode_hex("ffffff").unwrap();
        let expected: Vec<u8> = vec![255, 255, 255];
        assert!(vec_compare(&expected, &decoded), true);
    }

    #[test]
    fn should_repeat_letters_once() {
        assert_eq!("ff33ff".to_owned(), repeat_letters("f3f", 1));
    }

    #[test]
    fn should_crop_letters() {
        assert_eq!("ello", crop_letters("hello", 1));
        assert_eq!("fff", crop_letters("#fff", 1));
        assert_eq!("word", crop_letters("longword", 4));
    }
}
