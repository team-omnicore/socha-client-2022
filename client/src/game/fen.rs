use crate::game::{Board, Gamestate, Piece, PieceType, ShortForm, Team, Tile};
use lazy_static::lazy_static;
use regex::Regex;
use socha_client_2022::util::{SCError, SCResult};

pub trait Fen: Sized {
    type Err;
    fn to_fen(&self) -> String;
    fn load_fen(fen: &str) -> Result<Self, Self::Err>;
}

static FEN_REGEX_STRING: &str = r"^(?P<pieces>(?P<r1>(?:[1-8]|[msrhMSRH]\*?){1,8})/(?P<r2>(?:[1-8]|[msrhMSRH]\*?){1,8})/(?P<r3>(?:[1-8]|[msrhMSRH]\*?){1,8})/(?P<r4>(?:[1-8]|[msrhMSRH]\*?){1,8})/(?P<r5>(?:[1-8]|[msrhMSRH]\*?){1,8})/(?P<r6>(?:[1-8]|[msrhMSRH]\*?){1,8})/(?P<r7>(?:[1-8]|[msrhMSRH]\*?){1,8})/(?P<r8>(?:[1-8]|[msrhMSRH]\*?){1,8})) (?P<round>(?:[1-5]?[0-9]|60)) (?P<points>(?P<pt_red>[0-3])/(?P<pt_blu>[0-3]))$";

lazy_static! {
    pub static ref FEN_REGEX: Regex = Regex::new(FEN_REGEX_STRING).unwrap();
}

impl Fen for Gamestate {
    type Err = SCError;

    fn to_fen(&self) -> String {
        let mut fen = String::new();

        let iter = &mut self.board.iter_tiles();
        let v: Vec<_> = iter.collect();

        for row in v.rchunks(8) {
            let mut empty = 0;
            for tile in row.iter() {
                match tile {
                    Tile::Empty => empty += 1,
                    Tile::Piece(p) => {
                        if empty > 0 {
                            fen.push(char::from_digit(empty, 10).unwrap());
                            empty = 0;
                        }
                        fen.push_str(&*p.to_short_form());
                    }
                }
            }
            if empty > 0 {
                fen.push(char::from_digit(empty, 10).unwrap());
            }
            fen.push('/');
        }
        fen.pop().unwrap();
        fen.push_str(&*format!(
            " {} {}/{}",
            self.turn, self.ambers[0], self.ambers[1]
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

        for i in 0..8 {
            let row = captures.get(i + 2).unwrap().as_str();
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

                board.set_piece(8 * (7 - i) as u8 + pos_x as u8, piece);
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

        let state = Gamestate::new_with(board, round, [points_red, points_blu]);
        Ok(state)
    }
}
