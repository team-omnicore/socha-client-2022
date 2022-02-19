use num_traits::{Bounded, Num, NumCast};
use std::fmt::{Debug, Display};
use thincollections::thin_vec::ThinVec;

pub trait IMove /*: Debug + Display*/ {}

pub static mut COUNTER: u64 = 0;

pub trait MinMax: Copy + Display + Debug {
    type MoveType: IMove + Copy + Sized;
    type EvalType: Num + Sized + Copy + NumCast + PartialOrd + Ord + Bounded + Display;

    /// Get the available, legal moves of the current player
    fn available_moves(&self) -> ThinVec<Self::MoveType>;

    /// Apply a Move to the the gamestate
    fn apply_move(&mut self, game_move: &Self::MoveType);

    /// Return, whether the game has ended with this gamestate
    fn game_over(&self) -> bool;

    /// Set the current player to the next player
    fn next_player(&mut self);

    /// Evaluate the current position
    fn evaluate(&self) -> Self::EvalType;
}

pub trait Priv: MinMax {

    fn calculate_best_move(&self, search_depth: u8) -> Option<Self::MoveType> {
        unsafe { COUNTER = 0};
        let mut pairs = vec![];

        let legal = self.available_moves();
        for game_move in &legal {
            let mut clone = *self;
            clone.apply_move(game_move);
            clone.next_player();

            let eval = min_max(clone, search_depth-1, Self::EvalType::min_value(), Self::EvalType::max_value(), false);
            pairs.push((game_move, eval));
        }

        let max = pairs.iter().max_by_key(|pair|{
            pair.1
        });

        let best = max.unwrap().0.clone();

        unsafe { println!("Counted {} moves", COUNTER) };
        Some(best)
    }
}

impl<E: MinMax> Priv for E {}

/// Run min max algorithm, keep function in private scope.
fn min_max<T: MinMax>(
    state: T,
    depth: u8,
    mut alpha: T::EvalType,
    mut beta: T::EvalType,
    is_maximizing: bool,
) -> T::EvalType {
    let possible_moves = state.available_moves();
    return if is_maximizing {
        let mut value = T::EvalType::min_value();
        for game_move in &possible_moves {
            let mut child = state.clone();
            child.apply_move(game_move);
            child.next_player();

            let depth = depth-1;
            if depth == 0 || child.game_over() {
                unsafe { COUNTER += 1; }
                return child.evaluate();
            }

            value = T::EvalType::max(value, min_max(child, depth, alpha, beta, false));
            alpha = T::EvalType::max(alpha, value);

            if value >= beta {
                break; //* β-cutoff *
            }
        }
        value
    } else {
        //Minimizing player (Enemy player)
        let mut value = T::EvalType::max_value();
        for game_move in &possible_moves {
            let mut child = state.clone();
            child.apply_move(game_move);
            child.next_player();

            let depth = depth-1;
            if depth == 0 || child.game_over() {
                unsafe { COUNTER += 1; }
                return child.evaluate();
            }

            value = T::EvalType::min(value, min_max(child, depth, alpha, beta, true));
            beta = T::EvalType::min(beta, value);

            if value <= alpha {
                break; //* α-cutoff *
            }
        }
        value
    };
}
