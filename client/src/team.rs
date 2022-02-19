use std::str::FromStr;
use std::string::ParseError;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Team {
    ONE,
    TWO,
}

impl Team {
    pub fn next(&self) -> Team {
        match self {
            Team::ONE => Team::TWO,
            Team::TWO => Team::ONE,
        }
    }

    pub fn current(start_team: Team, turn: u8) -> Team {
        match turn + 1 & 0x1 {
            0 => start_team,
            1 => start_team.next(),
            _ => {
                panic!("Shouldn't happen.")
            }
        }
    }
}

impl FromStr for Team {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ONE" => Ok(Team::ONE),
            "TWO" => Ok(Team::TWO),
            s => {
                panic!("No team with name {}", s)
            }
        }
    }
}

impl From<&String> for Team {
    fn from(team: &String) -> Self {
        team.as_str().parse::<Team>().unwrap()
    }
}
