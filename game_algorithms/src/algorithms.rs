use std::time::Duration;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Algorithms {
    MinMax(u8),
    MonteCarloTreeSearch(Duration),
}