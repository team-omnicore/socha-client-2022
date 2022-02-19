#![allow(unused)]

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
use game_lib::game_move::Move;
use game_lib::piece::PieceType;

mod xml_node;
mod game_result;
mod game;
mod team;

fn main() {

    Builder::new()
        .parse_env(&env::var("MY_APP_LOG").unwrap_or_default())
        .filter_level(LevelFilter::Info)
        .init();

    let mut game = Join::ANY
        .connect("localhost:13050")
        .expect("Connection failed");

    let result = game.game_loop();

    match result {
        Ok(res) => {
            log::info!("{:?}", res);
        }
        Err(err) => {
            log::error!("Network error! {:?}", err);
        }
    }
}
