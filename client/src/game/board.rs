use std::fmt::{Display, Formatter};

use rand::prelude::SliceRandom;
use rand::Rng;
use thincollections::thin_vec::ThinVec;

use crate::bitboard;
use crate::game::{
    moewe_lookup_moves, muschel_lookup_moves, robbe_lookup_moves, seestern_lookup_moves, Bitboard,
    Move, Piece, PieceType, ShortForm, Team,
};
use crate::utils::{bit_loop, square_of};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Board {
    pub blue: Bitboard,
    pub red: Bitboard,
    pub seesterne: Bitboard,
    pub muscheln: Bitboard,
    pub moewen: Bitboard,
    pub robben: Bitboard,
    pub double: Bitboard,
}

impl Board {
    #[inline]
    pub const fn empty() -> Self {
        Board {
            blue: bitboard!(),
            red: bitboard!(),
            seesterne: bitboard!(),
            muscheln: bitboard!(),
            moewen: bitboard!(),
            robben: bitboard!(),
            double: bitboard!(),
        }
    }

    #[inline]
    pub fn new_random<T: Rng>(rng: &mut T) -> Self {
        let blue = bitboard!(0x8080808080808080);
        let red = bitboard!(0x101010101010101);
        let double = bitboard!();

        let mut start_positions = vec![
            0x100000000000000,
            0x1000000000000,
            0x10000000000,
            0x100000000,
            0x1000000,
            0x10000,
            0x100,
            0x1,
        ];
        start_positions.shuffle(rng);

        let muscheln = bitboard!(start_positions[0] | start_positions[1])
            | *bitboard!(start_positions[0] | start_positions[1]).rotate180();
        let moewen = bitboard!(start_positions[2] | start_positions[3])
            | *bitboard!(start_positions[2] | start_positions[3]).rotate180();
        let seesterne = bitboard!(start_positions[4] | start_positions[5])
            | *bitboard!(start_positions[4] | start_positions[5]).rotate180();
        let robben = bitboard!(start_positions[6] | start_positions[7])
            | *bitboard!(start_positions[6] | start_positions[7]).rotate180();

        Board {
            blue,
            red,
            seesterne,
            muscheln,
            moewen,
            robben,
            double,
        }
    }

    #[inline]
    pub fn piece_at(&self, pos: u8) -> Option<Piece> {
        let piece_type = if self.robben.get_bit(pos) {
            PieceType::Robbe
        } else if self.muscheln.get_bit(pos) {
            PieceType::Herzmuschel
        } else if self.moewen.get_bit(pos) {
            PieceType::Moewe
        } else if self.seesterne.get_bit(pos) {
            PieceType::Seestern
        } else {
            return None;
        };

        let stacked = self.double.get_bit(pos);
        let team = if self.red.get_bit(pos) {
            Team::ONE
        } else {
            Team::TWO
        };

        Some(Piece {
            piece_type,
            team,
            stacked,
        })
    }

    /// Calculates the points through reaching the end of the board with
    /// light game piece (Moewe, Seestern, Muschel).
    #[inline]
    fn on_finish_line(leichtfigur: Bitboard, piece_team: Team) -> bool {
        match piece_team {
            Team::ONE => (leichtfigur.bits & 0x8080808080808080) != 0,
            Team::TWO => (leichtfigur.bits & 0x101010101010101) != 0,
        }
    }

