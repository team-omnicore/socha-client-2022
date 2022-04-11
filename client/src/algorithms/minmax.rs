use crate::algorithms::{Algorithm, EvaluationFunction};
use crate::game::{Gamestate, IGamestate, Move, Team};
use num_traits::{Bounded, Num, NumCast};
use std::fmt::Display;
use crate::for_each_move;

#[derive(Clone)]
pub struct MinMax<E: MinMaxState> {
    max_depth: u8,
    my_team: Team,
    evaluation: fn(&E, Team) -> E::EvalType,
}

pub trait MinMaxState: IGamestate {
    type EvalType: Num + Sized + Copy + NumCast + PartialOrd + Ord + Bounded + Display;

    fn default_evaluation(&self, maximizing_team: Team) -> Self::EvalType;
}

impl<E: MinMaxState> MinMax<E> {
    pub fn new(search_depth: u8, evaluation: EvaluationFunction<E, E::EvalType>) -> Self {
        Self {
            max_depth: search_depth,
            my_team: Team::ONE, //Gets corrected anyway.
            evaluation,
        }
    }

    fn recommend_move(&mut self, state: E, my_team: Team) -> E::MoveType {
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
                E::EvalType::min_value(),
                E::EvalType::max_value(),
            );
            move_value_pairs.push((value, mov));
            false
        });
        let max = move_value_pairs.iter().max_by_key(|pair| pair.0);
        //println!("Value: {}", max.unwrap().0);

        max.unwrap().1.clone()
    }

    fn min_max(
        &self,
        state: E,
        depth: u8,
        team: Team,
        mut alpha: E::EvalType,
        mut beta: E::EvalType,
    ) -> E::EvalType {
        if depth == 0 || state.game_over() {
            return (self.evaluation)(&state, self.my_team);
        }

        let is_maximizing = team == self.my_team;

        return if is_maximizing {
            //Maximizing player (Client player)
            let mut value = E::EvalType::min_value();

            state.for_each_move(team, &mut |mov| {
                let mut child = state.clone();
                child.apply_move(&mov);
                child.next_player();

                value = E::EvalType::max(
                    value,
                    self.min_max(child, depth - 1, team.opponent(), alpha, beta),
                );
                alpha = E::EvalType::max(alpha, value);

                if value >= beta {
                    return true; //* β-cutoff *
                }
                false
            });
            value
        } else {
            //Minimizing player (Enemy player)
            let mut value = E::EvalType::max_value();

            state.for_each_move(team, &mut |mov| {
                let mut child = state.clone();
                child.apply_move(&mov);
                child.next_player();

                value = E::EvalType::min(
                    value,
                    self.min_max(child, depth - 1, team.opponent(), alpha, beta),
                );
                beta = E::EvalType::min(alpha, value);

                if value <= alpha {
                    return true; //* α-cutoff *
                }
                false
            });
            value
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

    fn default_evaluation(&self, maximizing_team: Team) -> Self::EvalType {
        let red_score = self.ambers[0] as i32;
        let blue_score = self.ambers[1] as i32;

        const WIN_REWARD: i32 = 2000;
        const LOSE_REWARD: i32 = -WIN_REWARD;

        const TIEBREAK_POSITIVE_REWARD: i32 = 100;
        const TIEBREAK_NEGATIV_REWARD: i32 = -TIEBREAK_POSITIVE_REWARD;
        const TIE_REWARD: i32 = 50;

        const POINTS_REWARD: i32 = 230;

        let mut eval: i32 = 0;

        match maximizing_team {
            Team::ONE => {
                eval -= self.board.blue.bits.count_ones() as i32;
                eval += self.count_moves(Team::ONE) as i32;
                eval -= self.count_moves(Team::TWO) as i32;
                eval += POINTS_REWARD * red_score * red_score;
                eval -= POINTS_REWARD * blue_score * blue_score;

                if self.game_over() {
                    eval += if red_score > blue_score {
                        WIN_REWARD
                    } else if red_score < blue_score {
                        LOSE_REWARD
                    } else {
                        let leicht_figuren =
                            self.board.moewen | self.board.seesterne | self.board.muscheln;
                        let red_l = *(leicht_figuren & self.board.red).rotate90_anti_clockwise();
                        let blue_l = *(leicht_figuren & self.board.blue).rotate90_clockwise();

                        let wins = Gamestate::draw_winner(red_l, blue_l);
                        match wins {
                            -1 => TIEBREAK_NEGATIV_REWARD,
                            1 => TIEBREAK_POSITIVE_REWARD,
                            0 => TIE_REWARD,
                            _ => {
                                debug_assert!(false);
                                0
                            }
                        }
                    }
                }
            }
            Team::TWO => {
                eval -= self.board.red.bits.count_ones() as i32;
                eval += self.count_moves(Team::TWO) as i32;
                eval -= self.count_moves(Team::ONE) as i32;
                eval += POINTS_REWARD * blue_score * blue_score;
                eval -= POINTS_REWARD * red_score * red_score;

                if self.game_over() {
                    eval += if blue_score > red_score {
                        WIN_REWARD
                    } else if blue_score < red_score {
                        LOSE_REWARD
                    } else {
                        let leicht_figuren =
                            self.board.moewen | self.board.seesterne | self.board.muscheln;
                        let red_l = *(leicht_figuren & self.board.red).rotate90_anti_clockwise();
                        let blue_l = *(leicht_figuren & self.board.blue).rotate90_clockwise();

                        let wins = Gamestate::draw_winner(blue_l, red_l);
                        match wins {
                            -1 => TIEBREAK_NEGATIV_REWARD,
                            1 => TIEBREAK_POSITIVE_REWARD,
                            0 => TIE_REWARD,
                            _ => {
                                debug_assert!(false);
                                0
                            }
                        }
                    }
                }
            }
        }
        eval
    }
}
