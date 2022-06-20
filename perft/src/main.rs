use std::time::SystemTime;
use client::game::{Fen, Gamestate};
use clap::Parser;
use crate::slow_perft::perft_up_to;

mod slow_perft;

#[derive(Parser, Debug)]
struct Args {
    /// Fen String of the gamestate
    #[clap(short, long)]
    fen: String,

    /// Port of the game server
    #[clap(short, long)]
    depth: u32,
}

fn main() {
    let args = Args::parse();

    let state = Gamestate::load_fen(&*args.fen.clone()).expect("Please input a valid FEN");
    let depth = args.depth;

    println!("Hash of state is {:x}", state.hash);

    let start = SystemTime::now();
    let move_count = perft_up_to(state, depth);
    let dur = SystemTime::now().duration_since(start).unwrap();
    println!("Perft to depth {} took {:?}",depth, dur);
    println!("Perft {}: {} total moves", depth, move_count);

    println!("That is: {:.2} MNodes/s", move_count as f64 / (dur.as_secs_f64() * 1e6))
}
