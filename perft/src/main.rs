use game_lib::fen::FenString;
use game_lib::gamestate::Gamestate;
use game_lib::team::Team;

fn main() {

    let fen = String::from("m5r1/4sH*1m/1s4S1/8/3h2r1/8/3S4/MR2R2M 39 1/0");

    let g = Gamestate::load_fen(&*fen, Team::BLUE).unwrap();
    println!("{}", g.board);
    println!("{}", g.to_fen());
    println!("{}", fen);

    let _b = Gamestate::load_fen(&*g.to_fen(), Team::BLUE).unwrap();

    //println!("{}", b.board);

    //println!("{}", b.to_fen());
}
