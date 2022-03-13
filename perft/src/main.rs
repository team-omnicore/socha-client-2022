use std::io;
use std::io::Write;
use std::time::{Duration, SystemTime};

use chrono::Local;
use game_algorithms::traits::IGamestate;
use game_lib::board::Board;
use game_lib::game_move::Move;
use game_lib::gamestate::Gamestate;
use game_lib::move_generation::{
    moewe_lookup_moves, muschel_lookup_moves, robbe_lookup_moves, seestern_lookup_moves,
};
use game_lib::piece::PieceType;
use game_lib::score::Score;
use rand::thread_rng;
use separator::Separatable;

use util::{bit_loop, square_of};

fn main() {
    let mut rng = thread_rng();
    let board = Board::new_random(&mut rng);

    let gamestate = Gamestate {
        board,
        round: 0,
        is_maximizing_player: true,
        score: Score { bytes: [0, 0] },
    };

    println!("{}", gamestate.board);

    begin_perft(gamestate, 10);
}

fn begin_perft(origin_state: Gamestate, depth: u8) {
    println!("|Depth|Move-count|Elapsed time|Speed|Multiplier|\n|---|---|---|---|---|");

    let mut last_time = 1f64;
    let mut last_count = 1f64;
    let mut time_multiplier = 1f64;
    let mut count_multiplier;
    for i in 0..depth {
        let current_time = Local::now();
        let estimated_duration = Duration::from_secs_f64(time_multiplier * last_time);
        let estimated_finish = current_time
            .checked_add_signed(chrono::Duration::from_std(estimated_duration).unwrap())
            .unwrap();

        print!(
            "> Estimated time is {:.1?} - Working since {} - Finishes {}",
            estimated_duration,
            current_time.format("%Y-%m-%d %H:%M:%S"),
            estimated_finish.format("%Y-%m-%d %H:%M:%S")
        );
        io::stdout().flush().unwrap();

        let start = SystemTime::now();
        let count = perft(&origin_state, i);
        let duration = start.elapsed().unwrap();

        count_multiplier = count as f64 / last_count;
        last_count = count as f64;

        time_multiplier = duration.as_secs_f64() / last_time;
        last_time = duration.as_secs_f64();

        let speed = count as f64 / duration.as_secs_f64();

        println!(
            "\rPerft {:>1} | {:>18} | {:>10.2?} | {:>16} | {:>3.1}x",
            i,
            count.separated_string(),
            duration,
            pretty_print_speed(speed),
            count_multiplier
        );
    }
}

#[inline]
fn perft(state: &Gamestate, depth: u8) -> u64 {
    let seesterne = state.board.seesterne & state.board.friendly;
    let robben = state.board.robben & state.board.friendly;
    let muscheln = state.board.muscheln & state.board.friendly;
    let moewen = state.board.moewen & state.board.friendly;
    let unoccupied = !state.board.friendly;

    let mut counter = 0;

    bit_loop(robben.bits, |robbe| {
        let from = square_of(robbe);
        let legal = robbe_lookup_moves(from) & unoccupied;

        if depth > 0 {
            bit_loop(legal.bits, |mov| {
                let to = square_of(mov);
                let set_move = Move {
                    from,
                    to,
                    piece: PieceType::ROBBE,
                };

                let mut clone = *state;
                clone.apply_move(&set_move);
                clone.next_player();

                if !clone.game_over() {
                    counter += perft(&clone, depth - 1)
                }
            });
        } else {
            counter += legal.bits.count_ones() as u64;
        }
    });

    bit_loop(moewen.bits, |moewe| {
        let from = square_of(moewe);
        let legal = moewe_lookup_moves(from) & unoccupied;

        if depth > 0 {
            bit_loop(legal.bits, |mov| {
                let to = square_of(mov);
                let set_move = Move {
                    from,
                    to,
                    piece: PieceType::MOEWE,
                };

                let mut clone = *state;
                clone.apply_move(&set_move);
                clone.next_player();

                if !clone.game_over() {
                    counter += perft(&clone, depth - 1)
                }
            });
        } else {
            counter += legal.bits.count_ones() as u64;
        }
    });

    bit_loop(seesterne.bits, |seestern| {
        let from = square_of(seestern);
        let legal = seestern_lookup_moves(from, state.is_maximizing_player) & unoccupied;

        if depth > 0 {
            bit_loop(legal.bits, |mov| {
                let to = square_of(mov);
                let set_move = Move {
                    from,
                    to,
                    piece: PieceType::SEESTERN,
                };

                let mut clone = *state;
                clone.apply_move(&set_move);
                clone.next_player();

                if !clone.game_over() {
                    counter += perft(&clone, depth - 1)
                }
            });
        } else {
            counter += legal.bits.count_ones() as u64;
        }
    });

    bit_loop(muscheln.bits, |muschel| {
        let from = square_of(muschel);
        let legal = muschel_lookup_moves(from, state.is_maximizing_player) & unoccupied;

        if depth > 0 {
            bit_loop(legal.bits, |mov| {
                let to = square_of(mov);
                let set_move = Move {
                    from,
                    to,
                    piece: PieceType::MUSCHEL,
                };

                let mut clone = *state;
                clone.apply_move(&set_move);
                clone.next_player();

                if !clone.game_over() {
                    counter += perft(&clone, depth - 1)
                }
            });
        } else {
            counter += legal.bits.count_ones() as u64;
        }
    });
    counter
}

#[inline]
pub fn pretty_print_speed(speed: f64) -> String {
    if speed < 1000f64 {
        format!("{:.1} Nodes/sec", speed)
    } else if speed < 1000000f64 {
        format!("{:.1} KNodes/sec", speed / 1000f64)
    } else if speed < 1000000000f64 {
        format!("{:.1} MNodes/sec", speed / 1000000f64)
    } else {
        format!("{:.2} GNodes/sec", speed / 1000000000f64)
    }
}
