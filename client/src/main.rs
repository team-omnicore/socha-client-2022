use env_logger::Builder;
use std::env;
use log::LevelFilter;
use crate::game::Join;
use game_lib::gamestate::Gamestate;
use game_lib::score::Score;
use game_lib::board::Board;
use rand::rngs::ThreadRng;
use rand::thread_rng;
use game_lib::min_max::{Priv, MinMax};

mod xml_node;
mod game_result;
mod game;
mod team;

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

fn main() {
    let args = Args::parse();

    Builder::new()
        .parse_env(&env::var("MY_APP_LOG").unwrap_or_default())
        .filter_level(LevelFilter::Info)
        .init();


    let join = if let Some(reservation) = args.reservation {
        Join::PREPARED(reservation)
    } else if let Some(room) = args.room {
        Join::ROOM(room)
    } else {
        Join::ANY
    };

    let network_address = format!("{}:{}", args.host, args.port);
    let mut game = join.connect(network_address.as_str()).expect("Connection failed");

    let result = game.game_loop();
    match result {
        Ok(res) => {
            log::info!("{:?}", res);
        }
        Err(err) => {
            log::error!("Network error! {:?}", err);
        }
    }


    //let mut rng = thread_rng();
//
    //let mut g = Gamestate{
    //    board: Board::new_random(&mut rng),
    //    round: 0,
    //    is_maximizing_player: true,
    //    score: Score{ bytes: [0,0] }
    //};
//
    //println!("{}", g);
//
    //let game_move = g.calculate_best_move(6).unwrap();
    //println!("From {} to {}", game_move.from, game_move.to);
//
    //g.apply_move(&game_move);
    //println!("{}", g);
}
