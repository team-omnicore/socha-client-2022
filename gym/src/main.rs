#![feature(thread_is_running)]
#![allow(unused)]
use crate::player::Player;
use crate::server::Server;
use client::algorithms::heuristics::*;
use client::algorithms::{Algorithm, MinMax, MinMaxState, RandomPick};
use client::game::{Gamestate, Team};
use std::thread;

mod player;
mod server;

fn main() {
    let mut handles = vec![];
    for i in 0..1 {
        let t = thread::spawn(move || {
            let mut server = Server::new("gym/.gym_files/gym.log".to_string());

            let p1 = Player::new(
                "Minmax_depth_5_EVAL_2603_1".to_string(),
                MinMax::new(5, EVAL_2603_1),
            );

            let p2 = Player::new(
                "Minmax_depth_5_EVAL_2703_1".to_string(),
                MinMax::new(5, EVAL_2703_1),
            );

            let p3 = Player::new(
                "Minmax_depth_5_EVAL_2703_2".to_string(),
                MinMax::new(5, EVAL_2703_2),
            );

            let p4 = Player::new(
                "Minmax_depth_5_DEFAULT_EVALUATION".to_string(),
                MinMax::new(5, Gamestate::default_evaluation),
            );

            let p5 = Player::new(
                "Minmax_depth_5_EVAL_2703_3".to_string(),
                MinMax::new(5, EVAL_2703_3),
            );

            server.add_player(p1);
            server.add_player(p2);
            server.add_player(p3);
            server.add_player(p4);
            server.add_player(p5);

            server.init();

            loop {
                println!("Playing on thread {}", i);
                server.play_match();
            }
        });
        handles.push(t);
    }
    for x in handles {
        x.join();
    }
}
