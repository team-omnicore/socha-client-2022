#![allow(dead_code)]

use std::fmt::{Display, Formatter};
use std::str::FromStr;
use std::string::ParseError;

pub mod pieces {
    use crate::piece::{Piece, PieceType};

    pub const ROBBE: Piece = Piece::new("Robbe", &PieceType::ROBBE);

    pub const MUSCHEL: Piece = Piece::new("Herzmuschel", &PieceType::MUSCHEL);

    pub const SEESTERN: Piece = Piece::new("Seestern", &PieceType::SEESTERN);

    pub const MOEWE: Piece = Piece::new("Möwe", &PieceType::MOEWE);
}

#[derive(Clone, Debug, Copy)]
pub enum PieceType {
    ROBBE,
    MUSCHEL,
    SEESTERN,
    MOEWE,
}

#[derive(Debug, Copy, Clone)]
pub struct Piece {
    pub name: &'static str,
    pub typ: &'static PieceType,
}

impl Piece {
    pub const fn new(name: &'static str, typ: &'static PieceType) -> Self {
        Piece { name, typ: &typ }
    }
}

impl PieceType {
    pub fn piece_type_from_name(name: &String) -> Option<PieceType> {
        return match name.as_str() {
            "Moewe" => Some(PieceType::MOEWE),
            "Robbe" => Some(PieceType::ROBBE),
            "Herzmuschel" => Some(PieceType::MUSCHEL),
            "Seestern" => Some(PieceType::SEESTERN),
            _ => None,
        };
    }
}

impl From<&String> for PieceType {
    fn from(str: &String) -> Self {
        return match str.as_str() {
            "Moewe" => PieceType::MOEWE,
            "Robbe" => PieceType::ROBBE,
            "Herzmuschel" => PieceType::MUSCHEL,
            "Seestern" => PieceType::SEESTERN,
            piece => {
                panic!("No piece of type: {}", piece)
            }
        };
    }
}

impl Display for PieceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PieceType::ROBBE => pieces::ROBBE.name,
                PieceType::MUSCHEL => pieces::MUSCHEL.name,
                PieceType::SEESTERN => pieces::SEESTERN.name,
                PieceType::MOEWE => pieces::MOEWE.name,
            }
        )
    }
}

impl FromStr for PieceType {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Moewe" => Ok(PieceType::MOEWE),
            "Robbe" => Ok(PieceType::ROBBE),
            "Herzmuschel" => Ok(PieceType::MUSCHEL),
            "Seestern" => Ok(PieceType::SEESTERN),
            piece => {
                panic!("No piece of type: {}", piece)
            }
        }
    }
}
