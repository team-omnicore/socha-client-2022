use crate::player::Player;
use client::algorithms::Algorithm;
use client::game::{Board, Gamestate, IGamestate, Team};
use rand::prelude::SliceRandom;
use rand::{thread_rng, Rng};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;

pub struct Server<A: Algorithm> {
    players: HashMap<String, Player<A>>,
    filename: String,
}

impl<A: Algorithm> Server<A> {
    pub fn new(working_file: String) -> Self {
        Self {
            players: Default::default(),
            filename: working_file,
        }
    }

    pub fn init(&mut self) {
        self.load_elo_scores(&self.filename.clone());
    }

    pub fn add_player(&mut self, player: Player<A>) {
        self.players.insert(player.name.clone(), player);
    }

    pub fn play_match(&mut self) {
        self.load_elo_scores(&self.filename.clone());

        let mut rng = thread_rng();
        let v: Vec<_> = self.players.iter().collect();

        let mut players: Vec<_> = v.choose_multiple(&mut rng, 2).cloned().collect();

        let mut players = [players[0].1.clone(), players[1].1.clone()];
        players.shuffle(&mut rng);

        println!("{} {:^24} {}", players[0], "vs", players[1]);

        let mut m = Match::new(players.clone(), &mut rng);
        let winner = m.play_game();
        self.load_elo_scores(&self.filename.clone());
        self.adjust_elo_scores(winner, players.clone());
        self.write_elo_scores(&self.filename);
    }

    fn adjust_elo_scores(&mut self, winner: Option<Team>, players: [Player<A>; 2]) {
        let bonus = if let Some(winner) = winner {
            1f32 - winner as u8 as f32
        } else {
            0.5f32
        };

        let k = 12f32;

        let p1 = &players[0];
        let p2 = &players[1];

        let r1 = players[0].elo_score;
        let r2 = players[1].elo_score;

        let e1 = 10f32.powf(r1 / 400f32);
        let e2 = 10f32.powf(r2 / 400f32);

        let r1_ = r1 + k * (bonus - (e1 / (e1 + e2)));
        let r2_ = r2 + k * ((1f32 - bonus) - (e2 / (e1 + e2)));

        if let Some(p1) = self.players.get_mut(&p1.name) {
            p1.elo_score = r1_;
        }
        if let Some(p2) = self.players.get_mut(&p2.name) {
            p2.elo_score = r2_;
        }
    }

    fn load_elo_scores(&mut self, filename: &String) {
        let contents = fs::read_to_string(filename);
        if contents.is_ok() {
            let contents = contents.unwrap();
            let players = contents.split(", ");
            for x in players {
                let split: Vec<_> = x.split(":").collect();
                let player_name = split[0].replace("\"", "").replace("{", "");
                let start_elo = split[1].find("elo=").unwrap();
                let end_elo = split[1].find("}").unwrap();
                let player_elo = split[1]
                    .get((start_elo + 4)..end_elo)
                    .unwrap()
                    .parse::<f32>()
                    .unwrap();

                let p = self.players.get_mut(&*player_name);
                if let Some(player) = p {
                    player.elo_score = player_elo;
                }
            }
        }
    }

    fn write_elo_scores(&self, filepath: &String) {
        let mut f = File::options()
            .create(true)
            .write(true)
            .open(filepath)
            .expect("Failed to create file");
        f.write_all(format!("{:?}", self.players).as_bytes());
    }
}

pub struct Match<A: Algorithm> {
    current_state: Gamestate,
    players: [Player<A>; 2],
}

impl<A: Algorithm> Match<A> {
    pub fn new<R: Rng>(players: [Player<A>; 2], rng: &mut R) -> Self {
        let board = Board::new_random(rng);
        let start_state = Gamestate::new(board);

        Self {
            current_state: start_state,
            players,
        }
    }

    fn do_turn(&mut self) {
        //println!("{}", self.current_state);
        //println!("Current player: {}", self.current_state.current_player);
        //println!("{}", self.current_state.board);
        let turn = self.current_state.round;
        let mut player = &mut self.players[Team::for_turn(turn) as usize];
        let mov = player.request_move(self.current_state, Team::for_turn(turn));
        self.current_state.apply_move(&mov);
        self.current_state.next_player();
    }

    pub fn play_game(&mut self) -> Option<Team> {
        while !self.current_state.game_over() {
            self.do_turn()
        }
        self.current_state.winner()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        let k = 32f32;

        let bonus = 0f32;

        let r1 = 1400f32;
        let r2 = 1100f32;

        let e1 = 10f32.powf(r1 / 400f32);
        let e2 = 10f32.powf(r2 / 400f32);

        let r1_ = r1 + k * (bonus - (e1 / (e1 + e2)));
        let r2_ = r2 + k * ((1f32 - bonus) - (e2 / (e1 + e2)));

        println!("{}", r1_);
        println!("{}", r2_);
    }
}
