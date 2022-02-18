use util::bitboard::Bitboard;
use std::fmt::{Display, Formatter};
use rand::Rng;
use rand::prelude::SliceRandom;
use crate::gamestate::Move;

#[derive(Debug, Copy, Clone)]
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
        }else {
            out.push(' ');
        }

        return out;
    }

    pub fn apply(&mut self, r#move: &Move) -> u8 {

        // clear from position
        self.robben.clear_bit(r#move.from);
        self.muscheln.clear_bit(r#move.from);
        self.seesterne.clear_bit(r#move.from);
        self.moewen.clear_bit(r#move.from);

        self.double.clear_bit(r#move.from);

        // check if we move a friendly or enemy piece
        if self.friendly.get_bit(r#move.from) {
            self.friendly.clear_bit(r#move.from);

            if self.enemy.get_bit(r#move.to) {
                self.enemy.clear_bit(r#move.to);
                self.seesterne.clear_bit(r#move.to);
                self.muscheln.clear_bit(r#move.to);
                self.moewen.clear_bit(r#move.to);
                self.robben.clear_bit(r#move.to);

                if self.double.get_bit(r#move.from) || self.double.get_bit(r#move.to) {
                    self.double.clear_bit(r#move.from);
                    self.double.clear_bit(r#move.to);
                    return 1;
                } else {
                    self.double.set_bit(r#move.to);
                }
            }

            self.friendly.set_bit(r#move.to);
        } else {
            self.enemy.clear_bit(r#move.from);

            if self.friendly.get_bit(r#move.to) {
                self.friendly.clear_bit(r#move.to);
                self.seesterne.clear_bit(r#move.to);
                self.muscheln.clear_bit(r#move.to);
                self.moewen.clear_bit(r#move.to);
                self.robben.clear_bit(r#move.to);

                if self.double.get_bit(r#move.to) {
                    self.double.clear_bit(r#move.from);
                    self.double.clear_bit(r#move.to);
                    return 1;
                } else {
                    self.double.set_bit(r#move.to);
                }
            }

            self.enemy.set_bit(r#move.to);
        }

        // place the piece
        self.robben.set_bit(r#move.to);
        self.muscheln.set_bit(r#move.to);
        self.seesterne.set_bit(r#move.to);
        self.moewen.set_bit(r#move.to);

        return 0;
    }

    fn rotate180(&mut self) {
        self.enemy.rotate180();
        self.friendly.rotate180();
        self.seesterne.rotate180();
        self.muscheln.rotate180();
        self.moewen.rotate180();
        self.robben.rotate180();
        self.double.rotate180();
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
