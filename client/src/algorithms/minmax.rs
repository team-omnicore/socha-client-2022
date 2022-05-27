use std::borrow::BorrowMut;
use crate::algorithms::{Algorithm, EvaluationFunction};
use crate::game::{Gamestate, IGamestate, Move, Team};
use num_traits::{Bounded, Num, NumCast, Zero};
use std::fmt::Display;
use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct MinMax<E: MinMaxState> {
    evaluation_function: fn(&E, Team) -> E::EvalType,
    max_search_duration: Duration,
}

pub trait MinMaxState: IGamestate {
    type EvalType: Num + Sized + Copy + NumCast + PartialOrd + Ord + Bounded + Display;
}

#[derive(Clone)]
struct MinMaxNode<E: MinMaxState> {
    children: Vec<MinMaxNode<E>>,
    state: E,
    evaluation: E::EvalType,
    mov: Option<E::MoveType>,
}

impl<E: MinMaxState> MinMaxNode<E> {
    fn from(state: E) -> Self {
        Self {
            children: vec![],
            state,
            evaluation: E::EvalType::zero(),
            mov: None,
        }
    }

    fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    fn expand(&mut self, team: Team, save_move: bool) {
        self.state.for_each_move(team, &mut |mov| {
            let mut child_state = self.state.clone();
            child_state.apply_move(&mov);
            child_state.next_player();

            let mut child_node = MinMaxNode::from(child_state);
            if save_move {
                child_node.mov = Some(mov);
            }
            self.children.push(child_node);
        });
    }

    fn sort_children(&mut self) {
        self.children.sort_by_key(|child| child.evaluation);
    }

    fn pretty_print(&self) {
        for child in self.children.iter() {
            println!("value: {}", child.evaluation);
        }
    }
}

#[derive(Clone)]
struct MinMaxTree<E: MinMaxState> {
    root: MinMaxNode<E>,
    evaluation_function: EvaluationFunction<E, E::EvalType>,
    my_team: Team,
    start_timer: Instant,
    max_search_duration: Duration,
}

impl<E: MinMaxState> MinMaxTree<E> {
    fn from(root_state: E, evaluation_function: EvaluationFunction<E, E::EvalType>, my_team: Team, start_timer: Instant, max_search_duration: Duration) -> Self {
        Self {
            root: MinMaxNode::from(root_state),
            evaluation_function,
            my_team,
            start_timer,
            max_search_duration,
        }
    }

    pub fn iterate(&mut self, depth: u8) -> E::MoveType {
        self.root.pretty_print();
        println!("------------------");
        self.root.sort_children();


        if self.root.is_leaf() {
            self.root.expand(self.my_team, true);
        }

        let mut move_value_index_pairs = vec![];
        //Maximizing player (Client player)
        for (index, child_node) in self.root.children.iter().enumerate() {
            let eval = self.min_max(child_node.state, depth, self.my_team.opponent(), E::EvalType::min_value(), E::EvalType::max_value());
            move_value_index_pairs.push((eval, child_node.mov.unwrap(), index));
        }

        for (value, _, index) in move_value_index_pairs.iter() {
            match self.root.children.get_mut(*index) {
                Some(child) => child.evaluation = *value,
                _ => {}
            }
        }

        let max = move_value_index_pairs.iter().max_by_key(|pair| pair.0);
        println!("Best Evaluation: {}", max.unwrap().0);
        println!("Depth {}", depth);
        max.unwrap().1.clone()
    }

    fn min_max(&self,
               state: E,
               depth: u8,
               team: Team,
               mut alpha: E::EvalType,
               mut beta: E::EvalType,
    ) -> E::EvalType {
        if depth == 0 || state.game_over() {
            let eval = (self.evaluation_function)(&state, self.my_team);
            //println!("{}", eval);
            return eval;
        }
        let is_maximizing = team == self.my_team;

        if self.start_timer.elapsed().as_millis() > self.max_search_duration.as_millis() {
            return if is_maximizing {
                alpha
            } else {
                beta
            }
        }



        if is_maximizing {
            //Maximizing player (Client player)
            let mut max_eval = E::EvalType::max_value();

            state.for_each_move(team, &mut |mov| {
                let mut child_state = state.clone();
                child_state.apply_move(&mov);
                child_state.next_player();

                let eval = self.min_max(child_state, depth - 1, team.opponent(), alpha, beta);
                max_eval = E::EvalType::max(max_eval, eval);
                alpha = E::EvalType::max(alpha, eval);

                if beta <= alpha { return; }
            });

            return max_eval;
        } else {
            //Minimizing player (Client player)
            let mut min_eval = E::EvalType::min_value();

            state.for_each_move(team, &mut |mov| {
                let mut child_state = state.clone();
                child_state.apply_move(&mov);
                child_state.next_player();

                let eval = self.min_max(child_state, depth - 1, team.opponent(), alpha, beta);
                min_eval = E::EvalType::min(min_eval, eval);
                beta = E::EvalType::min(beta, eval);

                if beta <= alpha { return; }
            });

            min_eval
        }
    }
}

impl<E: MinMaxState> MinMax<E> {
    pub fn new(
        evaluation_function: EvaluationFunction<E, E::EvalType>,
        max_search_duration: Duration,
    ) -> Self {
        Self {
            evaluation_function,
            max_search_duration,
        }
    }

    fn recommend_move(
        &mut self,
        state: E,
        my_team: Team,
    ) -> E::MoveType {
        let start_timer = Instant::now();

        let mut depth = 2;

        let mut tree = MinMaxTree::from(state, self.evaluation_function, my_team, start_timer, self.max_search_duration);

        let mut best_move = None;
        while start_timer.elapsed().as_millis() < self.max_search_duration.as_millis() {
            best_move = Some(tree.iterate(depth));
            depth += 1;
        }

        best_move.unwrap()
    }
}

impl Algorithm for MinMax<Gamestate> {
    fn best_move(&mut self, state: Gamestate, my_team: Team) -> Move {
        self.recommend_move(state, my_team)
    }
}

impl MinMaxState for Gamestate {
    type EvalType = i32;
}
