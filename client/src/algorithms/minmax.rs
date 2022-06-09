use crate::algorithms::{Algorithm, EvaluationFunction};
use crate::for_each_move;
use crate::game::{Gamestate, IGamestate, Move, Team};
use num_traits::{Bounded, Num, NumCast};
use std::fmt::Display;
use std::time::{Duration, Instant};

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

        let mut move_index: u8 = 0;
        let move_count = state.count_moves(my_team);
        let mut dynamic_depth: u8 = self.max_depth; // actually start_depth
        let max_search_duration = Duration::from_millis(1800);
        let mut average_search_duration = Duration::from_millis(0);
        let mut last_duration = Duration::from_millis(0);

        println!("Move count: {}", move_count);
        state.for_each_move(self.my_team, &mut |mov| {
            let start_timer = Instant::now();
            move_index += 1;

            if move_index > 2 {
                let end_average_millis = average_search_duration.as_millis() * move_count as u128;
                let end_last_millis = last_duration.as_millis() * move_count as u128;
                println!("Estimated AVERAGE end duration: {} ms", end_average_millis);
                println!("Estimated LAST end duration: {} ms", end_last_millis);

                if end_average_millis >= max_search_duration.as_millis()
                    || end_last_millis > max_search_duration.as_millis()
                {
                    dynamic_depth -= 1;
                } else if (end_average_millis * move_count as u128)
                    < max_search_duration.as_millis()
                    && (end_last_millis * move_count as u128) < max_search_duration.as_millis()
                {
                    dynamic_depth += 1;
                }
            }

            println!(
                "Average search duration: {} ms",
                average_search_duration.as_millis()
            );
            println!("Dynamic depth: {}", dynamic_depth);

            let mut child = state.clone();
            child.apply_move(&mov);
            child.next_player();

            let value = self.min_max(
                child,
                dynamic_depth - 1,
                self.my_team.opponent(),
                <Gamestate as MinMaxState>::EvalType::min_value(),
                <Gamestate as MinMaxState>::EvalType::max_value(),
            );

            last_duration = start_timer.elapsed();
            average_search_duration = Duration::from_millis(
                ((average_search_duration.as_millis() * (move_index - 1) as u128
                    + last_duration.as_millis())
                    / move_index as u128) as u64,
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
                    return max_eval; //* β-cutoff *
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
                    return min_eval; //* α-cutoff *
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
