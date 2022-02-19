use std::io::{BufReader, Result};
use std::io::{BufWriter, Write};
use std::net::TcpStream;

use xml::EventReader;

use crate::game_result::{Cause, GameResult, Score};
use crate::xml_node::{XmlNode, XmlState};
use game_lib::gamestate::Gamestate;
use game_lib::game_move::Move;
use crate::team::Team;

#[derive(Debug)]
pub struct Game {
    pub gamestate: Gamestate,
    pub room_id: String,
    pub stream: TcpStream,
    pub client_team: Team,
}

impl Game {
    fn send_move(&self, mut game_move: Move) {
        if self.client_team == Team::TWO {
            game_move.from ^= 63;
            game_move.to ^= 63;
        }

        let from_x = game_move.from / 8;
        let from_y = game_move.from % 8;
        let to_x = game_move.to / 8;
        let to_y = game_move.to % 8;

        log::info!(
            "Moving (Server_Coordinates) from ({},{}) to ({},{})",
            from_x,
            from_y,
            to_x,
            to_y
        );
        log::info!("Sending move...");

        BufWriter::new(&self.stream).write(format!("<room roomId=\"{}\"><data class=\"move\"><from x=\"{}\" y=\"{}\"/><to x=\"{}\" y=\"{}\"/></data></room>",
                                                   &self.room_id,
                                                   from_x,
                                                   from_y,
                                                   to_x,
                                                   to_y
        ).as_bytes()).expect("Failed to write move");
    }

    fn on_move_request(&mut self) {
        log::info!("Received MoveRequest");
        let mv = self.gamestate.best_move();
        self.send_move(mv);
    }

    fn on_receive_memento(&mut self, data_node: &XmlNode) {
        //TODO instead of reinstating the board, update it

        let gamestate_node = data_node.child("state").unwrap();

        let turn = gamestate_node
            .attributes
            .get("turn")
            .unwrap()
            .get(0)
            .unwrap()
            .parse::<u8>()
            .unwrap();

        let gamestate = Gamestate::from(&XmlState(gamestate_node, self.client_team));

        println!("{}", gamestate);
        println!("{}", gamestate.board);

        self.gamestate = gamestate;

        log::debug!(
            "\n[ReceivedMemento | Turn {}]\n{}",
            turn,
            self.gamestate.board
        );
    }

    fn on_receive_result(&mut self, data_node: &XmlNode) -> GameResult {
        let mut entries = vec![];

        for entry in &data_node.child("scores").unwrap().children {
            let player_attribs = &entry.child("player").unwrap().attributes;
            let score = entry.child("score").unwrap();

            let _player_name = player_attribs.get("name").unwrap().get(0).unwrap();
            let player_team = player_attribs
                .get("team")
                .unwrap()
                .get(0)
                .unwrap()
                .parse::<Team>()
                .unwrap();

            let cause = score.attributes.get("cause").unwrap().get(0).unwrap();
            let reason = score.attributes.get("reason").unwrap().get(0).unwrap();
            let cause = Cause::from_str(cause, reason).unwrap();

            let _sieg_punkte = &score.children.get(0).unwrap().data;
            let bernsteine = score.children.get(1).unwrap().data.parse::<u8>().unwrap();
            let _figur_vorne = &score.children.get(2).unwrap().data;

            entries.push((player_team, bernsteine, cause));
        }

        let p1 = entries[0];
        let p2 = entries[1];

        let (friendly, enemy) = if p1.0 == self.client_team {
            (p1, p2)
        } else {
            (p2, p1)
        };

        let (score, _cause) = if friendly.1 > enemy.1 {
            (Score::WIN(friendly.1, enemy.1), enemy.2)
        } else if friendly.1 < enemy.1 {
            (Score::LOSS(friendly.1, enemy.1), enemy.2)
        } else {
            (Score::DRAW(friendly.1), enemy.2)
        };

        GameResult { score } //TODO do something with cause
    }

    fn on_leave_session(&mut self, _data_node: &XmlNode) {}

