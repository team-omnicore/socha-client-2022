#![allow(unused)]

use crate::game::Join;
use env_logger::{Builder, Target};
use game_lib::board::Board;
use game_lib::game_move::Move;
use game_lib::gamestate::Gamestate;
use game_lib::piece::PieceType;
use game_lib::score::Score;
use log::LevelFilter;
use rand::rngs::ThreadRng;
use rand::thread_rng;
use std::{env, fs};
use game_algorithms::algorithms::Algorithms;

mod game;
mod game_result;
mod xml_node;

use crate::game_result::GameResult;
use crate::game_result::Score::{DRAW, LOSS, WIN};
use clap::Parser;

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


pub fn run(algorithm: Algorithms) {
    let args = Args::parse();

    Builder::new()
        .parse_env(&env::var("MY_APP_LOG").unwrap_or_default())
        .filter_level(LevelFilter::Info)
        .target(Target::Stdout)
        .init();

    let join = if let Some(reservation) = args.reservation {
        Join::PREPARED(reservation)
    } else if let Some(room) = args.room {
        Join::ROOM(room)
    } else {
        Join::ANY
    };

    let network_address = format!("{}:{}", args.host, args.port);
    let mut game = join
        .connect(network_address.as_str(), algorithm)
        .expect("Connection failed");

    let result = game.game_loop();
    match result {
        Ok(res) => {
            log::info!("{:?}", res);

            /*
            match res.score {
                DRAW(_) => draw_count += 1,
                WIN(_, _) => win_count += 1,
                LOSS(_, _) => lose_count += 1,
            }

            let data = format!(
                "client_2 - classic evaluation\nwins: {}\n losses: {}\n draws: {}\nwinrate: {:.2}%",
                win_count,
                lose_count,
                draw_count,
                (win_count as f32) / ((lose_count + draw_count + win_count) as f32) * 100f32
            );
            fs::write("client_1.txt", data).expect("Fuk");
            */
        }
        Err(err) => {
            log::error!("Network error! {:?}", err);
        }
    }
}
