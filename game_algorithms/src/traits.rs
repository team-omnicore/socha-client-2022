use std::fmt::{Debug, Display};
use thincollections::thin_vec::ThinVec;

pub trait IGamestate : Copy + Clone + Display + Debug {
    type MoveType: IMove + Copy + Sized;

    /// Get the available, legal moves of the current player
    fn available_moves(&self) -> ThinVec<Self::MoveType>;

    fn for_each_legal_move<F: FnMut(Self::MoveType) -> bool>(&self, f: &mut F);

    /// Apply a Move to the the gamestate
    fn apply_move(&mut self, game_move: &Self::MoveType);

    /// Return, whether the game has ended with this gamestate
    fn game_over(&self) -> bool;

    /// Set the current player to the next player
    fn next_player(&mut self);
}

pub trait IMove {}