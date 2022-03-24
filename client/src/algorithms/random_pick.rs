use rand::prelude::SliceRandom;
use rand::thread_rng;

use crate::algorithms::Algorithm;
use crate::game::{Gamestate, IGamestate, Move, Team};

#[derive(Copy, Clone)]
pub struct RandomPick;

impl Algorithm for RandomPick {
    fn best_move(&mut self, state: Gamestate, my_team: Team) -> Move {
        *state
            .available_moves(my_team)
            .choose(&mut thread_rng())
            .unwrap()
    }
}
