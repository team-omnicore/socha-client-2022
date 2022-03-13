use crate::traits::IGamestate;
use num_traits::{Bounded, Num, NumCast};
use std::fmt::Display;

pub static mut COUNTER: u64 = 0;

pub trait MinMaxState: IGamestate {
    type EvalType: Num + Sized + Copy + NumCast + PartialOrd + Ord + Bounded + Display;

    /// Evaluate the current position
    fn evaluate(&self, is_maximizing: bool) -> Self::EvalType;
}

pub trait MinMax: MinMaxState {
    fn best_min_max_move(&self, search_depth: u8) -> Option<Self::MoveType> {
        unsafe { COUNTER = 0 };
        let mut pairs = vec![];

        self.for_each_legal_move(&mut |mov| {
            let mut child = self.clone();
            child.apply_move(&mov);
            child.next_player();

            let value = min_max(
                child,
                search_depth - 1,
                Self::EvalType::min_value(),
                Self::EvalType::max_value(),
                false,
            );
            pairs.push((value, mov));
            false
        });

        let max = pairs.iter().max_by_key(|pair| pair.0);
        println!("Max Eval: {}", max.unwrap().0.clone());
        let best = max.unwrap().1.clone();

        unsafe { println!("Counted {} moves", COUNTER) };
        Some(best)
    }
}

impl<E: MinMaxState> MinMax for E {}

/// Run min max algorithm, keep function in MinMaxate scope.
fn min_max<T: MinMax>(
    state: T,
    depth: u8,
    mut alpha: T::EvalType,
    mut beta: T::EvalType,
    is_maximizing: bool,
) -> T::EvalType {
    if depth == 0 || state.game_over() {
        unsafe {
            COUNTER += 1;
        }
        return state.evaluate(is_maximizing);
    }

    return if is_maximizing {
        //Maximizing player (Client player)
        let mut value = T::EvalType::min_value();

        state.for_each_legal_move(&mut |mov| {
            let mut child = state.clone();
            child.apply_move(&mov);
            child.next_player();

            value = T::EvalType::max(value, min_max(child, depth - 1, alpha, beta, false));
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

        state.for_each_legal_move(&mut |mov| {
            let mut child = state.clone();
            child.apply_move(&mov);
            child.next_player();

            value = T::EvalType::min(value, min_max(child, depth - 1, alpha, beta, true));
            beta = T::EvalType::min(alpha, value);

            if value <= alpha {
                return true; //* α-cutoff *
            }
            false
        });
        value
    };
}
