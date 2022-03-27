use crate::game::fen::FEN_REGEX;
use crate::game::{Bitboard, Board, Fen, IGamestate, Move, Piece, PieceType, ShortForm, Team};
use socha_client_2022::util::{SCError, SCResult};
use std::fmt::{Display, Formatter};
use thincollections::thin_vec::ThinVec;

#[derive(Debug, Copy, PartialEq, Clone)]
pub struct Gamestate {
    pub board: Board,
    pub round: u8,
    pub current_player: Team,
    pub ambers: [u8; 2], //[ONE | TWO]
}

impl Gamestate {
    #[inline]
    pub const fn new(board: Board) -> Self {
        Gamestate {
            board,
            round: 1,
            current_player: Team::ONE,
            ambers: [0, 0],
        }
    }

    #[inline]
    pub fn winner(&self) -> Option<Team> {
        if !self.game_over() {
            None
        } else {
            let red_score = self.ambers[0];
            let blue_score = self.ambers[1];

            if red_score > blue_score {
                Some(Team::ONE)
            } else if red_score < blue_score {
                Some(Team::TWO)
            } else {
                let leicht_figuren = self.board.moewen | self.board.seesterne | self.board.muscheln;
                let red_l = *(leicht_figuren & self.board.red).rotate90_anti_clockwise();
                let blue_l = *(leicht_figuren & self.board.blue).rotate90_clockwise();

                let wins = Gamestate::draw_winner(red_l, blue_l);
                match wins {
                    1 => Some(Team::ONE),
                    -1 => Some(Team::TWO),
                    0 => None,
                    _ => {
                        debug_assert!(false);
                        None
                    }
                }
            }
        }
    }

    /// Calculates the winner in case of an amber draw. <br>
    ///
    /// **Returns:**
    /// - 1 if player a wins
    /// - 0 if absolute draw
    /// - -1 if player b wins
    #[inline]
    pub fn draw_winner(a_leicht: Bitboard, b_leicht: Bitboard) -> i32 {
        let bytes_a = a_leicht.bits.to_be_bytes();
        let bytes_b = b_leicht.bits.to_be_bytes();
        for i in 0..8 {
            //maybe could change index to 7
            if bytes_a[i].count_ones() > bytes_b[i].count_ones() {
                return 1;
            } else if bytes_a[i].count_ones() < bytes_b[i].count_ones() {
                return -1;
            }
        }
        return 0;
    }
}

impl IGamestate for Gamestate {
    type MoveType = Move;

    #[inline]
    fn available_moves(&self, team: Team) -> ThinVec<Self::MoveType> {
        self.board.available_moves(team)
    }

    #[inline]
    fn count_moves(&self, team: Team) -> u8 {
        self.board.count_moves(team)
    }

    #[inline]
    fn for_each_move<F: FnMut(Self::MoveType) -> bool>(&self, team: Team, f: &mut F) {
        self.board.for_each_move(team, f)
    }

    #[inline]
    fn apply_move(&mut self, game_move: &Self::MoveType) {
        let points = self.board.apply_move(game_move, self.current_player); //Apply the move to the board, return the points gotten by jumping on other pieces
        self.ambers[self.current_player as usize] += points;
    }

    #[inline]
    fn game_over(&self) -> bool {
        (self.round % 2 == 0 && (self.ambers[0] >= 2 || self.ambers[1] >= 2)) || self.round >= 60
    }

    #[inline]
    fn next_player(&mut self) {
        self.current_player = self.current_player.opponent();
        self.round += 1;
    }
}

impl Fen for Gamestate {
    type Err = SCError;

    fn to_fen(&self) -> String {
        let board = self.board;
        let mut fen = String::new();

        for rank in (0..8).rev() {
            let mut counter_without = 0;

            for file in 0..8 {
                let pos = rank * 8 + file;
                let piece = board.piece_at(pos);
                if piece.is_some() {
                    if counter_without != 0 {
                        fen.push(char::from_digit(counter_without as u32, 10).unwrap());
                        counter_without = 0;
                    }
                    fen.push_str(&*format!("{}", piece.unwrap().to_short_form()));
                } else {
                    counter_without += 1;
                }
            }
            if counter_without != 0 {
                fen.push(char::from_digit(counter_without as u32, 10).unwrap());
            }
            fen.push('/');
        }
        fen.pop().unwrap();
        fen.push_str(&*format!(
            " {} {}/{}",
            self.round, self.ambers[0], self.ambers[1],
        ));
        fen
    }

