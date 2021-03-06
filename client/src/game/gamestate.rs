use crate::game::{zobrist, Board, Fen, IGamestate, Move, Team, Tile};
use rand::Rng;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use thincollections::thin_vec::ThinVec;

#[derive(Debug, Copy, Clone)]
pub struct Gamestate {
    pub board: Board,
    pub turn: u8,
    pub ambers: [u8; 2], //[ONE | TWO]
    pub hash: u64,
}

impl Gamestate {
    /// Constructs a new gamestate with default starting settings and initialises its hash
    #[inline]
    pub fn new(board: Board) -> Self {
        Self::new_with(board, 0, [0, 0])
    }

    /// Constructs a new gamestate with the given parameters and initialises its hash
    #[inline]
    pub fn new_with(board: Board, turn: u8, ambers: [u8; 2]) -> Self {
        let mut state = Self {
            board,
            turn,
            ambers,
            hash: 0,
        };
        state.recalculate_hash();
        state
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

    /// Recalculates the Zobrist hash, discarding the previous hash information.
    #[inline]
    pub fn recalculate_hash(&mut self) -> u64 {
        self.hash = 0;
        for (pos, piece) in self
            .board
            .iter_tiles()
            .enumerate()
            .filter_map(|(pos, tile)| match tile {
                Tile::Empty => None,
                Tile::Piece(piece) => Some((pos, piece)),
            })
        {
            self.hash ^= zobrist::hash_for_piece(piece, pos as u8);
        }
        self.hash ^= zobrist::hash_for_score(self.ambers);
        self.hash
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
        if self.turn % 2 == 0 {
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
        //////////////// BEFORE APPLY MOVE!!! ///////////////////
        //Remove old piece which is getting moved
        let from_before = self.board.piece_at(game_move.from).unwrap();
        self.hash ^= zobrist::hash_for_piece(from_before, game_move.from);
        //Remove old piece which is getting moved onto
        if let Some(to_before) = self.board.piece_at(game_move.to) {
            self.hash ^= zobrist::hash_for_piece(to_before, game_move.to);
        }
        //Remove old score
        self.hash ^= zobrist::hash_for_score(self.ambers);
        /////////////////////////////////////////////////////////

        let points = self.board.apply_move(game_move, self.current_player()); //Apply the move to the board, return the points gotten by jumping on other pieces
        self.ambers[self.current_player() as usize] += points; //add points
        self.turn += 1; //Next round

        ///////////////// AFTER APPLY MOVE!!! ///////////////////
        //Add new piece
        if let Some(to_after) = self.board.piece_at(game_move.to) {
            self.hash ^= zobrist::hash_for_piece(to_after, game_move.to);
        }
        self.hash ^= zobrist::hash_for_score(self.ambers);
        /////////////////////////////////////////////////////////
    }

    #[inline]
    fn game_over(&self) -> bool {
        (self.turn % 2 == 0 && (self.ambers[0] >= 2 || self.ambers[1] >= 2)) || self.turn >= 60
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
        let mut gamestate = Gamestate::new(board);

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

impl Hash for Gamestate {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.hash);
    }
}

impl PartialEq for Gamestate {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}