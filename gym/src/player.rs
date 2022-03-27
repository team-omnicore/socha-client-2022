use client::algorithms::Algorithm;
use client::game::{Gamestate, Move, Team};
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::path::Path;

#[derive(Clone, PartialEq)]
pub struct Player<A: Algorithm> {
    pub name: String,
    pub algorithm: A,
    pub elo_score: f32,
}

impl<A: Algorithm> Player<A> {
    pub fn new(name: String, algorithm: A) -> Self {
        Self {
            name,
            algorithm,
            elo_score: 1000.0,
        }
    }

    pub fn request_move(&mut self, gamestate: Gamestate, team: Team) -> Move {
        self.algorithm.best_move(gamestate, team)
    }
}

impl<A: Algorithm> Debug for Player<A> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        //write!(f, "Player{{name: {}, elo: {}}} ", self.name, self.elo_score)
        //write!(f, "elo: {} ", self.elo_score)
        write!(f, "Player{{elo={}}}", self.elo_score)
    }
}

impl<A: Algorithm> Display for Player<A> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {:.0}", self.name, self.elo_score)
    }
}

impl<A: Algorithm> Hash for Player<A> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state)
    }
}
