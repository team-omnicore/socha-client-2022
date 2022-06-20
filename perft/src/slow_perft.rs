use std::collections::{HashMap};
use client::game::{Gamestate, IGamestate, Team};

pub fn perft_up_to(starting_position: Gamestate, depth: u32) -> u64{
    if depth == 0 {
        return 0;
    }else {
        perft_recursive(starting_position, starting_position.current_player(), depth-1)
    }
}

fn perft_recursive(state: Gamestate, current_team: Team, depth: u32) -> u64{
    if depth == 0  {
        return state.count_moves(current_team) as u64;
    }

    let mut mov_count = 0;

    state.for_each_move(current_team, &mut |m|{
        let mut child =  state.clone();
        child.apply_move(&m);

        if !child.game_over() {
            mov_count += perft_recursive(child, current_team.opponent(), depth -1);
        }

    });
    return mov_count;
}