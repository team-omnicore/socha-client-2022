#[derive(Debug)]
pub struct GameResult {
    pub score: Score,
}

#[derive(Copy, Clone, Debug)]
pub enum Score {
    DRAW(u8),
    WIN(u8, u8),
    LOSS(u8, u8),
}

#[derive(Copy, Clone, Debug)]
pub enum Cause<'a> {
    Regular,
    Left(&'a str),
    RuleViolation(&'a str),
    SoftTimeout(&'a str),
    HardTimeout(&'a str),
}

impl<'a> Cause<'a> {
    pub fn from_str(s: &str, reason: &'a str) -> Result<Self, ()> {
        match s {
            "REGULAR" => Ok(Cause::Regular),
            "LEFT" => Ok(Cause::Left(reason)),
            "RULE_VIOLATION" => Ok(Cause::RuleViolation(reason)),
            "SOFT_TIMEOUT" => Ok(Cause::SoftTimeout(reason)),
            "HARD_TIMEOUT" => Ok(Cause::HardTimeout(reason)),
            s => {
                panic!("No cause with name {}", s)
            }
        }
    }
}
