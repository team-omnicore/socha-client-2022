use crate::game::{Gamestate, Move, Team};

pub trait Algorithm {
    fn best_move(&mut self, state: Gamestate, my_team: Team) -> Move;
}
