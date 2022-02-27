use std::str::FromStr;
use std::string::ParseError;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Team {
    RED,
    BLUE,
}

impl Team {
    pub fn next(&self) -> Team {
        match self {
            Team::RED => Team::BLUE,
            Team::BLUE => Team::RED,
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
            "ONE" => Ok(Team::RED),
            "TWO" => Ok(Team::BLUE),
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
