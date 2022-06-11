use std::ops::Sub;
use std::thread::sleep;
use std::time::{Duration, Instant};
use rand::prelude::SliceRandom;
use rand::thread_rng;

use crate::algorithms::Algorithm;
use crate::game::{Gamestate, IGamestate, Move, Team};

#[derive(Copy, Clone)]
pub struct RandomPick{
    wait_time: Duration
}

impl RandomPick {
    fn do_while_wait(&self){}
}

impl Algorithm for RandomPick {
    fn best_move(&mut self, state: Gamestate, my_team: Team) -> Move {
        let start = Instant::now();
        self.do_while_wait();
        sleep(self.wait_time.sub(Instant::now().duration_since(start)));
        *state
            .available_moves(my_team)
            .choose(&mut thread_rng())
            .unwrap()
    }
}
