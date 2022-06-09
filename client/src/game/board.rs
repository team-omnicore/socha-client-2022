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

#[macro_export]
macro_rules! bit_loop {
    ($pieces:expr, $name:ident, $code:block) => {
        let mut $name = $pieces;
        while $name != 0 {
            $code
            $name &= $name - 1;
        }
    }
}

#[macro_export]
macro_rules! for_each_move {
    ($board:expr, $team:expr, $name:ident, $f:block) => {
        use crate::bit_loop;
        use crate::game::*;
        use crate::utils::square_of;

        let player_pieces = match $team {
            Team::ONE => $board.red,
            Team::TWO => $board.blue,
        };

        let unoccupied = !player_pieces;
        let moewen = $board.moewen & player_pieces;
        let robben = $board.robben & player_pieces;
        let seesterne = $board.seesterne & player_pieces;
        let muscheln = $board.muscheln & player_pieces;

        bit_loop!(moewen.bits, moewe, {
            let from = square_of(moewe);
            let legal = moewe_lookup_moves(from) & unoccupied;
            bit_loop!(legal.bits, mov_to, {
                let to = square_of(mov_to);
                let $name = Move {
                    from,
                    to,
                    piece: PieceType::Moewe,
                };
                $f
            });
        });

        bit_loop!(robben.bits, robbe, {
            let from = square_of(robbe);
            let legal = robbe_lookup_moves(from) & unoccupied;
            bit_loop!(legal.bits, mov_to, {
                let to = square_of(mov_to);
                let $name = Move {
                    from,
                    to,
                    piece: PieceType::Robbe,
                };
                $f
            });
        });

        bit_loop!(seesterne.bits, seestern, {
            let from = square_of(seestern);
            let legal = seestern_lookup_moves(from, $team) & unoccupied;
            bit_loop!(legal.bits, mov_to, {
                let to = square_of(mov_to);
                let $name = Move {
                    from,
                    to,
                    piece: PieceType::Seestern,
                };
                $f
            });
        });

        bit_loop!(muscheln.bits, muschel, {
            let from = square_of(muschel);
            let legal = muschel_lookup_moves(from, $team) & unoccupied;
            bit_loop!(legal.bits, mov_to, {
                let to = square_of(mov_to);
                let $name = Move {
                    from,
                    to,
                    piece: PieceType::Herzmuschel,
                };
                $f
            });
        });
    };
}

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
    /// Constructs an empty board, with no pieces on it.
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

    /// Returns: the pieces for a specific team encoded in a bitboard
    #[inline]
    pub const fn player_pieces(&self, team: Team) -> Bitboard {
        match team {
            Team::ONE => self.red,
            Team::TWO => self.blue,
        }
    }

    /// Constructs a board with a random starting position determined
    /// by the given rng.
    #[inline]
    pub fn new_random<T: Rng>(rng: &mut T) -> Self {
        let blue = bitboard!(0xFF00000000000000);
        let red = bitboard!(0xFF);
        let double = bitboard!();

        let mut start_positions = vec![0x1, 0x2, 0x4, 0x8, 0x10, 0x20, 0x40, 0x80];
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

    /// Returns: the piece at a specific position, or None if there is no piece
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

    /// Returns: whether a point should be given for reaching the opposite side of ones
    /// baseline with a leichtfigur
    #[inline]
    fn on_finish_line(leichtfigur: Bitboard, piece_team: Team) -> bool {
        match piece_team {
            Team::ONE => (leichtfigur.bits & 0xFF00000000000000) != 0,
            Team::TWO => (leichtfigur.bits & 0xFF) != 0,
        }
    }

    /// Applies the given move to the board, for the specific team. Does
    /// NOT check, whether the move is legal.
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

    /// Returns: the distance of the furthest leichtfigur from the baseline
    #[inline]
    pub fn leichtfigur_fortschritt(&self, team: Team) -> u8 {
        let leicht_figuren = self.moewen | self.seesterne | self.muscheln;
        match team {
            Team::ONE => {
                let player = self.red & leicht_figuren;
                let mut opp_side = bitboard!(0xFF00000000000000);
                for i in (0..8).rev() {
                    if (player & opp_side).bits != 0 {
                        return i;
                    } else {
                        opp_side.bits >>= 8;
                    }
                }
            }
            Team::TWO => {
                let player = self.blue & leicht_figuren;
                let mut opp_side = bitboard!(0xFF);
                for i in (0..8).rev() {
                    if (player & opp_side).bits != 0 {
                        return i;
                    } else {
                        opp_side.bits <<= 8;
                    }
                }
            }
        }
        0
    }

    /// Puts a new piece onto the board. <br>
    /// Replaces the piece at that position
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

    /// Counts the amount of available moves for a certain team.<br>
    /// Faster than getting the size of the available_moves() vector.
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

    /// Iterates over each move for a certain team, without auxiliary space
    #[inline]
    pub fn for_each_move<F: FnMut(Move)>(&self, team: Team, f: &mut F) {
        for_each_move!(self, team, mov, {
            f(mov);
        });
    }

    /// Returns: the available moves for a team
    #[inline]
    pub fn available_moves(&self, team: Team) -> ThinVec<Move> {
        let mut moves = ThinVec::with_capacity(25);
        self.for_each_move(team, &mut |mov| {
            moves.push(mov);
        });
        moves
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
    use rand::SeedableRng;
    use rand_xoshiro::Xoshiro128Plus;

    use crate::bitboard;
    use crate::game::{Bitboard, Board, Move, Piece, PieceType, Team};

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
        assert_eq!(board.leichtfigur_fortschritt(Team::TWO), 0);
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

        let mut points = board.apply_move(
            &Move {
                from: 6,
                to: 52,
                piece: PieceType::Robbe,
            },
            Team::ONE,
        );
        assert_eq!(points, 0);

        points = board.apply_move(
            &Move {
                from: 60,
                to: 52,
                piece: PieceType::Seestern,
            },
            Team::TWO,
        );
        assert_eq!(points, 0);

        points = board.apply_move(
            &Move {
                from: 1,
                to: 52,
                piece: PieceType::Moewe,
            },
            Team::ONE,
        );
        assert_eq!(points, 1);

        points = board.apply_move(
            &Move {
                from: 0,
                to: 62,
                piece: PieceType::Herzmuschel,
            },
            Team::ONE,
        );
        assert_eq!(points, 1);

        points = board.apply_move(
            &Move {
                from: 56,
                to: 11,
                piece: PieceType::Herzmuschel,
            },
            Team::TWO,
        );
        assert_eq!(points, 0);

        board.double.set_bit(11);
        points = board.apply_move(
            &Move {
                from: 11,
                to: 2,
                piece: PieceType::Herzmuschel,
            },
            Team::TWO,
        );
        assert_eq!(points, 2);
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
