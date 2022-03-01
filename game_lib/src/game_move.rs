use crate::piece::PieceType;
use game_algorithms::traits::IMove;

#[derive(Copy, Clone, Debug)]
pub struct Move {
    pub from: u8,
    pub to: u8,
    pub piece: PieceType,
}

impl IMove for Move {}