    //noinspection DuplicatedCode
    /// Applies the given move to the board, for the specific team. Does
    /// NOT check, whether the move is legal.
    ///
    /// Returns: the amount of points to add for the given move
    #[inline]
    pub fn apply_move(&mut self, game_move: &Move, team: Team) -> u8 {
        let (friendly, enemy) = match team {
            Team::ONE => (&mut self.red, &mut self.blue),
            Team::TWO => (&mut self.blue, &mut self.red),
        };

        let old_piece = bitboard!(1 << game_move.from);
        let new_piece = bitboard!(1 << game_move.to);

        let old_is_double = self.double.get_bit(game_move.from);
        let new_has_piece = enemy.get_bit(game_move.to);
        let new_is_double = self.double.get_bit(game_move.to);

        //Calculate whether double
        let new_becomes_double = (old_is_double ^ new_has_piece) & !new_is_double;
        let mut points = ((old_is_double | new_is_double) & new_has_piece) as u8;

        //Set double
        self.double &= !old_piece;
        self.double &= !new_piece;
        self.double |= bitboard!(new_piece.bits * new_becomes_double as u64);

        match game_move.piece {
            PieceType::Robbe => {
                //Clear old piece
                self.robben &= !old_piece;
                *friendly &= !old_piece;

                //Set new piece
                self.robben |= new_piece;
                *friendly |= new_piece;

                //If enemy overlaps with new piece, clear that enemy piece.
                if (*enemy & new_piece).bits != 0 {
                    *enemy &= !new_piece;
                    self.seesterne &= !new_piece;
                    self.muscheln &= !new_piece;
                    self.moewen &= !new_piece;

                    *friendly &= !new_piece | self.double;
                    self.robben &= !new_piece | self.double;
                }
            }
            PieceType::Herzmuschel => {
                //Clear old piece
                self.muscheln &= !old_piece;
                *friendly &= !old_piece;

                //Set new piece
                self.muscheln |= new_piece;
                *friendly |= new_piece;

                //If enemy overlaps with new piece, clear that enemy piece.
                if (*enemy & new_piece).bits != 0 {
                    *enemy &= !new_piece;
                    self.seesterne &= !new_piece;
                    self.robben &= !new_piece;
                    self.moewen &= !new_piece;

                    *friendly &= !new_piece | self.double;
                    self.muscheln &= !new_piece | self.double;
                }

                //Check for finish line, else clear that piece and add one point.
                if Self::on_finish_line(new_piece, team) {
                    self.red &= !new_piece;
                    self.blue &= !new_piece;
                    self.muscheln &= !new_piece;
                    self.seesterne &= !new_piece;
                    self.moewen &= !new_piece;
                    self.robben &= !new_piece;
                    points += 1;
                }
            }
            PieceType::Seestern => {
                //Clear old piece
                self.seesterne &= !old_piece;
                *friendly &= !old_piece;

                //Set new piece
                self.seesterne |= new_piece;
                *friendly |= new_piece;

                //If enemy overlaps with new piece, clear that enemy piece.
                if (*enemy & new_piece).bits != 0 {
                    *enemy &= !new_piece;
                    self.muscheln &= !new_piece;
                    self.robben &= !new_piece;
                    self.moewen &= !new_piece;

                    *friendly &= !new_piece | self.double;
                    self.seesterne &= !new_piece | self.double;
                }

                //Check for finish line, else clear that piece and add one point.
                if Self::on_finish_line(new_piece, team) {
                    self.red &= !new_piece;
                    self.blue &= !new_piece;
                    self.muscheln &= !new_piece;
                    self.seesterne &= !new_piece;
                    self.moewen &= !new_piece;
                    self.robben &= !new_piece;
                    points += 1;
                }
            }
            PieceType::Moewe => {
                //Clear old piece
                self.moewen &= !old_piece;
                *friendly &= !old_piece;

                //Set new piece
                self.moewen |= new_piece;
                *friendly |= new_piece;

                //If enemy overlaps with new piece, clear that enemy piece.
                if (*enemy & new_piece).bits != 0 {
                    *enemy &= !new_piece;
                    self.muscheln &= !new_piece;
                    self.robben &= !new_piece;
                    self.seesterne &= !new_piece;

                    *friendly &= !new_piece | self.double;
                    self.moewen &= !new_piece | self.double;
                }

                //Check for finish line, else clear that piece and add one point.
                if Self::on_finish_line(new_piece, team) {
                    self.red &= !new_piece;
                    self.blue &= !new_piece;
                    self.muscheln &= !new_piece;
                    self.seesterne &= !new_piece;
                    self.moewen &= !new_piece;
                    self.robben &= !new_piece;
                    points += 1;
                }
            }
        }

        points
    }

    #[inline]
    pub fn leichtfigur_fortschritt(&self, team: Team) -> u8 {
        let leicht_figuren = self.moewen | self.seesterne | self.muscheln;
        match team {
            Team::ONE => {
                let player = self.red & leicht_figuren;
                let mut opp_side = bitboard!(0x8080808080808080);
                for i in (0..8).rev() {
                    if (player & opp_side).bits != 0 {
                        return i;
                    } else {
                        opp_side.bits >>= 1;
                    }
                }
            }
            Team::TWO => {
                let player = self.blue & leicht_figuren;
                let mut opp_side = bitboard!(0x101010101010101);
                for i in (0..8).rev() {
                    if (player & opp_side).bits != 0 {
                        return i;
                    } else {
                        opp_side.bits <<= 1;
                    }
                }
            }
        }
        0
    }

