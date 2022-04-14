use crate::game::Team;
use thincollections::thin_vec::ThinVec;

pub trait IGamestate: Copy + Clone {
    type MoveType: IMove + Copy + Sized;

    /// Get the available, legal moves of the current player
    fn available_moves(&self, team: Team) -> ThinVec<Self::MoveType>;

    /// Efficiently count the amount of available moves without
    /// redundantly storing the moves.
    fn count_moves(&self, team: Team) -> u8;

    ///Iterate over each legal move, without allocating memory to store them
    fn for_each_move<F: FnMut(Self::MoveType)>(&self, team: Team, f: &mut F);

    /// Apply a Move to the the gamestate
    fn apply_move(&mut self, game_move: &Self::MoveType);

    /// Return, whether the game has ended with this gamestate
    fn game_over(&self) -> bool;

    /// Set the current player to the next player
    fn next_player(&mut self);
}

pub trait IMove {}

pub trait Evaluable {
    /// Evaluate the state and return it's value.<br>
    /// Higher is better, lower is worse.
    fn evaluate(&self, team: Team) -> i32;
}
