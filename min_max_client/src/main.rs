use client::run;
use game_algorithms::algorithms::Algorithms;

fn main() {
    let depth: u8 = 5;
    run(Algorithms::MinMax(depth));
}
