use crate::algorithms::EvaluationFunction;
use crate::game::{Gamestate, IGamestate, Team};

pub static EVAL_2603_1: EvaluationFunction<Gamestate, i32> = |state, team| -> i32 {
    let red_score = state.ambers[0];
    let blue_score = state.ambers[1];

    const WIN_REWARD: i32 = 100000;
    const LOSE_REWARD: i32 = -WIN_REWARD;

    const TIEBREAK_POSITIVE_REWARD: i32 = 50000;
    const TIEBREAK_NEGATIV_REWARD: i32 = -TIEBREAK_POSITIVE_REWARD;
    const TIE_REWARD: i32 = 1000;

    const POINTS_REWARD: i32 = 10000;
    const DOUBLE_PIECE_REWARD: i32 = 1000;
    const PIECE_REWARD: i32 = 100;

    let mut eval = 0;

    eval += red_score as i32 * POINTS_REWARD;
    eval -= blue_score as i32 * POINTS_REWARD;
    eval += (state.board.red & state.board.double).bits.count_ones() as i32 * DOUBLE_PIECE_REWARD;
    eval -= (state.board.blue & state.board.double).bits.count_ones() as i32 * DOUBLE_PIECE_REWARD;
    eval += state.board.red.bits.count_ones() as i32 * PIECE_REWARD;
    eval -= state.board.blue.bits.count_ones() as i32 * PIECE_REWARD;
    eval += state.count_moves(Team::ONE) as i32;
    eval -= state.count_moves(Team::TWO) as i32;

    if team == Team::TWO {
        eval *= -1;
    }

    match team {
        Team::ONE => {
            eval += if red_score > blue_score {
                WIN_REWARD
            } else if red_score < blue_score {
                LOSE_REWARD
            } else {
                let winner = state.wins_draw();
                if let Some(team) = winner {
                    match team {
                        Team::ONE => TIEBREAK_POSITIVE_REWARD,
                        Team::TWO => TIEBREAK_NEGATIV_REWARD,
                    }
                } else {
                    TIE_REWARD
                }
            }
        }
        Team::TWO => {
            eval += if blue_score > red_score {
                WIN_REWARD
            } else if blue_score < red_score {
                LOSE_REWARD
            } else {
                let winner = state.wins_draw();
                if let Some(team) = winner {
                    match team {
                        Team::ONE => TIEBREAK_NEGATIV_REWARD,
                        Team::TWO => TIEBREAK_POSITIVE_REWARD,
                    }
                } else {
                    TIE_REWARD
                }
            }
        }
    }
    eval
};
