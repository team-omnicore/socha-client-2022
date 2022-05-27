use rand::thread_rng;
use std::time::{Duration, Instant};

use num_traits::{Bounded, Num, NumCast, ToPrimitive};
use rand::prelude::SliceRandom;
use std::fmt::{Display, Formatter};
use crate::algorithms::{Algorithm};
use crate::algorithms::heuristics::{EVAL_END};
use crate::game::{Gamestate, IGamestate, IMove, Move, Team};



pub trait MctsMove: IMove {}

pub trait MctsGameState: IGamestate + PartialEq {
    type EvalType: Num + Sized + Copy + NumCast + PartialOrd + Ord + Bounded + Display;

    fn evaluate(&self) -> Self::EvalType;
}

#[derive(Clone)]
pub struct MonteCarloTreeSearch {
    exploration_constant: f32,
    search_duration: Duration,
}

impl MonteCarloTreeSearch {

    pub fn new(
        exploration_constant: f32,
        search_duration: Duration,
    ) -> Self {
        Self {
            exploration_constant,
            search_duration
        }
    }

    fn recommend_move<E: MctsGameState>(&mut self, state: E, _: Team) -> E::MoveType {
        let mut tree = MonteCarloTree::from(state);

        let start = Instant::now();
        loop {
            tree.iterate(self.exploration_constant);

            let elapsed_time = start.elapsed();
            if elapsed_time.as_millis() >= self.search_duration.as_millis() {
                break;
            }
        }
        let best_index = tree.best_node_index();
        println!("Best index: {}", best_index);

        tree.root.pretty_print(0);

        state.available_moves_current_player()[best_index]
    }
}


#[derive(Clone)]
struct MonteCarloTree<E: MctsGameState> {
    root: MctsNode<E>,
    path: Vec<usize>,
}

impl<E: MctsGameState> MonteCarloTree<E> {

    fn from(state: E) -> Self {
        Self {
            root: MctsNode {
                children: vec![],
                gamestate: state,
                visits: 0,
                value: 0.0,
            },
            path: vec![],
        }
    }

    fn traverse(&mut self, exploration_constant: f32) -> &MctsNode<E> {
        self.path.clear();
        let mut current = &self.root;
        while !current.is_leaf() {
            let (index, node) = current.max_ucb1(exploration_constant);
            self.path.push(index);
            current = node;
        }
        current
    }

    fn backprop(&mut self, score: E::EvalType) {
        let value = score.to_f32().unwrap();
        let mut current = &mut self.root;
        current.value += value;
        current.visits += 1;
        for index in &self.path {
            current = &mut current.children[*index];
            current.value += value;
            current.visits += 1;
        }
    }

    fn follow_path(&mut self) -> &mut MctsNode<E> {
        let mut current = &mut self.root;
        for index in &self.path {
            current = &mut current.children[*index];
        }
        current
    }

    pub fn iterate(&mut self, exploration_constant : f32) {
        //println!("Iterating!");

        let leaf = self.traverse(exploration_constant);

        let score: E::EvalType;
        if leaf.visits == 0 {
            score = leaf.rollout();
        } else {
            let mutable_leaf = self.follow_path();
            if !mutable_leaf.gamestate.game_over() {
                mutable_leaf.expand();
                score = mutable_leaf.children[0].rollout();
                self.path.push(0); //Push new index 0
                //println!("Pushed new leaf - root: {:?}", self.root)
            } else {
                score = mutable_leaf.gamestate.evaluate();
            }
        }
        self.backprop(score);
    }

    pub fn best_node_index(&self) -> usize {
        let best_node = self.root.children.get(0);
        let mut best_index: usize = 0;
        for (index, node) in self.root.children.iter().enumerate() {
            if node.value > best_node.unwrap().value {
                best_index = index;
            }
        }
        best_index
    }
}


impl<E: MctsGameState> Display for MonteCarloTree<E> {
    fn fmt(&self, _: &mut Formatter<'_>) -> std::fmt::Result {
        self.root.pretty_print(0);
        Ok(())
    }
}

/** Node **/
#[derive(Clone)]
struct MctsNode<E: MctsGameState> {
    children: Vec<MctsNode<E>>,
    gamestate: E,
    visits: u32,
    value: f32,
}

impl<E: MctsGameState> MctsNode<E> {
    fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    fn rollout(&self) -> E::EvalType {
        let mut rng = thread_rng();
        let mut gamestate = self.gamestate;
        while !gamestate.game_over() && gamestate.count_moves_current_player() > 0 {
            let legal = gamestate.available_moves_current_player();
            let random_move = legal.choose(&mut rng).expect(&*format!(
                "Failed to select random move. length is: {}",
                legal.len()
            ));
            gamestate.apply_move(&random_move);
            gamestate.next_player();
        }
        gamestate.evaluate()
    }

    fn expand(&mut self) {
        let legal = self.gamestate.available_moves(self.gamestate.current_player());
        for action in &legal {
            let mut new_gamestate = self.gamestate.clone();
            new_gamestate.apply_move(action);
            new_gamestate.next_player();
            let child_node = Self {
                children: vec![],
                gamestate: new_gamestate,
                visits: 0,
                value: 0.0,
            };
            self.children.push(child_node);
        }
    }

    fn ucb1(&self, total_visits: u32, exploration_constant: f32) -> f32 {
        self.value as f32
            + exploration_constant * f32::sqrt(f32::ln(total_visits as f32) / (self.visits as f32))
    }

    fn max_ucb1(&self, exploration_constant: f32) -> (usize, &MctsNode<E>) {
        let mut max = f32::MIN;
        let mut node = None;
        let mut index = 0;
        for (i, child_node) in self.children.iter().enumerate() {
            let ucb1_score = child_node.ucb1(self.visits, exploration_constant);
            if ucb1_score > max {
                max = ucb1_score;
                node = Some(child_node);
                index = i;
            }
        }
        (index, node.unwrap())
    }

    fn pretty_print(&self, depth: u32) {
        let mut tabs = String::new();
        for _ in 0..depth {
            tabs.push_str("\t");
        }
        println!("{}{{depth={}:", tabs, depth);
        println!("{}\tvalue: {}", tabs, self.value);
        println!("{}\tvisit: {}", tabs, self.visits);
        println!("{}", self.gamestate);
        println!("{}}}", tabs);

        if depth <= 0 {
            for child in &self.children {
                child.pretty_print(depth + 1);
            }
        }
    }
}


/** Implementation for our GamesState **/
impl MctsGameState for Gamestate {
    type EvalType = i32;

    fn evaluate(&self) -> Self::EvalType {
        EVAL_END(&self, self.current_player)
    }
}

impl Algorithm for MonteCarloTreeSearch {
    fn best_move(&mut self, state: Gamestate, my_team: Team) -> Move {
        self.recommend_move(state, my_team)
    }
}