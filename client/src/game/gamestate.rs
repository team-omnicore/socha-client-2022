use crate::game::{Board, Fen, IGamestate, Move, Team};
use rand::Rng;
use std::fmt::{Display, Formatter};
use thincollections::thin_vec::ThinVec;

#[derive(Debug, Copy, PartialEq, Clone)]
pub struct Gamestate {
    pub board: Board,
    pub round: u8,
    pub ambers: [u8; 2], //[ONE | TWO]
}

impl Gamestate {
    /// Constructs a new gamestate with default starting settings
    #[inline]
    pub const fn new(board: Board) -> Self {
        Gamestate {
            board,
            round: 1,
            ambers: [0, 0],
        }
    }

    /// Constructs a new gamestate with a random starting board
    #[inline]
    pub fn new_random<T: Rng>(rng: &mut T) -> Self {
        let board = Board::new_random(rng);
        Self::new(board)
    }

    /// Calculates the winner of the match. <br>
    /// Returns: None, if the game isn't over or is a true tie
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
                self.wins_draw()
            }
        }
    }

    /// Calculates the winner of an amber draw
    #[inline]
    pub fn wins_draw(&self) -> Option<Team> {
        let lf = self.board.moewen | self.board.seesterne | self.board.muscheln;
        let bytes_r = (lf & self.board.red).bits.to_be_bytes();
        let bytes_b = (lf & self.board.blue).bits.to_le_bytes();

        for i in 0..8 {
            let ones_r = bytes_r[i].count_ones();
            let ones_b = bytes_b[i].count_ones();

            if ones_r > ones_b {
                return Some(Team::ONE);
            } else if ones_r < ones_b {
                return Some(Team::TWO);
            }
        }
        None
    }
}

impl IGamestate for Gamestate {
    type MoveType = Move;

    #[inline]
    fn available_moves(&self, team: Team) -> ThinVec<Self::MoveType> {
        self.board.available_moves(team)
    }

    #[inline]
    fn current_player(&self) -> Team {
        if self.round % 2 == 1 {
            Team::ONE
        } else {
            Team::TWO
        }
    }

    #[inline]
    fn count_moves(&self, team: Team) -> u8 {
        self.board.count_moves(team)
    }

    #[inline]
    fn for_each_move<F: FnMut(Self::MoveType)>(&self, team: Team, f: &mut F) {
        self.board.for_each_move(team, f)
    }

    #[inline]
    fn apply_move(&mut self, game_move: &Self::MoveType) {
        let points = self.board.apply_move(game_move, self.current_player()); //Apply the move to the board, return the points gotten by jumping on other pieces
        self.ambers[self.current_player() as usize] += points;

        self.round += 1; //Next round
    }

    #[inline]
    fn game_over(&self) -> bool {
        (self.round % 2 == 0 && (self.ambers[0] >= 2 || self.ambers[1] >= 2)) || self.round >= 60
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
    use crate::game::PieceType;
    use rand::SeedableRng;
    use rand_xoshiro::Xoshiro128Plus;

    #[test]
    fn test_points_system() {
        let mut rng = Xoshiro128Plus::seed_from_u64(2);
        let board = Board::new_random(&mut rng);
        let mut gamestate = Gamestate {
            board,
            round: 1,
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
