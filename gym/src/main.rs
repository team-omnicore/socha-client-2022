#![allow(unused)]
use client::algorithms::{MinMax, Algorithm, RandomPick};
use client::client::Client;
use client::game::{Team, Gamestate};
use std::fs;
use std::fs::File;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;
use client::algorithms::heuristics::EVAL_2603_1;

fn main() {
    sim()
}

fn sim(){
    let mut i = 0;
    loop {
        if i%2==0 {
            sleep(Duration::from_millis(1100));
        }

        let mut wins = 0;
        let mut losses = 0;
        let mut draws = 0;

        let data = fs::read_to_string("gym/.gym_files/result.txt");
        if data.is_ok() {
            let data = data.unwrap();
            let vec:Vec<_> = data.split('\n').collect();
            wins = vec[0].split(": ").nth(1).unwrap().parse().unwrap();
            losses = vec[1].split(": ").nth(1).unwrap().parse().unwrap();
            draws = vec[2].split(": ").nth(1).unwrap().parse().unwrap();
        }else {
            fs::create_dir_all("gym/.gym_files/").expect("Failed to create dirs");
            File::create(Path::new("gym/.gym_files/result.txt")).expect("Failed to create file");
        }

        let minmax = MinMax::new(5, EVAL_2603_1);
        let mut client = Client::new(minmax, None);

        let result = client
            .connect("localhost", 13050)
            .expect("Failed to connect to Server");

        if let Some(winner) = result.winner().as_ref() {
            if Team::from(winner.team()) == client.team().unwrap() {
                wins += 1;
            } else {
                losses += 1;
            }
        } else {
            draws += 1;
        }

        let res = format!(
            "Wins: {}\nLosses: {}\nDraws: {}\nWinrate -> {:.1}%\nWinratio -> {:.1}",
            wins,
            losses,
            draws,
            wins as f32 / (wins + losses + draws) as f32 * 100f32,
            wins as f32 / losses as f32
        );
        fs::write("gym/.gym_files/result.txt", res).expect("Unable to write file");
        i+=1;
    }
}



/*enum Player<A: Algorithm>{
    Internal(Client<A>),
    External(Path)
}*/