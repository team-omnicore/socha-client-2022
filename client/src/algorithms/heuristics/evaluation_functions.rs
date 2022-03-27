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
                let leicht_figuren =
                    state.board.moewen | state.board.seesterne | state.board.muscheln;
                let mut red_l = leicht_figuren & state.board.red;
                let mut blue_l = leicht_figuren & state.board.blue;
                let wins = Gamestate::draw_winner(
                    *red_l.rotate90_anti_clockwise(),
                    *blue_l.rotate90_clockwise(),
                );
                match wins {
                    1 => TIEBREAK_POSITIVE_REWARD,
                    0 => TIE_REWARD,
                    -1 => TIEBREAK_NEGATIV_REWARD,
                    _ => {
                        debug_assert!(false);
                        0
                    }
                }
            }
        }
        Team::TWO => {
            eval += if blue_score > red_score {
                WIN_REWARD
            } else if blue_score < red_score {
                LOSE_REWARD
            } else {
                let leicht_figuren =
                    state.board.moewen | state.board.seesterne | state.board.muscheln;
                let mut red_l = leicht_figuren & state.board.red;
                let mut blue_l = leicht_figuren & state.board.blue;
                let wins = Gamestate::draw_winner(
                    *blue_l.rotate90_clockwise(),
                    *red_l.rotate90_anti_clockwise(),
                );
                match wins {
                    1 => TIEBREAK_POSITIVE_REWARD,
                    0 => TIE_REWARD,
                    -1 => TIEBREAK_NEGATIV_REWARD,
                    _ => {
                        debug_assert!(false);
                        0
                    }
                }
            }
        }
    }
    eval
};

/// Uses leichtfigur Fortschritt in heuristic
pub static EVAL_2703_1: EvaluationFunction<Gamestate, i32> = |state, team| -> i32 {
    let red_score = state.ambers[0];
    let blue_score = state.ambers[1];

    const WIN_REWARD: i32 = 100000;
    const LOSE_REWARD: i32 = -WIN_REWARD;

    const TIEBREAK_POSITIVE_REWARD: i32 = 50000;
    const TIEBREAK_NEGATIV_REWARD: i32 = -TIEBREAK_POSITIVE_REWARD;
    const TIE_REWARD: i32 = 1000;

    const POINTS_REWARD: i32 = 10000;
    const LEICHTFIGUR_FORTSCHRITT: i32 = 1000;
    const PIECE_REWARD: i32 = 100;

    let mut eval = 0;

    eval += red_score as i32 * POINTS_REWARD;
    eval -= blue_score as i32 * POINTS_REWARD;
    eval += state.board.leichtfigur_fortschritt(team) as i32 * LEICHTFIGUR_FORTSCHRITT;
    eval -= state.board.leichtfigur_fortschritt(team.opponent()) as i32 * LEICHTFIGUR_FORTSCHRITT;
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
                let leicht_figuren =
                    state.board.moewen | state.board.seesterne | state.board.muscheln;
                let mut red_l = leicht_figuren & state.board.red;
                let mut blue_l = leicht_figuren & state.board.blue;
                let wins = Gamestate::draw_winner(
                    *red_l.rotate90_anti_clockwise(),
                    *blue_l.rotate90_clockwise(),
                );
                match wins {
                    1 => TIEBREAK_POSITIVE_REWARD,
                    0 => TIE_REWARD,
                    -1 => TIEBREAK_NEGATIV_REWARD,
                    _ => {
                        debug_assert!(false);
                        0
                    }
                }
            }
        }
        Team::TWO => {
            eval += if blue_score > red_score {
                WIN_REWARD
            } else if blue_score < red_score {
                LOSE_REWARD
            } else {
                let leicht_figuren =
                    state.board.moewen | state.board.seesterne | state.board.muscheln;
                let mut red_l = leicht_figuren & state.board.red;
                let mut blue_l = leicht_figuren & state.board.blue;
                let wins = Gamestate::draw_winner(
                    *blue_l.rotate90_clockwise(),
                    *red_l.rotate90_anti_clockwise(),
                );
                match wins {
                    1 => TIEBREAK_POSITIVE_REWARD,
                    0 => TIE_REWARD,
                    -1 => TIEBREAK_NEGATIV_REWARD,
                    _ => {
                        debug_assert!(false);
                        0
                    }
                }
            }
        }
    }
    eval
};

