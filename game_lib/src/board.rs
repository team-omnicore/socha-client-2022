use crate::game_move::Move;
use crate::piece::PieceType;
use rand::prelude::SliceRandom;
use rand::Rng;
use std::fmt::{Display, Formatter};
use util::bitboard::Bitboard;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Board {
    pub enemy: Bitboard,
    pub friendly: Bitboard,
    pub seesterne: Bitboard,
    pub muscheln: Bitboard,
    pub moewen: Bitboard,
    pub robben: Bitboard,
    pub double: Bitboard,
}

impl Board {
    pub const fn new() -> Self {
        Board {
            enemy: Bitboard::new(),
            friendly: Bitboard::new(),
            seesterne: Bitboard::new(),
            muscheln: Bitboard::new(),
            moewen: Bitboard::new(),
            robben: Bitboard::new(),
            double: Bitboard::new(),
        }
    }

    pub fn new_random<T: Rng>(rng: &mut T) -> Self {
        let enemy = Bitboard::from(0xFF00000000000000u64);
        let friendly = Bitboard::from(0xFFu64);
        let double = Bitboard::new();

        let mut start_positions = vec![
            0x80u64, 0x40u64, 0x20u64, 0x10u64, 0x8u64, 0x4u64, 0x2u64, 0x1u64,
        ];
        start_positions.shuffle(rng);

        let muscheln = Bitboard::from(start_positions[0] | start_positions[1])
            | Bitboard::from(start_positions[0] | start_positions[1]).rotate180();
        let moewen = Bitboard::from(start_positions[2] | start_positions[3])
            | Bitboard::from(start_positions[2] | start_positions[3]).rotate180();
        let seesterne = Bitboard::from(start_positions[4] | start_positions[5])
            | Bitboard::from(start_positions[4] | start_positions[5]).rotate180();
        let robben = Bitboard::from(start_positions[6] | start_positions[7])
            | Bitboard::from(start_positions[6] | start_positions[7]).rotate180();

        Board {
            enemy,
            friendly,
            seesterne,
            muscheln,
            moewen,
            robben,
            double,
        }
    }

    fn piece_at(&self, pos: u8) -> String {
        let mut x = if self.robben.get_bit(pos) {
            'r'
        } else if self.muscheln.get_bit(pos) {
            'h'
        } else if self.moewen.get_bit(pos) {
            'm'
        } else if self.seesterne.get_bit(pos) {
            's'
        } else {
            '-'
        };
        if self.friendly.get_bit(pos) {
            x = x.to_ascii_uppercase();
        }

        let mut out = String::from(x);

        if self.double.get_bit(pos) {
            out.push('*');
        } else {
            out.push(' ');
        }

        return out;
    }

    pub fn apply(&mut self, game_move: &Move) -> u8 {
        let old_piece = Bitboard::from(1 << game_move.from);
        let new_piece = Bitboard::from(1 << game_move.to);

        //Calculate whether double
        let double = {
            let old_was_double = self.double.get_bit(game_move.from);
            let new_has_piece = self.enemy.get_bit(game_move.to);
            let new_is_double = self.double.get_bit(game_move.to);
            (old_was_double ^ new_has_piece) & !new_is_double
        };

        //Set double
        self.double &= !new_piece;
        self.double |= Bitboard::from(new_piece.bits * double as u64);

        let mut points = 0;
        match game_move.piece {
            PieceType::ROBBE => {
                //Clear old piece
                self.robben &= !old_piece;
                self.friendly &= !old_piece;

                //Set new piece
                self.robben |= new_piece;
                self.friendly |= new_piece;

                //If enemy overlaps with new piece, clear that enemy piece.
                if (self.enemy & new_piece).bits != 0 {
                    self.enemy &= !new_piece;
                    self.seesterne &= !new_piece;
                    self.muscheln &= !new_piece;
                    self.moewen &= !new_piece;

                    self.friendly &= !new_piece | self.double;
                    self.robben &= !new_piece | self.double;
                    points = 1;
                }
            }
            PieceType::MUSCHEL => {
                //Clear old piece
                self.muscheln &= !old_piece;
                self.friendly &= !old_piece;

                //Set new piece
                self.muscheln |= new_piece;
                self.friendly |= new_piece;

                //If enemy overlaps with new piece, clear that enemy piece.
                if (self.enemy & new_piece).bits != 0 {
                    self.enemy &= !new_piece;
                    self.seesterne &= !new_piece;
                    self.robben &= !new_piece;
                    self.moewen &= !new_piece;

                    self.friendly &= !new_piece | self.double;
                    self.muscheln &= !new_piece | self.double;
                    points = 1;
                }
            }
            PieceType::SEESTERN => {
                //Clear old piece
                self.seesterne &= !old_piece;
                self.friendly &= !old_piece;

                //Set new piece
                self.seesterne |= new_piece;
                self.friendly |= new_piece;

                //If enemy overlaps with new piece, clear that enemy piece.
                if (self.enemy & new_piece).bits != 0 {
                    self.enemy &= !new_piece;
                    self.muscheln &= !new_piece;
                    self.robben &= !new_piece;
                    self.moewen &= !new_piece;

                    self.friendly &= !new_piece | self.double;
                    self.seesterne &= !new_piece | self.double;
                    points = 1;
                }
            }
            PieceType::MOEWE => {
                //Clear old piece
                self.moewen &= !old_piece;
                self.friendly &= !old_piece;

                //Set new piece
                self.moewen |= new_piece;
                self.friendly |= new_piece;

                //If enemy overlaps with new piece, clear that enemy piece.
                if (self.enemy & new_piece).bits != 0 {
                    self.enemy &= !new_piece;
                    self.muscheln &= !new_piece;
                    self.robben &= !new_piece;
                    self.seesterne &= !new_piece;

                    self.friendly &= !new_piece | self.double;
                    self.moewen &= !new_piece | self.double;
                    points = 1;
                }
            }
        }
        points
    }

    pub fn rotate180(&mut self) {
        self.enemy.rotate180();
        self.friendly.rotate180();
        self.seesterne.rotate180();
        self.muscheln.rotate180();
        self.moewen.rotate180();
        self.robben.rotate180();
        self.double.rotate180();
    }

    pub fn set_piece(&mut self, pos: u8, piece: PieceType, friendly: bool, is_stacked: bool) {
        match piece {
            PieceType::ROBBE => self.robben.set_bit(pos),
            PieceType::MUSCHEL => self.muscheln.set_bit(pos),
            PieceType::SEESTERN => self.seesterne.set_bit(pos),
            PieceType::MOEWE => self.moewen.set_bit(pos),
        };

        if friendly {
            self.friendly.set_bit(pos);
        } else {
            self.enemy.set_bit(pos);
        }

        if is_stacked {
            self.double.set_bit(pos);
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut out: String = String::from("╔══════════════════════════╗\n");
        let mut index_min = 56 + 8;
        let mut index_max = 64 + 8;
        for _ in 0..8 {
            index_max -= 8;
            index_min -= 8;
            out.push_str("║  ");
            for j in index_min..index_max {
                out.push_str(&*self.piece_at(j));
                out.push_str(" ");
            }
            out.push_str("║\n");
        }
        out.push_str("╚══════════════════════════╝");
        write!(f, "{}", out)
    }
}