    #[inline]
    pub fn set_piece(&mut self, pos: u8, piece: Piece) {
        match piece.piece_type {
            PieceType::Robbe => self.robben.set_bit(pos),
            PieceType::Herzmuschel => self.muscheln.set_bit(pos),
            PieceType::Seestern => self.seesterne.set_bit(pos),
            PieceType::Moewe => self.moewen.set_bit(pos),
        };
        match piece.team {
            Team::ONE => self.red.set_bit(pos),
            Team::TWO => self.blue.set_bit(pos),
        }
        if piece.stacked {
            self.double.set_bit(pos);
        }
    }

    #[inline]
    pub fn count_moves(&self, team: Team) -> u8 {
        let player = match team {
            Team::ONE => self.red,
            Team::TWO => self.blue,
        };

        let unoccupied = !player;

        let mut count = 0;
        bit_loop((self.moewen & player).bits, |moewe| {
            count += (moewe_lookup_moves(square_of(moewe)) & unoccupied)
                .bits
                .count_ones();
        });
        bit_loop((self.muscheln & player).bits, |moewe| {
            count += (muschel_lookup_moves(square_of(moewe), team) & unoccupied)
                .bits
                .count_ones();
        });
        bit_loop((self.seesterne & player).bits, |moewe| {
            count += (seestern_lookup_moves(square_of(moewe), team) & unoccupied)
                .bits
                .count_ones();
        });
        bit_loop((self.robben & player).bits, |moewe| {
            count += (robbe_lookup_moves(square_of(moewe)) & unoccupied)
                .bits
                .count_ones();
        });
        count as u8
    }

    #[inline]
    pub fn for_each_move<F: FnMut(Move) -> bool>(&self, team: Team, f: &mut F) {
        let player_pieces = match team {
            Team::ONE => self.red,
            Team::TWO => self.blue,
        };
        let unoccupied = !player_pieces;
        let moewen = self.moewen & player_pieces;
        let robben = self.robben & player_pieces;
        let seesterne = self.seesterne & player_pieces;
        let muscheln = self.muscheln & player_pieces;

        bit_loop(moewen.bits, |moewe| {
            let from = square_of(moewe);
            let legal = moewe_lookup_moves(from) & unoccupied;
            let mov = legal.bits;
            bit_loop(mov, |mov_to| {
                let to = square_of(mov_to);
                if f(Move {
                    from,
                    to,
                    piece: PieceType::Moewe,
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
                    piece: PieceType::Robbe,
                }) {
                    return;
                }
            });
        });

        bit_loop(seesterne.bits, |seestern| {
            let from = square_of(seestern);
            let legal = seestern_lookup_moves(from, team) & unoccupied;
            let mov = legal.bits;
            bit_loop(mov, |mov_to| {
                let to = square_of(mov_to);
                if f(Move {
                    from,
                    to,
                    piece: PieceType::Seestern,
                }) {
                    return;
                }
            });
        });

        bit_loop(muscheln.bits, |muschel| {
            let from = square_of(muschel);
            let legal = muschel_lookup_moves(from, team) & unoccupied;
            let mov = legal.bits;
            bit_loop(mov, |mov_to| {
                let to = square_of(mov_to);
                if f(Move {
                    from,
                    to,
                    piece: PieceType::Herzmuschel,
                }) {
                    return;
                }
            });
        });
    }