/// Fixes bug in EVAL_2703_1
pub static EVAL_2703_2: EvaluationFunction<Gamestate, i32> = |state, team| -> i32 {
    let red_score = state.ambers[0];
    let blue_score = state.ambers[1];

    const WIN_REWARD: i32 = 100000;
    const LOSE_REWARD: i32 = -WIN_REWARD;

    const TIEBREAK_POSITIVE_REWARD: i32 = 50000;
    const TIEBREAK_NEGATIV_REWARD: i32 = -TIEBREAK_POSITIVE_REWARD;
    const TIE_REWARD: i32 = 1000;

    const POINTS_REWARD: i32 = 10000;
    const LEICHTFIGUR_FORTSCHRITT: i32 = 1000;
    const PIECE_REWARD: i32 = 100;

    let mut eval = 0;

    eval += red_score as i32 * POINTS_REWARD;
    eval -= blue_score as i32 * POINTS_REWARD;
    eval += state.board.leichtfigur_fortschritt(Team::ONE) as i32 * LEICHTFIGUR_FORTSCHRITT;
    eval -= state.board.leichtfigur_fortschritt(Team::TWO) as i32 * LEICHTFIGUR_FORTSCHRITT;
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
                let leicht_figuren =
                    state.board.moewen | state.board.seesterne | state.board.muscheln;
                let mut red_l = leicht_figuren & state.board.red;
                let mut blue_l = leicht_figuren & state.board.blue;
                let wins = Gamestate::draw_winner(
                    *red_l.rotate90_anti_clockwise(),
                    *blue_l.rotate90_clockwise(),
                );
                match wins {
                    1 => TIEBREAK_POSITIVE_REWARD,
                    0 => TIE_REWARD,
                    -1 => TIEBREAK_NEGATIV_REWARD,
                    _ => {
                        debug_assert!(false);
                        0
                    }
                }
            }
        }
        Team::TWO => {
            eval += if blue_score > red_score {
                WIN_REWARD
            } else if blue_score < red_score {
                LOSE_REWARD
            } else {
                let leicht_figuren =
                    state.board.moewen | state.board.seesterne | state.board.muscheln;
                let mut red_l = leicht_figuren & state.board.red;
                let mut blue_l = leicht_figuren & state.board.blue;
                let wins = Gamestate::draw_winner(
                    *blue_l.rotate90_clockwise(),
                    *red_l.rotate90_anti_clockwise(),
                );
                match wins {
                    1 => TIEBREAK_POSITIVE_REWARD,
                    0 => TIE_REWARD,
                    -1 => TIEBREAK_NEGATIV_REWARD,
                    _ => {
                        debug_assert!(false);
                        0
                    }
                }
            }
        }
    }
    eval
};

/// Leichtfigurfortschritt & double in heuristic
pub static EVAL_2703_3: EvaluationFunction<Gamestate, i32> = |state, team| -> i32 {
    let red_score = state.ambers[0];
    let blue_score = state.ambers[1];

    const WIN_REWARD: i32 = 100000;
    const LOSE_REWARD: i32 = -WIN_REWARD;

    const TIEBREAK_POSITIVE_REWARD: i32 = 50000;
    const TIEBREAK_NEGATIV_REWARD: i32 = -TIEBREAK_POSITIVE_REWARD;
    const TIE_REWARD: i32 = 1000;

    const DOUBLE_PIECE_REWARD: i32 = 1000;

    const POINTS_REWARD: i32 = 10000;
    const LEICHTFIGUR_FORTSCHRITT: i32 = 1000;
    const PIECE_REWARD: i32 = 100;

    let mut eval = 0;

    eval += red_score as i32 * POINTS_REWARD;
    eval -= blue_score as i32 * POINTS_REWARD;
    eval += state.board.leichtfigur_fortschritt(Team::ONE) as i32 * LEICHTFIGUR_FORTSCHRITT;
    eval -= state.board.leichtfigur_fortschritt(Team::TWO) as i32 * LEICHTFIGUR_FORTSCHRITT;
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
                let leicht_figuren =
                    state.board.moewen | state.board.seesterne | state.board.muscheln;
                let mut red_l = leicht_figuren & state.board.red;
                let mut blue_l = leicht_figuren & state.board.blue;
                let wins = Gamestate::draw_winner(
                    *red_l.rotate90_anti_clockwise(),
                    *blue_l.rotate90_clockwise(),
                );
                match wins {
                    1 => TIEBREAK_POSITIVE_REWARD,
                    0 => TIE_REWARD,
                    -1 => TIEBREAK_NEGATIV_REWARD,
                    _ => {
                        debug_assert!(false);
                        0
                    }
                }
            }
        }
        Team::TWO => {
            eval += if blue_score > red_score {
                WIN_REWARD
            } else if blue_score < red_score {
                LOSE_REWARD
            } else {
                let leicht_figuren =
                    state.board.moewen | state.board.seesterne | state.board.muscheln;
                let mut red_l = leicht_figuren & state.board.red;
                let mut blue_l = leicht_figuren & state.board.blue;
                let wins = Gamestate::draw_winner(
                    *blue_l.rotate90_clockwise(),
                    *red_l.rotate90_anti_clockwise(),
                );
                match wins {
                    1 => TIEBREAK_POSITIVE_REWARD,
                    0 => TIE_REWARD,
                    -1 => TIEBREAK_NEGATIV_REWARD,
                    _ => {
                        debug_assert!(false);
                        0
                    }
                }
            }
        }
    }
    eval
};
