use crate::min_max::IMove;

#[derive(Copy, Clone)]
pub struct Move {
    pub from: u8,
    pub to: u8,
}

impl IMove for Move {}
