#![allow(dead_code)]

mod algorithms;
mod bridge;
mod client;
mod game;
mod utils;

use crate::algorithms::{MinMax};
use crate::client::Client;
use clap::Parser;
use std::io::Write;
use env_logger::{Builder, Target};
use log::LevelFilter;
use std::env;
use chrono::Local;

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
            writeln!(buf,
                     "{} [{}] - {}",
                     Local::now().format("%Y-%m-%dT%H:%M:%S"),
                     record.level(),
                     record.args()
            )
        })
        .filter_level(LevelFilter::Info)
        .target(Target::Stdout)
        .init();

    let minmax = MinMax::new(5);
    Client::new(minmax, None)
        .connect(&args.host, args.port)
        .expect("Failed to connect to Server");
}
