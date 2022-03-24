use socha_client_2022::client::{DebugMode, SCClient, SCClientDelegate};
use socha_client_2022::game::Move as SCMove;
use socha_client_2022::game::State as SCState;
use socha_client_2022::game::Team as SCTeam;
use socha_client_2022::util::SCResult;
use std::time::SystemTime;

use crate::algorithms::Algorithm;
use crate::game::{Fen, Gamestate, Team};
use socha_client_2022::protocol::GameResult;

struct ClientDelegate<E: Algorithm> {
    inner: E,
    result: Option<GameResult>,
}

impl<E: Algorithm> SCClientDelegate for ClientDelegate<E> {
    fn on_update_state(&mut self, state: &SCState) {
        let gamestate: Gamestate = state.clone().into();
        println!("{}", gamestate.to_fen());
        println!("{}", gamestate.board);
    }

    fn on_game_end(&mut self, result: &GameResult) {
        self.result = Some(result.clone());
        log::info!("{:?}", result);
    }

    fn on_welcome(&mut self, team: SCTeam) {
        let team: Team = team.into();
        log::info!("Joined as Team {}", team);
    }

    fn request_move(&mut self, sc_state: &SCState, my_team: SCTeam) -> SCMove {
        log::info!("Beginning move calculation");
        let start = SystemTime::now();

        let best = self
            .inner
            .best_move(sc_state.clone().into(), my_team.into());
        let stop = SystemTime::now().duration_since(start).unwrap();
        println!("Finished calculation {:?}", stop);
        let piece = best.piece;
        let mov = best.into();
        println!("Sending move: {} {}", piece, mov);
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
            result: None,
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

    pub fn connect(self, host: &str, port: u16) -> SCResult<GameResult> {
        self.inner.connect(host, port)
    }
}
