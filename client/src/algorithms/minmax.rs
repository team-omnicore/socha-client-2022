use crate::algorithms::{Algorithm, EvaluationFunction};
use crate::for_each_move;
use crate::game::{Gamestate, IGamestate, Move, Team};
use num_traits::{Bounded, Num, NumCast};
use std::fmt::Display;

#[derive(Clone)]
pub struct MinMax<E: MinMaxState + IGamestate> {
    max_depth: u8,
    my_team: Team,
    evaluation: fn(&E, Team) -> E::EvalType,
}

pub trait MinMaxState {
    type EvalType: Num + Sized + Copy + NumCast + PartialOrd + Ord + Bounded + Display;
}

impl MinMax<Gamestate> {
    pub fn new(
        search_depth: u8,
        evaluation: EvaluationFunction<Gamestate, <Gamestate as MinMaxState>::EvalType>,
    ) -> Self {
        Self {
            max_depth: search_depth,
            my_team: Team::ONE, //Gets corrected anyway.
            evaluation,
        }
    }

    fn recommend_move(
        &mut self,
        state: Gamestate,
        my_team: Team,
    ) -> <Gamestate as IGamestate>::MoveType {
        let mut move_value_pairs = vec![];
        self.my_team = my_team;
        state.for_each_move(self.my_team, &mut |mov| {
            let mut child = state.clone();
            child.apply_move(&mov);
            child.next_player();

            let value = self.min_max(
                child,
                self.max_depth - 1,
                self.my_team.opponent(),
                <Gamestate as MinMaxState>::EvalType::min_value(),
                <Gamestate as MinMaxState>::EvalType::max_value(),
            );
            move_value_pairs.push((value, mov));
        });
        let max = move_value_pairs.iter().max_by_key(|pair| pair.0);
        println!("Value: {}", max.unwrap().0);
        max.unwrap().1.clone()
    }

    fn min_max(
        &self,
        state: Gamestate,
        depth: u8,
        team: Team,
        mut alpha: <Gamestate as MinMaxState>::EvalType,
        mut beta: <Gamestate as MinMaxState>::EvalType,
    ) -> <Gamestate as MinMaxState>::EvalType {
        if depth == 0 || state.game_over() {
            return (self.evaluation)(&state, self.my_team);
        }

        let is_maximizing = team == self.my_team;

        if is_maximizing {
            //Maximizing player (Client player)
            let mut max_eval = <Gamestate as MinMaxState>::EvalType::min_value();
            for_each_move!(state.board, team, mov, {
                let mut child = state.clone();
                child.apply_move(&mov);
                child.next_player();

                let eval = self.min_max(child, depth - 1, team.opponent(), alpha, beta);
                max_eval = <Gamestate as MinMaxState>::EvalType::max(max_eval, eval);
                alpha = <Gamestate as MinMaxState>::EvalType::max(alpha, eval);

                if beta <= alpha {
                    return max_eval;//* β-cutoff *
                }
            });
            return max_eval;
        } else {
            //Minimizing player (Enemy player)
            let mut min_eval = <Gamestate as MinMaxState>::EvalType::max_value();
            for_each_move!(state.board, team, mov, {
                let mut child = state.clone();
                child.apply_move(&mov);
                child.next_player();

                let eval = self.min_max(child, depth - 1, team.opponent(), alpha, beta);
                min_eval = <Gamestate as MinMaxState>::EvalType::min(min_eval, eval);
                beta = <Gamestate as MinMaxState>::EvalType::min(beta, eval);

                if beta <= alpha {
                    return min_eval;//* α-cutoff *
                }
            });
            return min_eval;
        };
    }
}

impl Algorithm for MinMax<Gamestate> {
    fn best_move(&mut self, state: Gamestate, my_team: Team) -> Move {
        self.recommend_move(state, my_team)
    }
}

impl MinMaxState for Gamestate {
    type EvalType = i32;
}
