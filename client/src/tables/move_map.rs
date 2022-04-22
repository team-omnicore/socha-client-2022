use crate::for_each_move;
use crate::game::{Gamestate, IGamestate, Move, Team};
use std::collections::HashMap;
use std::fs::File;

use crate::bit_loop;
use crate::game::*;
use crate::utils::square_of;

pub struct MoveMap {
    pairs: HashMap<u64, Move>,
}

impl MoveMap {
    fn write_to_file(&self, path: &str) {
        let file = File::create(path).unwrap();

        let vec: Vec<_> = self.pairs.iter().collect();
    }
}

pub fn gen_table(always_win_states:&mut HashMap<Gamestate, Move>, state: Gamestate, client_player: Team) -> bool {
    if state.game_over() {
        if let Some(winner) = state.winner() {
            return winner == client_player;
        }
    }

    if state.current_player == client_player {
        for_each_move!(state.board, state.current_player, m, {
            let mut child = state.clone();
            child.apply_move(&m);

            if gen_table(always_win_states, child, client_player) {
                always_win_states.insert(state, m);
                return true;
            }
        });
    } else {
        for_each_move!(state.board, state.current_player, m, {
            let mut child = state.clone();
            child.apply_move(&m);

            if !gen_table(always_win_states, child, client_player) {
                return false;
            }
        });
    }
    false
}
