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
