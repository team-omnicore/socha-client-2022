use std::fmt::{Display, Formatter};
use std::ops::Index;

use crate::board::Board;
use crate::game_move::Move;
use crate::min_max::{MinMax, Priv};
use crate::move_generation::{
    moewe_lookup_moves, muschel_lookup_moves, robbe_lookup_moves, seestern_lookup_moves,
};
use crate::piece::PieceType;
use crate::score::Score;
use crate::team::Team;
use crate::team::Team::{BLUE, RED};
use std::time::SystemTime;
use regex::Error;
use thincollections::thin_vec::ThinVec;
use util::bitboard::Bitboard;
use util::{bit_loop, square_of};
use crate::fen::{FEN_REGEX, FenString};

#[derive(Debug, Copy)]
pub struct Gamestate {
    pub board: Board,
    pub round: u8,
    pub is_maximizing_player: bool,
    pub score: Score, // [client_player | other_player]
}

static mut AVERAGE_SIZE: f64 = 0f64;
static mut COUNT: f64 = 0f64;

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
        let start = SystemTime::now();
        let best_move = self.calculate_best_move(6).unwrap();
        let duration = SystemTime::now().duration_since(start);
        println!("Calculation took {:?}", duration.unwrap());
        unsafe {
            println!("Average move count: {}", AVERAGE_SIZE);
            AVERAGE_SIZE = 0f64;
            COUNT = 0f64;
        };
        best_move
    }

    pub fn legal_moves(&self) -> ThinVec<Move> {
        let unoccupied = !self.board.friendly;
        let moewen = self.board.moewen & self.board.friendly;
        let robben = self.board.robben & self.board.friendly;
        let seesterne = self.board.seesterne & self.board.friendly;
        let muscheln = self.board.muscheln & self.board.friendly;

        let mut moves = ThinVec::with_capacity(25);

        bit_loop(moewen.bits, |moewe| {
            let from = square_of(moewe);
            let legal = moewe_lookup_moves(from) & unoccupied;
            bit_loop(legal.bits, |mov_to| {
                let to = square_of(mov_to);
                moves.push(Move {
                    from,
                    to,
                    piece: PieceType::MOEWE,
                })
            });
        });

        bit_loop(robben.bits, |robbe| {
            let from = square_of(robbe);
            let legal = robbe_lookup_moves(from) & unoccupied;
            bit_loop(legal.bits, |mov_to| {
                let to = square_of(mov_to);
                moves.push(Move {
                    from,
                    to,
                    piece: PieceType::ROBBE,
                })
            });
        });

        bit_loop(seesterne.bits, |seestern| {
            let from = square_of(seestern);
            let legal = seestern_lookup_moves(from, self.is_maximizing_player) & unoccupied;
            bit_loop(legal.bits, |mov_to| {
                let to = square_of(mov_to);
                moves.push(Move {
                    from,
                    to,
                    piece: PieceType::SEESTERN,
                })
            });
        });

        bit_loop(muscheln.bits, |muschel| {
            let from = square_of(muschel);
            let legal = muschel_lookup_moves(from, self.is_maximizing_player) & unoccupied;
            bit_loop(legal.bits, |mov_to| {
                let to = square_of(mov_to);
                moves.push(Move {
                    from,
                    to,
                    piece: PieceType::MUSCHEL,
                })
            });
        });

        unsafe {
            AVERAGE_SIZE += 1f64 / (COUNT + 1f64) * (moves.len() as f64 - AVERAGE_SIZE);
            COUNT += 1f64;
        }

        return moves;
    }

    pub fn player_to_move(&self) -> Team {
        let round_even = self.round & 0x1 == 0;
        if round_even {
            Team::RED
        } else {
            Team::BLUE
        }
    }

    fn calculate_points(&self, bitboard: Bitboard) -> u8 {
        ((bitboard.bits & 0xFF00000000000000 & ((self.is_maximizing_player as u64) * u64::MAX)
            | bitboard.bits & 0xFF & ((!self.is_maximizing_player as u64) * u64::MAX))
            != 0) as u8
    }
}

impl FenString for Gamestate {
    fn to_fen(&self) -> String {
        let mut board = self.board.clone();

        let next_player = self.player_to_move();
        let mut score = self.score;

        if (next_player == BLUE && !self.is_maximizing_player) || (next_player == RED  && !self.is_maximizing_player){
            board.rotate180();
            board.friendly.swap_with(&mut board.enemy);
            score.bytes.swap(0,1);
        }

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
            " {} {}/{}",
            self.round,
            score.bytes[0],
            score.bytes[1],
        ));
        fen
    }

    fn load_fen(fen: &str, serialize_for:Team) -> Result<Self, regex::Error> {
        if !FEN_REGEX.is_match(fen) {
            return Err(Error::Syntax(String::from("Failed to deserialize FEN- string does not match specification")))
        }

        let captures = FEN_REGEX.captures(fen).unwrap();

        let mut board = Board::new();
        for i in 2..10 {
            let cap = captures.index(i);
            let mut board_index = 8;
            for j in 0..cap.len() {
                let c = cap.chars().nth(j).unwrap();
                if c == '*'{
                    continue;
                }
                if c.is_ascii_digit() {
                    let digit = c.to_digit(10).unwrap() as i8;
                    board_index -= digit;
                    if board_index < 0 {
                        return Err(Error::Syntax(format!("Failed to deserialize FEN - too many pieces in rank {}", i-1)))
                    }
                } else {
                    board_index -= 1;
                    if board_index < 0 {
                        return Err(Error::Syntax(format!("Failed to deserialize FEN - too many pieces in rank {}", i-1)))
                    }
                    let piece = PieceType::from(c.to_ascii_lowercase());
                    let stacked = if let Some(c) = cap.chars().nth(j + 1) {
                        c == '*'
                    } else {
                        false
                    };
                    board.set_piece((8i8 * (9-i) as i8 + (8-board_index)-1) as u8, piece, c.is_ascii_uppercase(), stacked)
                }
            }
            if board_index != 0 {
                return Err(Error::Syntax(format!("Failed to deserialize FEN - mismatched piece count in rank {}", i-1)))
            }
        }

        let round = captures.name("round").expect("Failed to deserialize FEN - failed parsing round").as_str().parse::<u8>().unwrap();

        let mut score = Score {
            bytes: [
                captures.name("pt_red").unwrap().as_str().parse().unwrap(),
                captures.name("pt_blu").unwrap().as_str().parse().unwrap(),
            ]
        };

        if serialize_for == BLUE {
            println!("Rotated");
            board.rotate180();
            board.friendly.swap_with(&mut board.enemy);
            score.bytes.swap(0,1);
        }

        Ok(Self {
            board,
            round,
            is_maximizing_player: true,
            score,
        })
    }
}

