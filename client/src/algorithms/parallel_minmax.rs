use crate::algorithms::{Algorithm, EvaluationFunction, MinMaxState};
use crate::for_each_move;
use crate::game::{Gamestate, IGamestate, Move, Team};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Clone, Copy)]
pub struct ParallelMinmax<E: MinMaxState + IGamestate> {
    max_depth: u8,
    my_team: Team,
    evaluation: fn(&E, Team) -> E::EvalType,
    worker_count: usize,
}

impl ParallelMinmax<Gamestate> {
    pub fn new(
        search_depth: u8,
        evaluation: EvaluationFunction<Gamestate, <Gamestate as MinMaxState>::EvalType>,
        worker_count: usize,
    ) -> Self {
        Self {
            max_depth: search_depth,
            my_team: Team::ONE, //Gets corrected anyway.
            evaluation,
            worker_count,
        }
    }

    fn recommend_move(&self, state: Gamestate) -> <Gamestate as IGamestate>::MoveType {
        println!("Using {} threads", self.worker_count);

        let algo = self.clone();
        let moves = state.available_moves(algo.my_team);
        let mut handles = Vec::new();
        let move_value = Arc::new(Mutex::new(Vec::new()));

        let mut chunks = vec![];
        for chunk in moves.chunks((moves.len() / algo.worker_count).max(1)) {
            chunks.push(chunk.to_owned());
        }

        for chunk in chunks {
            let values = Arc::clone(&move_value);
            let handle = thread::spawn(move || {
                for mov in chunk {
                    let mut child = state.clone();
                    child.apply_move(&mov);
                    child.next_player();

                    let value = algo.min_max(
                        child,
                        algo.max_depth - 1,
                        algo.my_team.opponent(),
                        <Gamestate as MinMaxState>::EvalType::min_value(),
                        <Gamestate as MinMaxState>::EvalType::max_value(),
                    );
                    values.lock().unwrap().push((mov, value));
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let max = Arc::try_unwrap(move_value).unwrap().into_inner().unwrap();
        let max = max.iter().max_by_key(|pair| pair.1).unwrap();
        println!("Value: {:?}", max.1);
        return max.0.clone();
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

impl Algorithm for ParallelMinmax<Gamestate> {
    fn best_move(&mut self, state: Gamestate, my_team: Team) -> Move {
        self.my_team = my_team;
        self.recommend_move(state)
    }
}
