use crate::game::{Board, Gamestate, Move, Piece, PieceType, Team};
use socha_client_2022::game::{
    Board as SCBoard, Move as SCMove, Piece as SCPiece, PieceType as SCPieceType, State as SCState,
    Team as SCTeam, Vec2,
};

impl From<SCPieceType> for PieceType {
    fn from(piece_type: SCPieceType) -> Self {
        match piece_type {
            SCPieceType::Herzmuschel => PieceType::Herzmuschel,
            SCPieceType::Moewe => PieceType::Moewe,
            SCPieceType::Seestern => PieceType::Seestern,
            SCPieceType::Robbe => PieceType::Robbe,
        }
    }
}

impl From<Move> for SCMove {
    fn from(m: Move) -> Self {
        let from = Vec2::new((m.from / 8) as i32, (m.from % 8) as i32);
        let to = Vec2::new((m.to / 8) as i32, (m.to % 8) as i32);
        Self::new(from, to)
    }
}

impl From<SCTeam> for Team {
    fn from(team: SCTeam) -> Self {
        match team {
            SCTeam::One => Self::ONE,
            SCTeam::Two => Self::TWO,
        }
    }
}

impl From<SCPiece> for Piece {
    fn from(piece: SCPiece) -> Self {
        let piece_type = PieceType::from(piece.piece_type());
        let team = Team::from(piece.team());
        let stacked = piece.count() > 1;
        Self {
            piece_type,
            team,
            stacked,
        }
    }
}

impl From<SCBoard> for Board {
    fn from(sc_board: SCBoard) -> Self {
        let mut board = Self::empty();
        sc_board
            .pieces()
            .iter()
            .map(|(coord, piece)| {
                let pos = (8 * coord.x + coord.y) as u8;
                let piece = Piece::from(*piece);
                (pos, piece)
            })
            .for_each(|(pos, piece)| {
                board.set_piece(pos, piece);
            });
        board
    }
}

impl From<SCState> for Gamestate {
    fn from(state: SCState) -> Self {
        let board = Board::from(state.board().clone());
        let turn = state.turn() as u8;
        let ambers = [
            (*state.ambers().get(&SCTeam::One).unwrap_or(&0)) as u8,
            (*state.ambers().get(&SCTeam::Two).unwrap_or(&0)) as u8,
        ];
        Gamestate::new_with(board, turn, ambers)
    }
}