impl Clone for Gamestate {
    fn clone(&self) -> Self {
        Self {
            board: self.board,
            round: self.round,
            is_maximizing_player: self.is_maximizing_player,
            score: self.score,
        }
    }
}

impl Display for Gamestate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_fen())
    }
}

impl MinMax for Gamestate {
    type MoveType = Move;
    type EvalType = i32;

    fn available_moves(&self) -> ThinVec<Self::MoveType> {
        self.legal_moves()
    }

    fn for_each_legal_move<F: FnMut(Self::MoveType) -> bool>(&self, f: &mut F) {
        let unoccupied = !self.board.friendly;
        let moewen = self.board.moewen & self.board.friendly;
        let robben = self.board.robben & self.board.friendly;
        let seesterne = self.board.seesterne & self.board.friendly;
        let muscheln = self.board.muscheln & self.board.friendly;

        bit_loop(moewen.bits, |moewe| {
            let from = square_of(moewe);
            let legal = moewe_lookup_moves(from) & unoccupied;
            let mov = legal.bits;
            bit_loop(mov, |mov_to| {
                let to = square_of(mov_to);
                if f(Move {
                    from,
                    to,
                    piece: PieceType::MOEWE,
                }) {
                    return;
                }
            });
        });

        bit_loop(robben.bits, |robbe| {
            let from = square_of(robbe);
            let legal = robbe_lookup_moves(from) & unoccupied;
            let mov = legal.bits;
            bit_loop(mov, |mov_to| {
                let to = square_of(mov_to);
                if f(Move {
                    from,
                    to,
                    piece: PieceType::ROBBE,
                }) {
                    return;
                }
            });
        });

        bit_loop(seesterne.bits, |seestern| {
            let from = square_of(seestern);
            let legal = seestern_lookup_moves(from, self.is_maximizing_player) & unoccupied;
            let mov = legal.bits;
            bit_loop(mov, |mov_to| {
                let to = square_of(mov_to);
                if f(Move {
                    from,
                    to,
                    piece: PieceType::SEESTERN,
                }) {
                    return;
                }
            });
        });

        bit_loop(muscheln.bits, |muschel| {
            let from = square_of(muschel);
            let legal = muschel_lookup_moves(from, self.is_maximizing_player) & unoccupied;
            let mov = legal.bits;
            bit_loop(mov, |mov_to| {
                let to = square_of(mov_to);
                if f(Move {
                    from,
                    to,
                    piece: PieceType::MUSCHEL,
                }) {
                    return;
                }
            });
        });
    }

    fn apply_move(&mut self, game_move: &Self::MoveType) {
        let points = self.board.apply(game_move); //Apply the move to the board, return the points gotten by jumping on other pieces
        self.score.bytes[!self.is_maximizing_player as usize] += points;

        self.score.bytes[!self.is_maximizing_player as usize] += //Calculate the points gotten through reaching the end of the board
            self.calculate_points(Bitboard::from(1 << game_move.to))
    }

    fn game_over(&self) -> bool {
        self.score.bytes[0] >= 2 || self.score.bytes[1] >= 2 || self.round > 60
    }

    fn next_player(&mut self) {
        self.is_maximizing_player = !self.is_maximizing_player;
        self.board.friendly.swap_with(&mut self.board.enemy);
    }

    fn evaluate(&self) -> Self::EvalType {
        let client_score = self.score.bytes[0];
        let enemy_score = self.score.bytes[1];

        const POSITIV_REWARD: i32 = 10;
        const NEGATIV_REWARD: i32 = -POSITIV_REWARD;
        const TIEBREAK_POSITIVE_REWARD: i32 = 5;
        const TIEBREAK_NEGATIV_REWARD: i32 = -POSITIV_REWARD;
        const TIE_REWARD: i32 = 1;

        let out = if client_score > enemy_score {
            POSITIV_REWARD
        } else if client_score < enemy_score {
            NEGATIV_REWARD
        } else {
            //TIE_REWARD
            let leicht_figuren = self.board.moewen | self.board.seesterne | self.board.muscheln;
            let friendly_l = leicht_figuren & self.board.friendly;
            let enemy_l = (leicht_figuren & self.board.enemy).rotate180();

            if friendly_l.bits > enemy_l.bits {
                TIEBREAK_POSITIVE_REWARD
            } else if friendly_l.bits < enemy_l.bits {
                TIEBREAK_NEGATIV_REWARD
            } else {
                TIE_REWARD
            }
        };
        out
    }
}
