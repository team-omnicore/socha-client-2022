use std::fmt;
use std::str::FromStr;

use socha_client_2022::util::{SCError, SCResult};
use std::ops::Not;

/// A playing party in the game.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Team {
    ONE = 0,
    TWO = 1,
}

impl Team {
    /// The opponent of the given team.
    pub fn opponent(self) -> Team {
        match self {
            Team::ONE => Team::TWO,
            Team::TWO => Team::ONE,
        }
    }

    /// The x-direction of the team on the board.
    pub fn direction(self) -> i32 {
        match self {
            Team::ONE => 1,
            Team::TWO => -1,
        }
    }
}

impl Not for Team {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.opponent()
    }
}

//noinspection DuplicatedCode
impl fmt::Display for Team {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Team::ONE => write!(f, "ONE"),
            Team::TWO => write!(f, "TWO"),
        }
    }
}

//noinspection DuplicatedCode
impl FromStr for Team {
    type Err = SCError;

    fn from_str(s: &str) -> SCResult<Self> {
        match s {
            "ONE" => Ok(Team::ONE),
            "TWO" => Ok(Team::TWO),
            _ => Err(SCError::UnknownVariant(format!("Unknown team {}", s))),
        }
    }
}
