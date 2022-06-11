#![allow(dead_code)]

pub mod algorithms;
pub mod bridge;
pub mod client;
pub mod game;
pub mod utils;

use crate::algorithms::heuristics::*;
use crate::algorithms::*;
use crate::client::Client;
use chrono::Local;
use clap::Parser;
use env_logger::{Builder, Target};
use log::LevelFilter;
use std::env;
use std::io::Write;
use std::time::Duration;

/// Rust client for the board game "Ostseeschach"
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// host of the game server
    #[clap(short, long, default_value = "localhost")]
    host: String,

    /// Port of the game server
    #[clap(short, long, default_value_t = 13050)]
    port: u16,

    /// Reservationnumber for a game
    #[clap(short, long)]
    reservation: Option<String>,

    /// Room ID for a game
    #[clap(long)]
    room: Option<String>,
}

fn main() {
    let args = Args::parse();

    Builder::new()
        .parse_env(&env::var("MY_APP_LOG").unwrap_or_default())
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter_level(LevelFilter::Info)
        .target(Target::Stdout)
        .init();

    //let algorithm = MinMax::new(6, EVAL_2603_1);
    //let algorithm = MonteCarloTreeSearch::new(2000000.0, Duration::from_millis(1800));

    let algorithm = CratesMCTS::new(0.5, Duration::from_millis(1600));
    let mut client = Client::new(algorithm, args.reservation.clone());

    log::info!("Reservation: {:?}", args.reservation.clone());

    let _result = client
        .connect(&args.host, args.port)
        .expect("Failed to connect to Server");
}
