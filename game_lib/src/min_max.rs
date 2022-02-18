use num_traits::{NumCast, Num, Bounded};
use std::ops::Neg;
use thincollections::thin_vec::ThinVec;
use std::fmt::{Debug, Display};

pub trait IMove/*: Debug + Display*/ {}

pub trait MinMax: Copy + Display + Default + PartialEq + Debug{

    type MoveType: IMove + Copy + Sized;
    type EvalType: Num + Sized + Copy + NumCast + PartialOrd + Ord + Bounded + Neg<Output = Self>;

    /// Get the available, legal moves of the current player
    fn available_moves(&self)->ThinVec<Self::MoveType>;

    /// Apply a Move to the the gamestate
    fn apply_move<E: IMove>(&mut self, game_move:&E);

    /// Return, whether the game has ended with this gamestate
    fn game_over(&self)->bool;

    /// Set the current player to the next player
    fn next_player(&mut self);

    /// Evaluate the current position
    fn evaluate(&self, maximizing_player: bool)->Self::EvalType;

    /// Calculate the best legal move of the current player
    fn best_move(&self, search_depth: u8) -> Option<Self::MoveType> {
        let mut max_eval = Self::EvalType::max_value();
        let mut best_move = None;

        let mut alpha = Self::EvalType::min_value();
        let beta = Self::EvalType::max_value();

        let legal_moves = &self.available_moves();
        for game_move in legal_moves {
            let mut next_state = *self;
            next_state.apply_move(game_move);
            next_state.next_player();
            let eval = min_max(
                next_state,
                search_depth,
                alpha,
                beta,
                false,
            );
            if eval > max_eval {
                max_eval = eval;
                best_move = Some(*game_move);
            }
            let alpha = Self::EvalType::max(alpha, eval);
            if beta <= alpha {
                break;
            }
        }
        best_move
    }
}

/// Run min max algorithm, keep function in private scope.
fn min_max<T: MinMax>(
    state: T,
    depth: u8,
    mut alpha: T::EvalType,
    mut beta: T::EvalType,
    is_maximizing: bool,
) -> T::EvalType {
    let possible_moves = state.available_moves();
    if depth == 0 || state.game_over() {
        return state.evaluate(is_maximizing); //TODO maybe add to previous recursion step
    }
    return if is_maximizing {
        //Maximizing player (Client player)
        let mut max_eval = T::EvalType::min_value();
        for m in &possible_moves {
            let mut next_state = state.clone();
            next_state.apply_move(m);
            next_state.next_player();
            let eval = min_max(next_state, depth - 1, alpha, beta, false);
            max_eval = T::EvalType::max(max_eval, eval);
            alpha = T::EvalType::max(alpha, eval);
            if beta <= alpha {
                break;
            }
        }
        max_eval
    } else {
        //Minimizing palyer (Enemy player)
        let mut min_eval = T::EvalType::max_value();
        for m in &possible_moves {
            let mut next_state = state.clone();
            next_state.apply_move(m);
            next_state.next_player();
            let eval = min_max(next_state, depth - 1, alpha, beta, true);
            min_eval = T::EvalType::min(min_eval, eval);
            beta = T::EvalType::min(min_eval, eval);
            if beta <= alpha {
                break;
            }
        }
        min_eval
    };
}