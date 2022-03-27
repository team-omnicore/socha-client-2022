use crate::game::{Gamestate, Move, Team};

pub trait Algorithm: Clone {
    fn best_move(&mut self, state: Gamestate, my_team: Team) -> Move;
}

pub type EvaluationFunction<G, E> = fn(&G, Team) -> E;
