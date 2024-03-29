use core::fmt;
use std::fmt::Formatter;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

#[macro_export]
macro_rules! bitboard {
    ($b: expr) => {
        Bitboard { bits: $b }
    };
    () => {
        Bitboard { bits: 0u64 }
    };
}

#[derive(Debug, Copy, Clone)]
pub struct Bitboard {
    pub bits: u64,
}

const K1: u64 = 0x00FF00FF00FF00FF;
const K2: u64 = 0x0000FFFF0000FFFF;

const D1: u64 = 0x5500550055005500;
const D2: u64 = 0x3333000033330000;
const D3: u64 = 0x0f0f0f0f00000000;

const H1: u64 = 0x5555555555555555;
const H2: u64 = 0x3333333333333333;
const H3: u64 = 0x0F0F0F0F0F0F0F0F;

impl Bitboard {
    #[inline]
    pub const fn new() -> Self {
        Bitboard { bits: 0 }
    }

    #[inline]
    pub fn get_bit(&self, pos: u8) -> bool {
        (self.bits >> pos & 1u64) != 0
    }

    #[inline]
    pub fn set_bit(&mut self, pos: u8) {
        self.bits |= 1 << pos
    }

    #[inline]
    pub fn clear_bit(&mut self, pos: u8) {
        self.bits &= !(1 << pos)
    }

    #[inline]
    pub fn toggle_bit(&mut self, pos: u8) {
        self.bits ^= 1 << pos
    }

    #[inline]
    pub fn clear_all(&mut self) {
        self.bits = 0
    }

    #[inline]
    pub fn set_all(&mut self) {
        self.bits = u64::MAX
    }

    #[inline]
    pub fn reverse(&mut self) {
        self.bits = self.bits.reverse_bits()
    }

    #[inline]
    pub fn rotate90_clockwise(&mut self) -> &mut Self {
        self.flip_diagonal_a1_h8();
        self.flip_vertical();
        self
    }

    #[inline]
    pub fn rotate90_anti_clockwise(&mut self) -> &mut Self {
        self.flip_vertical();
        self.flip_diagonal_a1_h8();
        self
    }

    #[inline]
    pub fn flip_vertical(&mut self) -> &mut Self {
        let mut x = self.bits;
        x = ((x >> 8) & K1) | ((x & K1) << 8);
        x = ((x >> 16) & K2) | ((x & K2) << 16);
        x = (x >> 32) | (x << 32);
        self.bits = x;
        self
    }

    #[inline]
    pub fn flip_horizontal(&mut self) -> &mut Self {
        let mut x = self.bits;
        x = ((x >> 1) & H1) + 2 * (x & H1);
        x = ((x >> 2) & H2) + 4 * (x & H2);
        x = ((x >> 4) & H3) + 16 * (x & H3);
        self.bits = x;
        self
    }

    #[inline]
    pub fn flip_diagonal_a1_h8(&mut self) -> &mut Self {
        let mut x = self.bits;
        let mut _t: u64 = 0;
        _t = D3 & (x ^ (x << 28));
        x ^= _t ^ (_t >> 28);
        _t = D2 & (x ^ (x << 14));
        x ^= _t ^ (_t >> 14);
        _t = D1 & (x ^ (x << 7));
        x ^= _t ^ (_t >> 7);
        self.bits = x;
        self
    }

    #[inline]
    pub fn rotate180(&mut self) -> &mut Self {
        let mut x = self.bits;
        x = ((x >> 1) & H1) | ((x & H1) << 1);
        x = ((x >> 2) & H2) | ((x & H2) << 2);
        x = ((x >> 4) & H3) | ((x & H3) << 4);
        x = ((x >> 8) & K1) | ((x & K1) << 8);
        x = ((x >> 16) & K2) | ((x & K2) << 16);
        x = (x >> 32) | (x << 32);
        self.bits = x;
        self
    }

    #[inline]
    pub fn swap_with(&mut self, other: &mut Bitboard) {
        self.bits ^= other.bits;
        other.bits ^= self.bits;
        self.bits ^= other.bits;
    }

    #[inline]
    pub fn overlaps(&self, rhs: Bitboard) -> Bitboard {
        bitboard!(((rhs.bits & self.bits) != 0) as u64 * u64::MAX)
    }
}

impl From<u64> for Bitboard {
    fn from(bits: u64) -> Self {
        Bitboard { bits }
    }
}

impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut out: String = String::from("╔══════════════════════════╗\n");
        let mut index_min = 56 + 8;
        let mut index_max = 64 + 8;
        for _ in 0..8 {
            index_max -= 8;
            index_min -= 8;
            out.push_str("║  ");
            for j in index_min..index_max {
                out.push(if self.get_bit(j) { 'x' } else { '-' });
                out.push_str("  ");
            }
            out.push_str("║\n");
        }
        out.push_str("╚══════════════════════════╝");
        write!(f, "{}", out)
    }
}

impl BitAnd for Bitboard {
    type Output = Bitboard;

    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard {
            bits: self.bits & rhs.bits,
        }
    }
}

impl BitOr for Bitboard {
    type Output = Bitboard;

    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard {
            bits: self.bits | rhs.bits,
        }
    }
}

impl BitXor for Bitboard {
    type Output = Bitboard;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Bitboard {
            bits: self.bits ^ rhs.bits,
        }
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.bits &= rhs.bits
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.bits |= rhs.bits
    }
}

impl BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.bits ^= rhs.bits
    }
}

impl Not for Bitboard {
    type Output = Bitboard;

    fn not(self) -> Self::Output {
        Bitboard { bits: !self.bits }
    }
}

impl PartialEq for Bitboard {
    fn eq(&self, other: &Self) -> bool {
        self.bits == other.bits
    }

    fn ne(&self, other: &Self) -> bool {
        self.bits != other.bits
    }
}

impl fmt::Binary for Bitboard {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:066b}", self.bits)
    }
}

impl fmt::LowerHex for Bitboard {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:x}", self.bits)
    }
}

impl fmt::UpperHex for Bitboard {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:X}", self.bits)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_90_anticlockwise() {
        assert_eq!(
            bitboard!(0xff00000000000000),
            *bitboard!(0x8080808080808080).rotate90_anti_clockwise()
        )
    }

    #[test]
    fn test_rotate_90_clockwise() {
        assert_eq!(
            bitboard!(0x8080808080808080),
            *bitboard!(0xff00000000000000).rotate90_clockwise()
        )
    }

    #[test]
    fn test_rotate_180() {
        assert_eq!(
            bitboard!(0x101010101010101),
            *bitboard!(0x8080808080808080).rotate180()
        )
    }

    //TODO finish testing
}
