use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Clone, Default, PartialEq, Eq)]
pub enum FavouriteColor {
    #[default]
    Red,
    Yellow,
    Cyan,
    Custom {
        red: u8,
        green: u8,
        blue: u8,
    },
}

impl FromStr for FavouriteColor {
    type Err = FavouriteColorParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "red" => Ok(FavouriteColor::Red),
            "yellow" => Ok(FavouriteColor::Yellow),
            ("cyan" | "blue") => Ok(FavouriteColor::Cyan),
            _ => {
                match s.chars().nth(0) {
                    Some('0'..='9') => {}
                    _ => return Err(FavouriteColorParseError::InvalidColorString),
                }

                let mut bytes = Vec::new();

                for value in s.split(' ') {
                    bytes.push(u8::from_str(value)?)
                }

                if bytes.len() > 3 {
                    Err(FavouriteColorParseError::InvalidCustomColor(None))
                } else {
                    Ok(FavouriteColor::Custom {
                        red: bytes[0],
                        green: bytes[1],
                        blue: bytes[2],
                    })
                }
            }
        }
    }
}




#[derive(Clone, PartialEq, Eq)]
pub enum FavouriteColorParseError {
    InvalidColorString,
    InvalidCustomColor(Option<ParseIntError>),
}

impl Display for FavouriteColorParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FavouriteColorParseError::InvalidColorString => write!(f, "Invalid color string"),
            FavouriteColorParseError::InvalidCustomColor(e) => match e {
                None => write!(f, "Invalid custom color"),
                Some(e) => write!(f, "Failed to parse integer - {}", e),
            },
        }
    }
}

impl From<ParseIntError> for FavouriteColorParseError {
    fn from(e: ParseIntError) -> Self {
        FavouriteColorParseError::InvalidCustomColor(Some(e))
    }
}