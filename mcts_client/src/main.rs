use std::time::Duration;
use client::run;
use game_algorithms::algorithms::Algorithms;

fn main() {
    let calculation_time = Duration::from_millis(1900);
    run(Algorithms::MonteCarloTreeSearch(calculation_time));
}
