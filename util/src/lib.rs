pub mod bitboard;
pub mod fen;

pub fn bit_loop<F: FnMut(u64)>(mut x: u64, mut f: F) {
    while x != 0 {
        f(x);
        x &= x - 1;
    }
}

pub fn square_of(bitboard: u64) -> u8 {
    bitboard.trailing_zeros() as u8
}