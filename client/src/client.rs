use std::time::{Duration, SystemTime};

use log::info;
use socha_client_2022::client::{DebugMode, SCClient, SCClientDelegate};
use socha_client_2022::game::Move as SCMove;
use socha_client_2022::game::State as SCState;
use socha_client_2022::game::Team as SCTeam;
use socha_client_2022::protocol::GameResult;
use socha_client_2022::protocol::ScoreCause;
use socha_client_2022::util::SCResult;

use crate::algorithms::Algorithm;
use crate::game::{Fen, Gamestate, Team};

struct ClientDelegate<E: Algorithm> {
    inner: E,
    client_team: Option<Team>,
    time_tracker: Option<SystemTime>,
}

impl<E: Algorithm> SCClientDelegate for ClientDelegate<E> {
    fn on_update_state(&mut self, state: &SCState) {
        let gamestate: Gamestate = state.clone().into();
        println!("{}", gamestate.to_fen());
        println!("{}", gamestate.board);

        if let Some(player) = self.client_team {
            if let Some(time) = self.time_tracker {
                if gamestate.current_player == player {
                    info!(
                        "Enemy took: {:?}",
                        time.elapsed()
                            .unwrap()
                            .saturating_sub(Duration::from_millis(17)) //heuristic value
                    );
                }
            }
        }
    }

    fn on_game_end(&mut self, result: &GameResult, my_team: SCTeam) {
        let red_score = result
            .scores()
            .iter()
            .find_map(|(player, score)| {
                if player.team() == SCTeam::One {
                    Some(score)
                } else {
                    None
                }
            })
            .unwrap();
        let blue_score = result
            .scores()
            .iter()
            .find_map(|(player, score)| {
                if player.team() == SCTeam::Two {
                    Some(score)
                } else {
                    None
                }
            })
            .unwrap();
        let (my_score, enemy_score) = match my_team {
            SCTeam::One => (red_score, blue_score),
            SCTeam::Two => (blue_score, red_score),
        };
        if let Some(winner) = result.winner() {
            if my_team == winner.team() {
                info!(
                    "WON({} : {}) -> Points({} : {})    [{:?}]",
                    red_score.parts().get(1).unwrap(),
                    blue_score.parts().get(1).unwrap(),
                    red_score.parts().get(0).unwrap(),
                    blue_score.parts().get(0).unwrap(),
                    enemy_score.cause()
                );
            } else {
                info!(
                    "LOST({} : {}) -> Points({} : {})    [{:?}]",
                    red_score.parts().get(1).unwrap(),
                    blue_score.parts().get(1).unwrap(),
                    red_score.parts().get(0).unwrap(),
                    blue_score.parts().get(0).unwrap(),
                    my_score.cause()
                );
            }
        } else {
            if !matches!(my_score.cause(), ScoreCause::Regular) {
                info!(
                    "SCORE({} : {})  [{:?}]",
                    red_score.parts().get(1).unwrap(),
                    blue_score.parts().get(1).unwrap(),
                    my_score.cause()
                );
            } else if !matches!(enemy_score.cause(), ScoreCause::Regular) {
                info!(
                    "SCORE({} : {})  [{:?}]",
                    red_score.parts().get(1).unwrap(),
                    blue_score.parts().get(1).unwrap(),
                    enemy_score.cause()
                );
            } else {
                info!(
                    "DRAW({}) [{:?}]",
                    red_score.parts().get(0).unwrap(),
                    my_score.cause()
                );
            }
        }
    }

    fn on_welcome(&mut self, team: SCTeam) {
        let team: Team = team.into();
        info!("Joined as Team {}", team);
        self.client_team = Some(team);
    }

    fn request_move(&mut self, sc_state: &SCState, my_team: SCTeam) -> SCMove {
        info!("Beginning move calculation");
        self.time_tracker = Some(SystemTime::now());
        let best = self
            .inner
            .best_move(sc_state.clone().into(), my_team.into());
        info!(
            "Finished calculation {:?}",
            self.time_tracker.unwrap().elapsed().unwrap()
        );
        self.time_tracker = Some(SystemTime::now());
        let piece = best.piece;
        let mov = best.into();
        info!("Sending move: {} {}", piece, mov);
        mov
    }
}

pub struct Client<A: Algorithm> {
    inner: SCClient<ClientDelegate<A>>,
}

impl<A: Algorithm> Client<A> {
    pub fn new(algorithm: A, reservation_code: Option<String>) -> Self {
        let algorithm_wrapper = ClientDelegate {
            inner: algorithm,
            client_team: None,
            time_tracker: None,
        };
        Self {
            inner: SCClient::new(
                algorithm_wrapper,
                DebugMode {
                    debug_reader: false,
                    debug_writer: false,
                },
                reservation_code,
            ),
        }
    }

    pub fn connect(&mut self, host: &str, port: u16) -> SCResult<GameResult> {
        self.inner.connect(host, port)
    }

    pub fn team(&self) -> Option<Team> {
        if let Some(team) = self.inner.team() {
            Some(Team::from(team))
        } else {
            None
        }
    }
}
