use crate::game::Team;
use thincollections::thin_vec::ThinVec;

pub trait IGamestate: Copy + Clone {
    type MoveType: IMove + Copy + Sized;

    /// Get the available, legal moves of the current player
    fn available_moves(&self, team: Team) -> ThinVec<Self::MoveType>;

    /// Get the available moves for the current player
    fn available_moves_current_player(&self) -> ThinVec<Self::MoveType> {
        self.available_moves(self.current_player())
    }

    /// Get the current player
    fn current_player(&self) -> Team;

    /// Efficiently count the amount of available moves without
    /// redundantly storing the moves.
    fn count_moves(&self, team: Team) -> u8;

    /// Count the moves of the current player
    fn count_moves_current_player(&self) -> u8 {
        self.count_moves(self.current_player())
    }

    ///Iterate over each legal move, without allocating memory to store them
    fn for_each_move<F: FnMut(Self::MoveType)>(&self, team: Team, f: &mut F);

    /// Apply a Move to the the gamestate
    fn apply_move(&mut self, game_move: &Self::MoveType);

    /// Return, whether the game has ended with this gamestate
    fn game_over(&self) -> bool;
}

pub trait IMove {}
