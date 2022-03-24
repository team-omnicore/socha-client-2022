use crate::game::{PieceType, ShortForm, Team};
use socha_client_2022::util::{SCError, SCResult};

/// A placeable figure on the board.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Piece {
    /// Type of the (topmost) piece.
    pub(crate) piece_type: PieceType,
    /// Which team this piece belongs to.
    pub(crate) team: Team,
    /// Whether the piece is a tower.
    pub(crate) stacked: bool,
}

impl Piece {
    /// Creates a new piece.
    #[inline]
    pub fn new(piece_type: PieceType, team: Team, stacked: bool) -> Self {
        Self {
            piece_type,
            team,
            stacked,
        }
    }
}

impl ShortForm<String> for Piece {
    type Err = SCError;

    fn to_short_form(&self) -> String {
        let piece = if self.team == Team::ONE {
            self.piece_type.to_short_form().to_ascii_uppercase()
        } else {
            self.piece_type.to_short_form()
        };
        let mut out = String::from(piece);
        if self.stacked {
            out.push('*');
        }
        out
    }

    fn from_short_form(s: &String) -> SCResult<Self> {
        let piece_type = PieceType::from_short_form(&s.chars().nth(0).unwrap())?;
        let team = if s.chars().nth(0).unwrap().is_uppercase() {
            Team::ONE
        } else {
            Team::TWO
        };
        let stacked = s.chars().nth(1) == Some('*');
        Ok(Piece {
            piece_type,
            team,
            stacked,
        })
    }
}