    #[inline]
    pub fn available_moves(&self, team: Team) -> ThinVec<Move> {
        let player_pieces = match team {
            Team::ONE => self.red,
            Team::TWO => self.blue,
        };

        let unoccupied = !player_pieces;
        let moewen = self.moewen & player_pieces;
        let robben = self.robben & player_pieces;
        let seesterne = self.seesterne & player_pieces;
        let muscheln = self.muscheln & player_pieces;

        let mut moves = ThinVec::with_capacity(25);

        bit_loop(moewen.bits, |moewe| {
            let from = square_of(moewe);
            let legal = moewe_lookup_moves(from) & unoccupied;
            bit_loop(legal.bits, |mov_to| {
                let to = square_of(mov_to);
                moves.push(Move {
                    from,
                    to,
                    piece: PieceType::Moewe,
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
                    piece: PieceType::Robbe,
                })
            });
        });

        bit_loop(seesterne.bits, |seestern| {
            let from = square_of(seestern);
            let legal = seestern_lookup_moves(from, team) & unoccupied;
            bit_loop(legal.bits, |mov_to| {
                let to = square_of(mov_to);
                moves.push(Move {
                    from,
                    to,
                    piece: PieceType::Seestern,
                })
            });
        });

        bit_loop(muscheln.bits, |muschel| {
            let from = square_of(muschel);
            let legal = muschel_lookup_moves(from, team) & unoccupied;
            bit_loop(legal.bits, |mov_to| {
                let to = square_of(mov_to);
                moves.push(Move {
                    from,
                    to,
                    piece: PieceType::Herzmuschel,
                })
            });
        });
        return moves;
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut out: String = String::from("╔══════════════════════════╗\n");
        let mut index_min = 56 + 8;
        let mut index_max = 64 + 8;
        for _ in (0..8).rev() {
            index_max -= 8;
            index_min -= 8;
            out.push_str("║  ");
            for j in index_min..index_max {
                if let Some(piece) = self.piece_at(j) {
                    out.push_str(&*format!("{:<3}", piece.to_short_form()))
                } else {
                    out.push_str("-  ");
                };
            }
            out.push_str("║\n");
        }
        out.push_str("╚══════════════════════════╝");
        write!(f, "{}", out)
    }
}

#[cfg(test)]
mod test {
    use crate::bitboard;
    use crate::game::{Bitboard, Board, Move, Piece, PieceType, Team};
    use rand::SeedableRng;
    use rand_xoshiro::Xoshiro128Plus;

    #[test]
    fn test_frontmost_piece() {
        let mut board = Board::empty();

        board.set_piece(
            45,
            Piece {
                piece_type: PieceType::Herzmuschel,
                team: Team::ONE,
                stacked: false,
            },
        );
        assert_eq!(board.leichtfigur_fortschritt(Team::ONE), 5);

        board.set_piece(
            61,
            Piece {
                piece_type: PieceType::Seestern,
                team: Team::TWO,
                stacked: false,
            },
        );
        assert_eq!(board.leichtfigur_fortschritt(Team::TWO), 2);
    }

    #[test]
    fn test_on_finish_line() {
        let mut board = Board::empty();
        board.set_piece(
            7,
            Piece {
                piece_type: PieceType::Herzmuschel,
                team: Team::ONE,
                stacked: false,
            },
        );
    }

    #[test]
    fn test_board_points() {
        let mut rng = Xoshiro128Plus::seed_from_u64(2);
        let mut board = Board::new_random(&mut rng);

        let m = Move {
            from: 0,
            to: 14,
            piece: PieceType::Herzmuschel,
        };

        assert_eq!(board.apply_move(&m, Team::ONE), 0);

        let m = Move {
            from: 14,
            to: 7,
            piece: PieceType::Herzmuschel,
        };

        assert_eq!(board.apply_move(&m, Team::ONE), 1);

        let m = Move {
            from: 63,
            to: 57,
            piece: PieceType::Herzmuschel,
        };

        assert_eq!(board.apply_move(&m, Team::TWO), 0);

        let m = Move {
            from: 57,
            to: 56,
            piece: PieceType::Herzmuschel,
        };
        board.double.set_bit(57);
        assert_eq!(board.apply_move(&m, Team::TWO), 2);
    }

    #[test]
    fn test_double_board() {
        let mut board = Board::empty();
        board.set_piece(
            34,
            Piece {
                piece_type: PieceType::Robbe,
                team: Team::ONE,
                stacked: true,
            },
        );

        assert_eq!(board.double, bitboard!(1 << 34));

        let m = Move {
            from: 34,
            to: 44,
            piece: PieceType::Robbe,
        };

        assert_eq!(board.apply_move(&m, Team::ONE), 0);
        assert_eq!(board.double, bitboard!(1 << 44))
    }

    #[test]
    fn test_board_piece_interactions() {
        let mut board = Board::empty();
        board.set_piece(
            34,
            Piece {
                piece_type: PieceType::Robbe,
                team: Team::ONE,
                stacked: true,
            },
        );

        board.set_piece(
            24,
            Piece {
                piece_type: PieceType::Moewe,
                team: Team::TWO,
                stacked: true,
            },
        );

        let m = Move {
            from: 24,
            to: 34,
            piece: PieceType::Moewe,
        };

        assert_eq!(board.apply_move(&m, Team::TWO), 1);
    }
}
