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

    fn sort_tree(&mut self) {
        if self.is_leaf() {
            return;
        }

        for child in &mut self.children {
            child.sort_children();
            child.sort_tree();
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
            max_search_duration
        }
    }

    pub fn iterate(&mut self, depth: u8) -> E::MoveType {
        self.root.sort_tree();

        let mut move_value_pairs = vec![];

        if self.root.is_leaf() {
            self.root.expand(self.my_team, true);
        }
        //Maximizing player (Client player)
        for child_node in self.root.children.iter_mut() {
            let eval = Self::min_max(child_node, depth, self.my_team, E::EvalType::min_value(), E::EvalType::max_value(), self.start_timer, self.max_search_duration, self.my_team, self.evaluation_function);
            move_value_pairs.push((eval, child_node.mov.unwrap()));
        }

        let max = move_value_pairs.iter().max_by_key(|pair| pair.0);
        println!("Best Evaluation: {}", max.unwrap().0);
        println!("Depth {}", depth);
        max.unwrap().1.clone()
    }

    fn min_max(
        node: &mut MinMaxNode<E>,
        depth: u8,
        team: Team,
        mut alpha: E::EvalType,
        mut beta: E::EvalType,
        start_timer: Instant,
        max_search_duration: Duration,
        my_team: Team,
        evaluation_function: EvaluationFunction<E, E::EvalType>,
    ) -> E::EvalType {

        if depth == 0 || node.state.game_over() || start_timer.elapsed().as_millis() > max_search_duration.as_millis() {
            let eval = (evaluation_function)(&node.state, my_team);
            node.evaluation = eval;
            return eval;
        }

        if node.is_leaf() {
            node.expand(team, false);
        }

        let is_maximizing = team == my_team;


        return if is_maximizing {
            //Maximizing player (Client player)
            let mut max_eval = E::EvalType::max_value();

            for child_node in node.children.iter_mut() {
                let eval = Self::min_max(child_node, depth - 1, team.opponent(), alpha, beta, start_timer, max_search_duration, my_team, evaluation_function);
                max_eval = E::EvalType::max(max_eval, eval);
                alpha = E::EvalType::max(alpha, eval);

                if beta <= alpha { break };
            }

            max_eval
        } else {
            //Minimizing player (Client player)
            let mut min_eval = E::EvalType::min_value();

            for child_node in node.children.iter_mut() {
                let eval = Self::min_max(child_node, depth - 1, team.opponent(), alpha, beta, start_timer, max_search_duration, my_team, evaluation_function);
                min_eval = E::EvalType::min(min_eval, eval);
                beta = E::EvalType::min(beta, eval); // TODO alpha beta?

                if beta <= alpha { break };
            }

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