    pub fn game_loop(&mut self) -> std::result::Result<GameResult, GameError> {
        let copy_of_stream = self.stream.try_clone().unwrap();
        let mut parser = EventReader::new(BufReader::new(&copy_of_stream));

        let mut game_result: Option<GameResult> = None;

        loop {
            let received = XmlNode::read_from(&mut parser);

            match received.name.as_str() {
                "protocol" => {
                    log::info!("Ending game");
                    return if let Some(res) = game_result {
                        Ok(res)
                    } else {
                        Err(GameError::EndedGameEarly)
                    };
                }
                "data" => {
                    let class = received
                        .attributes
                        .get("class")
                        .expect("Received node without class")
                        .get(0)
                        .unwrap();

                    match class.as_str() {
                        "memento" => {
                            self.on_receive_memento(&received);
                        }
                        "moveRequest" => {
                            self.on_move_request();
                        }
                        "welcomeMessage" => {
                            panic!("Received multiple welcome messages!")
                        }
                        "result" => {
                            game_result = Some(self.on_receive_result(&received));
                        }
                        class => {
                            panic!("Failed to match class: {}", class)
                        }
                    }
                }
                "left" => {
                    log::info!(
                        "Left game {}",
                        received.attributes.get("roomId").unwrap().get(0).unwrap()
                    );
                    return if let Some(res) = game_result {
                        Ok(res)
                    } else {
                        Err(GameError::LeftGameEarly)
                    };
                }
                name => {
                    panic!("Failed to match node '{}': {:?}", name, received)
                }
            }
        }
    }
}

impl Clone for Game {
    fn clone(&self) -> Self {
        let gamestate = self.gamestate;
        let room_id = self.room_id.clone();
        let stream = self.stream.try_clone().expect("Failed to clone stream");
        let team = self.client_team;

        Self {
            gamestate,
            room_id,
            stream,
            client_team: team,
        }
    }
}

pub enum Join<'a> {
    ANY,
    ROOM(&'a str),
    PREPARED(&'a str),
}

impl<'a> Join<'a> {
    pub fn connect(&self, network_address: &str) -> Result<Game> {
        let stream = TcpStream::connect(network_address)?;

        log::info!("Connected to server...");

        let mut writer = BufWriter::new(stream.try_clone().expect("Couldn't clone stream"));

        let _sent = match self {
            Join::ANY => writer.write("<protocol><join/>".as_bytes()),
            Join::ROOM(room_id) => {
                writer.write(format!("<protocol><joinRoom roomId=\"{}\"/>", room_id).as_bytes())
            }
            Join::PREPARED(reservation) => writer.write(
                format!(
                    "<protocol><joinPrepared reservationCode=\"{}\"/>",
                    reservation
                )
                .as_bytes(),
            ),
        }?;
        writer.flush()?;
        log::info!("Sent join-request to server");

        let mut parser = EventReader::new(BufReader::new(&stream));
        let joined = XmlNode::read_from(&mut parser);
        let welcome = XmlNode::read_from(&mut parser);

        let room_id = joined.attributes.get("roomId").unwrap().get(0).unwrap();
        let my_team: Team = welcome
            .attributes
            .get("color")
            .expect("No attribute named \"color\"")
            .get(0)
            .unwrap()
            .into();

        let node = XmlNode::read_from(&mut parser);

        match node.attributes.get("class") {
            None => {}
            Some(class) => match class[0].as_str() {
                "memento" => {
                    let gamestate_node = node.child("state").unwrap();

                    let gamestate = Gamestate::from(&XmlState(gamestate_node, my_team));

                    let game = Game {
                        gamestate,
                        room_id: room_id.clone(),
                        stream,
                        client_team: my_team,
                    };

                    log::info!("Joined {} as Team {:?}", game.room_id, game.client_team);
                    log::debug!("\n[Start | Turn 0]\n{}", game.gamestate.board);
                    println!("{}", gamestate);
                    println!("{}", gamestate.board);

                    return Ok(game);
                }
                "moveRequest" => {
                    log::error!("Received a moveRequest before constructing a game");
                    panic!()
                }
                &_ => {}
            },
        }
        panic!()
    }
}

#[derive(Debug)]
pub enum GameError {
    LeftGameEarly,
    EndedGameEarly,
}
