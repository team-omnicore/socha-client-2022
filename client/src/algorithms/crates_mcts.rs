use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::{Duration, Instant};
use mcts::*;
use mcts::tree_policy::*;
use mcts::transposition_table::*;
use thincollections::thin_vec::ThinVec;
use crate::algorithms::Algorithm;
use crate::algorithms::heuristics::EVAL_2603_1;
use crate::game::{Gamestate, IGamestate, Move, Team};

impl TranspositionHash for Gamestate {
    fn hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        Hash::hash(self, &mut hasher);
        hasher.finish()
    }
}

#[derive(Clone)]
struct MyMCTS;

struct MyEvaluator;

impl GameState for Gamestate {
    type Move = Move;
    type Player = Team;
    type MoveList = ThinVec<Move>;

    fn current_player(&self) -> Self::Player {
        self.current_player
    }

    fn available_moves(&self) -> Self::MoveList {
        self.available_moves_current_player()
    }

    fn make_move(&mut self, mov: &Self::Move) {
        self.apply_move(mov)
    }
}

impl Evaluator<MyMCTS> for MyEvaluator {
    type StateEvaluation = i32;

    fn evaluate_new_state(&self, state: &Gamestate, _: &MoveList<MyMCTS>, _: Option<SearchHandle<MyMCTS>>) -> (Vec<MoveEvaluation<MyMCTS>>, Self::StateEvaluation) {

        /*
            let mut move_evals = vec![];
            for m in moves {
                let mut new_state = state.clone();
                new_state.apply_move(m);
                move_evals.push(EVAL_2603_1(&new_state, new_state.current_player) as i64)
            } */

            (vec![] ,EVAL_2603_1(state, state.current_player))
    }

    fn evaluate_existing_state(&self, state: &Gamestate, _: &Self::StateEvaluation, _: SearchHandle<MyMCTS>) -> Self::StateEvaluation {
        EVAL_2603_1(state, state.current_player)
    }

    fn interpret_evaluation_for_player(&self, evaluation: &Self::StateEvaluation, _: &Player<MyMCTS>) -> i64 {
        *evaluation as i64
    }
}

impl MCTS for MyMCTS {
    type State = Gamestate;
    type Eval = MyEvaluator;
    type TreePolicy = UCTPolicy;
    type NodeData = ();
    type TranspositionTable = ApproxTable<Self>;
    type ExtraThreadData = ();

    fn cycle_behaviour(&self) -> CycleBehaviour<Self> {
        CycleBehaviour::UseCurrentEvalWhenCycleDetected
    }
}
#[derive(Clone)]
pub struct CratesMCTS {
    exploration_constant: f64,
    search_duration: Duration,
}

impl CratesMCTS {
    pub fn new(
        exploration_constant: f64,
        search_duration: Duration,
    ) -> Self {
        Self {
            exploration_constant,
            search_duration
        }
    }
}

impl Algorithm for CratesMCTS {

    fn best_move(&mut self, state: Gamestate, _: Team) -> Move {

        let mut mcts = MCTSManager::new(state, MyMCTS, MyEvaluator, UCTPolicy::new(self.exploration_constant),
                                        ApproxTable::new(1024));

        let start = Instant::now();
        loop {
            mcts.playout();

            let elapsed_time = start.elapsed();
            if elapsed_time.as_millis() >= self.search_duration.as_millis() {
                break;
            }
        }

        mcts.tree().debug_moves();
        println!("{}", mcts.tree().diagnose());

        mcts.best_move().unwrap()
    }
}