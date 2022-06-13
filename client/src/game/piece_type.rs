use std::fmt;
use std::str::FromStr;

use crate::game::ShortForm;
use socha_client_2022::util::{SCError, SCResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PieceType {
    /// Moves only diagonally forwards.
    Herzmuschel = 0,
    /// Moves only to adjacent fields.
    Moewe = 1,
    /// Moves only diagonally or forwards.
    Seestern = 2,
    /// Like a knight in chess. Only non-light figure.
    Robbe = 3,
}

impl PieceType {
    /// Checks whether a piece is lightweight. Only the 'robbe' is non-light.
    #[inline]
    pub fn is_light(self) -> bool {
        !matches!(self, Self::Robbe)
    }
}

//noinspection DuplicatedCode
impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PieceType::Herzmuschel => write!(f, "Herzmuschel"),
            PieceType::Moewe => write!(f, "Moewe"),
            PieceType::Seestern => write!(f, "Seestern"),
            PieceType::Robbe => write!(f, "Robbe"),
        }
    }
}

//noinspection DuplicatedCode
impl FromStr for PieceType {
    type Err = SCError;

    fn from_str(s: &str) -> SCResult<Self> {
        match s {
            "Herzmuschel" => Ok(Self::Herzmuschel),
            "Moewe" => Ok(Self::Moewe),
            "Seestern" => Ok(Self::Seestern),
            "Robbe" => Ok(Self::Robbe),
            _ => Err(SCError::UnknownVariant(format!("Unknown piece type {}", s))),
        }
    }
}

impl ShortForm<char> for PieceType {
    type Err = SCError;

    fn to_short_form(&self) -> char {
        match self {
            Self::Herzmuschel => 'h',
            Self::Moewe => 'm',
            Self::Seestern => 's',
            Self::Robbe => 'r',
        }
    }

    fn from_short_form(c: &char) -> SCResult<Self> {
        match c.to_ascii_lowercase() {
            'h' => Ok(Self::Herzmuschel),
            'm' => Ok(Self::Moewe),
            's' => Ok(Self::Seestern),
            'r' => Ok(Self::Robbe),
            _ => Err(SCError::UnknownVariant(format!("Unknown short form {}", c))),
        }
    }
}
