use std::fmt::{Display, Formatter};

use rand::prelude::SliceRandom;
use rand::SeedableRng;
use rand_xoshiro::Xoshiro128PlusPlus;
use util::{bit_loop, square_of};
use crate::move_generation::{robbe_lookup_moves, seestern_lookup_moves, muschel_lookup_moves, moewe_lookup_moves};
use thincollections::thin_vec::ThinVec;
use crate::min_max::{IMove, MinMax, Priv};
use crate::board::Board;
use util::fen::FenString;
use util::bitboard::Bitboard;

#[derive(Debug, Copy)]
pub struct Gamestate {
    pub board: Board,
    pub round: u8,
    pub is_maximizing_player: bool,
    pub score: Score, // [max_player | min_player]
}

#[derive(Debug, Copy, Clone)]
pub struct Score {
    pub bytes: [u8; 2],
}

impl Gamestate {
    pub const fn new() -> Self {
        Gamestate {
            board: Board::new(),
            round: 1,
            is_maximizing_player: true,
            score: Score { bytes: [0, 0] },
        }
    }

    pub fn best_move(&self) -> Move {
        self.calculate_best_move(12).unwrap()
    }

    pub fn random_move(&self) -> Move {
        let mut rng = Xoshiro128PlusPlus::seed_from_u64(1);
        let legal_moves = self.legal_moves();
        *legal_moves.choose(&mut rng).unwrap()
    }

    pub fn apply_move(&mut self, game_move: &Move) {
        let points = self.board.apply(game_move);
        self.score.bytes[!self.is_maximizing_player as usize] += points;
    }

    pub fn legal_moves(&self) -> ThinVec<Move> {
        let unoccupied = !self.board.friendly;
        let moewen = self.board.moewen & self.board.friendly;
        let robben = self.board.robben & self.board.friendly;
        let seesterne = self.board.seesterne & self.board.friendly;
        let muscheln = self.board.muscheln & self.board.friendly;

        let mut moves = ThinVec::new();

        bit_loop(moewen.bits, |moewe| {
            let from = square_of(moewe);
            let legal = moewe_lookup_moves(from) & unoccupied;
            let mov = legal.bits;
            bit_loop(mov, |mov_to| {
                let to = square_of(mov_to);
                moves.push(Move { from, to })
            });
        });

        bit_loop(robben.bits, |robbe| {
            let from = square_of(robbe);
            let legal = robbe_lookup_moves(from) & unoccupied;
            let mov = legal.bits;
            bit_loop(mov, |mov_to| {
                let to = square_of(mov_to);
                moves.push(Move { from, to })
            });
        });

        bit_loop(seesterne.bits, |seestern| {
            let from = square_of(seestern);
            let legal = seestern_lookup_moves(from, self.is_maximizing_player) & unoccupied;
            let mov = legal.bits;
            bit_loop(mov, |mov_to| {
                let to = square_of(mov_to);
                moves.push(Move { from, to })
            });
        });

        bit_loop(muscheln.bits, |muschel| {
            let from = square_of(muschel);
            let legal = muschel_lookup_moves(from, self.is_maximizing_player) & unoccupied;
            let mov = legal.bits;
            bit_loop(mov, |mov_to| {
                let to = square_of(mov_to);
                moves.push(Move { from, to })
            });
        });
        return moves;
    }

    /// Calculates points received through reaching the end of the board
    fn calculate_points(&self, bitboard: Bitboard) -> u8 {
        ((bitboard.bits & 0xFF00000000000000 & ((self.is_maximizing_player as u64) * u64::MAX)
            | bitboard.bits & 0xFF & ((!self.is_maximizing_player as u64) * u64::MAX))
            != 0) as u8
    }

    /// Checks whether we won the game
    pub const fn game_result(&self) -> i8 {
        if self.score.bytes[0] >= 2 {
            return 2;
        }
        if self.score.bytes[1] >= 2 {
            return -2;
        }
        if self.round > 60 {
            return 1;
        }
        return 0;
    }

    /// Checks whether the game is over
    pub const fn is_game_over(&self) -> bool {
        self.score.bytes[0] >= 2 || self.score.bytes[1] >= 2 || self.round > 60
    }
}

impl FenString for Gamestate {
    fn to_fen(&self) -> String {
        let board = self.board;

        let mut fen = String::new();

        for rank in (0..8).rev() {
            let mut counter_without = 0;
            let mut append = String::new();

            for file in 0..8 {
                let pos = rank * 8 + file;
                if board.friendly.get_bit(pos) {
                    if counter_without != 0 {
                        append.push(char::from_digit(counter_without as u32, 10).unwrap());
                        counter_without = 0;
                    }
                    if board.muscheln.get_bit(pos) {
                        append.push('H');
                    } else if board.moewen.get_bit(pos) {
                        append.push('M');
                    } else if board.robben.get_bit(pos) {
                        append.push('R');
                    } else if board.seesterne.get_bit(pos) {
                        append.push('S');
                    }
                    if board.double.get_bit(pos) {
                        append.push('*');
                    }
                } else if board.enemy.get_bit(pos) {
                    if counter_without != 0 {
                        append.push(char::from_digit(counter_without as u32, 10).unwrap());
                        counter_without = 0;
                    }
                    if board.muscheln.get_bit(pos) {
                        append.push('h');
                    } else if board.moewen.get_bit(pos) {
                        append.push('m');
                    } else if board.robben.get_bit(pos) {
                        append.push('r');
                    } else if board.seesterne.get_bit(pos) {
                        append.push('s');
                    }
                    if board.double.get_bit(pos) {
                        append.push('*');
                    }
                } else {
                    if board.double.get_bit(pos) {
                        println!("Double\n{}", board.double);
                        println!("Thingy\n{}", board);
                    }
                    counter_without += 1;
                }
            }
            if counter_without != 0 {
                append.push(char::from_digit(counter_without as u32, 10).unwrap())
            }
            fen.push_str(append.as_str());
            fen.push('/');
        }
        fen.pop().unwrap();
        fen.push_str(&*format!(
            " {}/{}",
            self.score.bytes[0], self.score.bytes[1]
        ));
        fen
    }

    fn load_fen() -> Self {
        todo!()
    }
}

impl Clone for Gamestate {
    fn clone(&self) -> Self {
        *self
    }
}

impl Display for Gamestate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_fen())
    }
}

impl MinMax for Gamestate{
    type MoveType = Move;
    type EvalType = i32;

    fn available_moves(&self) -> ThinVec<Self::MoveType> {
        todo!()
    }

    fn apply_move(&mut self, game_move: &Self::MoveType) {
        self.board.apply(game_move);
    }

    fn game_over(&self) -> bool {
        self.is_game_over()
    }

    fn next_player(&mut self) {

    }

    fn evaluate(&self, maximizing_player: bool) -> Self::EvalType {
        todo!()
    }
}


#[derive(Copy, Clone)]
pub struct Move {
    pub from: u8,
    pub to: u8,
}

impl IMove for Move {}




























