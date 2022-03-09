use rand::thread_rng;
use std::time::{Duration, SystemTime};

use crate::traits::IGamestate;
use num_traits::{Bounded, Num, NumCast, ToPrimitive};
use rand::prelude::SliceRandom;
use std::fmt::{Debug, Display, Formatter};
use thincollections::thin_vec::ThinVec;

pub trait MonteCarloState: IGamestate + PartialEq {
    type EvalType: Num + Sized + Copy + NumCast + PartialOrd + Ord + Bounded + Display;

    /// Evaluate the current position
    fn evaluate(&self) -> Self::EvalType;
}

pub trait MonteCarlo: MonteCarloState {
    fn best_mcts_move(&self, calculation_time: Duration) -> Option<Self::MoveType> {
        let mut tree = MonteCarloTree::from(*self);

        let start_time = SystemTime::now();

        while SystemTime::now()
            .duration_since(start_time)
            .expect("Time error!")
            .as_millis()
            < calculation_time.as_millis()
        {
            tree.iterate();
        }

        let next_node = tree
            .root
            .children
            .iter()
            .max_by(|a, b| a.value.total_cmp(&b.value))
            .expect("Failed to find max node");

        let mut action = None;
        for game_move in tree.root.gamestate.available_moves() {
            let mut clone = *self;
            clone.apply_move(&game_move);
            clone.next_player();
            if clone == next_node.gamestate {
                action = Some(game_move);
            }
        }
        let best = action.expect("Failed to find move that develops to next node");
        Some(best)
    }
}

impl<E: MonteCarloState> MonteCarlo for E {}

#[derive(Clone, Debug)]
struct MctsNode<E: MonteCarlo + Copy + Debug> {
    children: Vec<MctsNode<E>>,
    gamestate: E,
    visits: u32,
    value: f32,
}

struct MonteCarloTree<E: MonteCarlo> {
    root: MctsNode<E>,
    path: Vec<usize>,
    exploration_constant: f32,
}

impl<E: MonteCarlo> MonteCarloTree<E> {
    fn traverse(&mut self) -> &MctsNode<E> {
        self.path.clear();
        let mut current = &self.root;
        while !current.is_leaf() {
            let (index, node) = current.max_ucb1(self.exploration_constant);
            self.path.push(index);
            current = node;
        }
        current
    }

    fn backprop(&mut self, value: f32) {
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

    pub fn iterate(&mut self) {
        //println!("Iterating!");

        let leaf = self.traverse();

        let mut score = 0.0;
        if leaf.visits == 0 {
            score = leaf.rollout().to_f32().expect("Failed to cast EvalType");
        } else {
            let mutable_leaf = self.follow_path();
            if !mutable_leaf.gamestate.game_over() {
                mutable_leaf.expand();
                score = mutable_leaf.children[0].rollout().to_f32().unwrap();
                self.path.push(0); //Push new index 0
                                   //println!("Pushed new leaf - root: {:?}", self.root)
            } else {
                score = mutable_leaf.gamestate.evaluate().to_f32().expect("f"); //TODO change is_client_turn
            }
        }
        self.backprop(score);
    }
}

impl<E: MonteCarlo> From<E> for MonteCarloTree<E> {
    fn from(state: E) -> Self {
        Self {
            root: MctsNode {
                children: vec![],
                gamestate: state,
                visits: 0,
                value: 0.0,
            },
            path: vec![],
            exploration_constant: 20000 as f32,
        }
    }
}

impl<E: MonteCarlo> Display for MonteCarloTree<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.root.pretty_print(0);
        Ok(())
    }
}

impl<E: MonteCarlo> MctsNode<E> {
    fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    fn rollout(&self) -> E::EvalType {
        let mut rng = thread_rng();
        let mut gamestate = self.gamestate;
        while !gamestate.game_over() {
            let legal = gamestate.available_moves();
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
        let legal = self.gamestate.available_moves();
        for action in &legal {
            let mut new_gamestate = self.gamestate.clone();
            new_gamestate.apply_move(action);
            new_gamestate.next_player();
            let mut child_node = Self {
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
        for i in 0..depth {
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
