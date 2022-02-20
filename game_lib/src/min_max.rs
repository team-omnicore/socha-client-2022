use num_traits::{Bounded, Num, NumCast};
use std::fmt::{Debug, Display};
use thincollections::thin_vec::ThinVec;

pub trait IMove /*: Debug + Display*/ {}

pub static mut COUNTER: u64 = 0;

pub trait MinMax: Copy + Clone + Display + Debug {
    type MoveType: IMove + Copy + Sized;
    type EvalType: Num + Sized + Copy + NumCast + PartialOrd + Ord + Bounded + Display;

    /// Get the available, legal moves of the current player
    fn available_moves(&self) -> ThinVec<Self::MoveType>;

    fn for_each_legal_move<F: FnMut(Self::MoveType)->bool>(&self, f: &mut F);

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

        self.for_each_legal_move(&mut |mov|{
            let mut child = self.clone();
            child.apply_move(&mov);
            child.next_player();

            let value = min_max(child, search_depth-1, Self::EvalType::min_value(), Self::EvalType::max_value(), false);
            pairs.push((value, mov));
            false
        });

        let max = pairs.iter().max_by_key(|pair|{
            pair.0
        });

        let best = max.unwrap().1.clone();

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

    if depth == 0 || state.game_over() {
        unsafe { COUNTER += 1; }
        return state.evaluate();
    }

    return if is_maximizing {
        //Maximizing player (Client player)
        let mut value = T::EvalType::min_value();

        state.for_each_legal_move(&mut |mov|{
            let mut child = state.clone();
            child.apply_move(&mov);
            child.next_player();

            value = T::EvalType::max(value, min_max(child, depth-1, alpha, beta, false));
            alpha = T::EvalType::max(alpha, value);

            if value >= beta {
                return true; //* β-cutoff *
            }
            false
        });
        value
    } else {
        //Minimizing player (Enemy player)
        let mut value = T::EvalType::max_value();

        state.for_each_legal_move(&mut |mov|{
            let mut child = state.clone();
            child.apply_move(&mov);
            child.next_player();

            value = T::EvalType::min(value, min_max(child, depth-1, alpha, beta, true));
            beta = T::EvalType::min(alpha, value);

            if value <= alpha {
                return true; //* α-cutoff *
            }
            false
        });
        value
    };
}