    fn load_fen(fen: &str) -> SCResult<Self> {
        if !FEN_REGEX.is_match(fen) {
            return Err(SCError::Custom(format!(
                "Input does not match FEN Specification: {}",
                fen
            )));
        }

        let captures = FEN_REGEX.captures(fen).unwrap();

        let mut board = Board::empty();

        for i in 2..10 {
            let row = captures.get(i).unwrap().as_str();
            let mut pos_x = 0;
            for j in 0..row.len() {
                let piece = row.chars().nth(j).unwrap();

                if piece.is_digit(10) {
                    pos_x += piece.to_digit(10).unwrap();
                    continue;
                }

                let piece_type = PieceType::from_short_form(&piece.to_ascii_lowercase()).unwrap();

                let next = row.chars().nth(j + 1);
                let double = next.is_some() && next.unwrap() == '*';

                let piece = Piece {
                    piece_type,
                    team: if piece.is_uppercase() {
                        Team::ONE
                    } else {
                        Team::TWO
                    },
                    stacked: double,
                };

                board.set_piece(((7 - (i as u8 - 2u8)) * 8u8) + pos_x as u8, piece);
                pos_x += 1;
            }
        }

        let round = captures
            .name("round")
            .unwrap()
            .as_str()
            .parse::<u8>()
            .unwrap();
        let points_red = captures
            .name("pt_red")
            .unwrap()
            .as_str()
            .parse::<u8>()
            .unwrap();
        let points_blu = captures
            .name("pt_blu")
            .unwrap()
            .as_str()
            .parse::<u8>()
            .unwrap();

        let state = Gamestate {
            board,
            round,
            current_player: if round % 2 == 0 { Team::TWO } else { Team::ONE },
            ambers: [points_red, points_blu],
        };

        Ok(state)
    }
}

impl Display for Gamestate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_fen())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_xoshiro::Xoshiro128Plus;

    #[test]
    fn test_write_fen() {
        let mut rng = Xoshiro128Plus::seed_from_u64(2);
        let board = Board::new_random(&mut rng);
        let mut gamestate = Gamestate {
            board,
            round: 1,
            current_player: Team::ONE,
            ambers: [0, 0],
        };

        assert_eq!(gamestate.to_fen(), "hrrmssmh/8/8/8/8/8/8/HMSSMRRH 1 0/0");

        gamestate.round = 25;
        assert_eq!(gamestate.to_fen(), "hrrmssmh/8/8/8/8/8/8/HMSSMRRH 25 0/0");

        gamestate.ambers[0] = 2;
        gamestate.ambers[1] = 1;
        assert_eq!(gamestate.to_fen(), "hrrmssmh/8/8/8/8/8/8/HMSSMRRH 25 2/1");
    }

    #[test]
    fn test_read_fen() {
        let mut rng = Xoshiro128Plus::seed_from_u64(2);
        let board = Board::new_random(&mut rng);
        let should_be = Gamestate {
            board,
            round: 8,
            current_player: Team::TWO,
            ambers: [2, 3],
        };

        assert_eq!(
            should_be,
            Gamestate::load_fen("hrrmssmh/8/8/8/8/8/8/HMSSMRRH 8 2/3").unwrap()
        );
        assert_eq!(
            should_be,
            Gamestate::load_fen(should_be.to_fen().as_str()).unwrap()
        );
    }

    #[test]
    fn test_points_system() {
        let mut rng = Xoshiro128Plus::seed_from_u64(2);
        let board = Board::new_random(&mut rng);
        let mut gamestate = Gamestate {
            board,
            round: 1,
            current_player: Team::ONE,
            ambers: [0, 0],
        };

        let m = Move {
            from: 0,
            to: 6,
            piece: PieceType::Herzmuschel,
        };
        gamestate.board.double.set_bit(0);
        gamestate.apply_move(&m);
        //println!("{}", gamestate.board);
        //println!("{:?}", gamestate.ambers);

        let m = Move {
            from: 6,
            to: 7,
            piece: PieceType::Herzmuschel,
        };
        gamestate.apply_move(&m);
        //println!("{}", gamestate.board);
        //println!("{:?}", gamestate.ambers);

        gamestate.next_player();

        let m = Move {
            from: 63,
            to: 17,
            piece: PieceType::Herzmuschel,
        };
        gamestate.apply_move(&m);
        //println!("{}", gamestate.board);
        //println!("{:?}", gamestate.ambers);

        let m = Move {
            from: 17,
            to: 16,
            piece: PieceType::Herzmuschel,
        };
        gamestate.apply_move(&m);
        //println!("{}", gamestate.board);
        //println!("{:?}", gamestate.ambers);
    }
}
