use crate::game::{IMove, PieceType};

#[derive(Copy, Clone, Debug)]
pub struct Move {
    pub from: u8,
    pub to: u8,
    pub piece: PieceType,
}

impl IMove for Move {}
