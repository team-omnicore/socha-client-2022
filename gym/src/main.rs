#![allow(unused)]

use serde::{Serialize,Deserialize};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::hash::{Hash, Hasher};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};
use rand::thread_rng;
use rand::prelude::{SliceRandom, IteratorRandom};
use std::process::{Command, Stdio};
use client::game::{Gamestate, Board, Team};

fn main() {

    let m = Board::new_random(&mut thread_rng());

    println!("{}", m);
    let mut available = m.available_moves(Team::ONE);
    available.sort_unstable_by_key(|x| x.from);
    println!("{:?}",available );

    //let mut m = MatchMaker::new(PathBuf::from(Path::new("gym/.gym_files/")), 1);
    //loop {
    //    m.run_match();
    //}
}

struct MatchMaker{
    players: Vec<Player>,
    directory: PathBuf,
    threads: u16,
    k: u32,
    n: u32
}

impl MatchMaker {
    fn new(working_dir: PathBuf, threads: u16)->Self{
        Self{ players: Default::default(), directory: working_dir, threads, k: 12, n: 6 }
    }

    fn load_players(&mut self){
        for player in &self.players {
            player.save_to_file(&self.directory);
        }
        self.players.clear();
        let dir = &self.directory;
        let jsons: Vec<_> = std::fs::read_dir(dir).unwrap().filter(|file|{
            file.as_ref().unwrap().path().extension().unwrap() == "json"
        }).collect();
        for file in jsons {
            let json = std::fs::read_to_string(file.unwrap().path()).unwrap();
            let player: Player = serde_json::from_str(&json).expect("Failed to parse Json");
            self.players.push(player);
        }
    }

    fn run_match(&mut self){
        self.load_players();
        if self.players.len() < 2 {
            eprintln!("Insufficient amount of clients!");
            return;
        }
        let mut players: Vec<_> = self.players.iter_mut().choose_multiple(&mut thread_rng(), 2);

        let player1_path = self.directory.join(&players[0].bin_filename);
        let player2_path = self.directory.join(&players[1].bin_filename);

        println!("player1_path: {:?}", player1_path.as_os_str());
        println!("player2_path: {:?}", player2_path.as_os_str());

        let output = File::create(Path::new("tmp.log")).unwrap();
        let io = Stdio::from(output);
        let mut command = Command::new("java");
        command.args([
            "-jar", "-Dlogback.configurationFile=logback.xml", "gym/scserver/test-client.jar",
            "--tests", &self.n.to_string(),
            "--name1", "player1",
            "--player1", player1_path.to_str().unwrap(),
            "--name2", "player2",
            "--player2", player2_path.to_str().unwrap(),
            "--port", &13060u16.to_string(),
        ]).stdout(io);

        let mut wait = command.spawn().expect("Failed to start");
        wait.wait();

        let output = std::fs::read_to_string(Path::new("tmp.log")).unwrap();
        let scores_start = output.rfind("=============== SCORES ================").unwrap() + "=============== SCORES ================".len()+1;
        let scores_end = output.rfind("=======================================").unwrap() -1;
        let relevant = String::from(output.get(scores_start..scores_end).unwrap());
        let start = relevant.find("player1: ").unwrap() + "player1: ".len() ;
        let end = relevant.rfind("player2: ").unwrap() - 1;
        let player1_score = relevant.get(start..end).unwrap().parse::<u32>().unwrap();
        let start = end  + "player2: ".len()+1;
        let end = relevant.len();
        let player2_score = relevant.get(start..end).unwrap().parse::<u32>().unwrap();

        let (player1_score, player2_score) = if player1_score > player2_score {
            (1.0, 0.0)
        }else if player1_score < player2_score {
            (0.0,1.0)
        }else {
            (0.5,0.5)
        };

        let r1 = players[0].elo_rating;
        let r2 = players[0].elo_rating;

        let e1 = 10f32.powf(r1 / 400f32);
        let e2 = 10f32.powf(r2 / 400f32);

        let r1_ = r1 + self.k as f32* (player1_score - (e1 / (e1 + e2)));
        let r2_ = r2 + self.k as f32* (player2_score - (e2 / (e2 + e2)));

        players[0].elo_rating = r1_;
        players[1].elo_rating = r2_;
        players[0].games_played += self.n;
        players[1].games_played += self.n;

        players[0].save_to_file(&self.directory);
        players[1].save_to_file(&self.directory);
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Player{
    bin_filename: String,
    elo_rating: f32,
    games_played: u32,
    description: Description
}

impl Player {
    fn save_to_file(&self, path: &PathBuf){
        let write_to = File::create(path.join(&self.bin_filename).with_extension("json")).expect("Failed to create file");
        let writer = BufWriter::new(write_to);
        serde_json::to_writer_pretty(writer, &self).expect("Failed to serialize player");
        println!("Saved {} to file!", self.bin_filename);
    }
}

impl PartialEq for Player{
    fn eq(&self, other: &Self) -> bool {
        self.games_played == other.games_played &&
        self.bin_filename.eq(&other.bin_filename) &&
        self.elo_rating.eq(&other.elo_rating)
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
struct Description{
    title: String,
    heuristic: String,
    algorithm: String
}
