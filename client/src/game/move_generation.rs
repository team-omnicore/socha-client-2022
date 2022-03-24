use crate::bitboard;
use crate::game::{Bitboard, Team};

const NOT_FILE_A: u64 = !0x0101010101010101;
const NOT_FILE_B: u64 = !0x0202020202020202;
const NOT_FILE_G: u64 = !0x4040404040404040;
const NOT_FILE_H: u64 = !0x8080808080808080;

//------------------------------------------------------------------------------------------------//

const LOOKUP_ROBBEN: [u64; 64] = [
    0x20400,
    0x50800,
    0xA1100,
    0x142200,
    0x284400,
    0x508800,
    0xA01000,
    0x402000,
    0x2040004,
    0x5080008,
    0xA110011,
    0x14220022,
    0x28440044,
    0x50880088,
    0xA0100010,
    0x40200020,
    0x204000402,
    0x508000805,
    0xA1100110A,
    0x1422002214,
    0x2844004428,
    0x5088008850,
    0xA0100010A0,
    0x4020002040,
    0x20400040200,
    0x50800080500,
    0xA1100110A00,
    0x142200221400,
    0x284400442800,
    0x508800885000,
    0xA0100010A000,
    0x402000204000,
    0x2040004020000,
    0x5080008050000,
    0xA1100110A0000,
    0x14220022140000,
    0x28440044280000,
    0x50880088500000,
    0xA0100010A00000,
    0x40200020400000,
    0x204000402000000,
    0x508000805000000,
    0xA1100110A000000,
    0x1422002214000000,
    0x2844004428000000,
    0x5088008850000000,
    0xA0100010A0000000,
    0x4020002040000000,
    0x400040200000000,
    0x800080500000000,
    0x1100110A00000000,
    0x2200221400000000,
    0x4400442800000000,
    0x8800885000000000,
    0x100010A000000000,
    0x2000204000000000,
    0x4020000000000,
    0x8050000000000,
    0x110A0000000000,
    0x22140000000000,
    0x44280000000000,
    0x88500000000000,
    0x10A00000000000,
    0x20400000000000,
];
const LOOKUP_MOEWEN: [u64; 64] = [
    0x102,
    0x205,
    0x40A,
    0x814,
    0x1028,
    0x2050,
    0x40A0,
    0x8040,
    0x10201,
    0x20502,
    0x40A04,
    0x81408,
    0x102810,
    0x205020,
    0x40A040,
    0x804080,
    0x1020100,
    0x2050200,
    0x40A0400,
    0x8140800,
    0x10281000,
    0x20502000,
    0x40A04000,
    0x80408000,
    0x102010000,
    0x205020000,
    0x40A040000,
    0x814080000,
    0x1028100000,
    0x2050200000,
    0x40A0400000,
    0x8040800000,
    0x10201000000,
    0x20502000000,
    0x40A04000000,
    0x81408000000,
    0x102810000000,
    0x205020000000,
    0x40A040000000,
    0x804080000000,
    0x1020100000000,
    0x2050200000000,
    0x40A0400000000,
    0x8140800000000,
    0x10281000000000,
    0x20502000000000,
    0x40A04000000000,
    0x80408000000000,
    0x102010000000000,
    0x205020000000000,
    0x40A040000000000,
    0x814080000000000,
    0x1028100000000000,
    0x2050200000000000,
    0x40A0400000000000,
    0x8040800000000000,
    0x201000000000000,
    0x502000000000000,
    0xA04000000000000,
    0x1408000000000000,
    0x2810000000000000,
    0x5020000000000000,
    0xA040000000000000,
    0x4080000000000000,
];

#[inline]
pub fn robbe_gen_moves(robbe: Bitboard) -> Bitboard {
    let robbe_loc = robbe.bits;

    let clip_file_ab = NOT_FILE_A & NOT_FILE_B;
    let clip_file_a = NOT_FILE_A;
    let clip_file_h = NOT_FILE_H;
    let clip_file_ag = NOT_FILE_H & NOT_FILE_G;

    let spot_1 = (robbe_loc & clip_file_ab) << 6;
    let spot_2 = (robbe_loc & clip_file_a) << 15;
    let spot_3 = (robbe_loc & clip_file_h) << 17;
    let spot_4 = (robbe_loc & clip_file_ag) << 10;

    let spot_5 = (robbe_loc & clip_file_ag) >> 6;
    let spot_6 = (robbe_loc & clip_file_h) >> 15;
    let spot_7 = (robbe_loc & clip_file_a) >> 17;
    let spot_8 = (robbe_loc & clip_file_ab) >> 10;

    let moves = spot_1 | spot_2 | spot_3 | spot_4 | spot_5 | spot_6 | spot_7 | spot_8;

    bitboard!(moves)
}

#[inline]
pub fn moewe_gen_moves(moewe: Bitboard) -> Bitboard {
    let moewe_loc = moewe.bits;

    let clip_file_a = moewe_loc & NOT_FILE_A;
    let clip_file_h = moewe_loc & NOT_FILE_H;

    let spot_2 = moewe_loc << 8;
    let spot_4 = clip_file_h << 1;
    let spot_6 = moewe_loc >> 8;
    let spot_8 = clip_file_a >> 1;

    let moves = spot_2 | spot_4 | spot_6 | spot_8;

    bitboard!(moves)
}

#[inline]
pub fn robbe_lookup_moves(pos: u8) -> Bitboard {
    bitboard!(LOOKUP_ROBBEN[pos as usize])
}

#[inline]
pub fn moewe_lookup_moves(pos: u8) -> Bitboard {
    bitboard!(LOOKUP_MOEWEN[pos as usize])
}

#[inline]
pub fn muschel_lookup_moves(pos: u8, player: Team) -> Bitboard {
    bitboard!(LOOKUP_MUSCHELN[(((player as u8) << 6) + pos) as usize])
}

#[inline]
pub fn seestern_lookup_moves(pos: u8, player: Team) -> Bitboard {
    bitboard!(LOOKUP_SEESTERN[(((player as u8) << 6) + pos) as usize])
}

//------------------------------------------------------------------------------------------------//

#[cfg(not(feature = "straight_board"))]
const LOOKUP_SEESTERN: [u64; 128] = [
    0x202,
    0x504,
    0xA08,
    0x1410,
    0x2820,
    0x5040,
    0xA080,
    0x4000,
    0x20202,
    0x50405,
    0xA080A,
    0x141014,
    0x282028,
    0x504050,
    0xA080A0,
    0x400040,
    0x2020200,
    0x5040500,
    0xA080A00,
    0x14101400,
    0x28202800,
    0x50405000,
    0xA080A000,
    0x40004000,
    0x202020000,
    0x504050000,
    0xA080A0000,
    0x1410140000,
    0x2820280000,
    0x5040500000,
    0xA080A00000,
    0x4000400000,
    0x20202000000,
    0x50405000000,
    0xA080A000000,
    0x141014000000,
    0x282028000000,
    0x504050000000,
    0xA080A0000000,
    0x400040000000,
    0x2020200000000,
    0x5040500000000,
    0xA080A00000000,
    0x14101400000000,
    0x28202800000000,
    0x50405000000000,
    0xA080A000000000,
    0x40004000000000,
    0x202020000000000,
    0x504050000000000,
    0xA080A0000000000,
    0x1410140000000000,
    0x2820280000000000,
    0x5040500000000000,
    0xA080A00000000000,
    0x4000400000000000,
    0x202000000000000,
    0x405000000000000,
    0x80A000000000000,
    0x1014000000000000,
    0x2028000000000000,
    0x4050000000000000,
    0x80A0000000000000,
    0x40000000000000,
    0x200,
    0x501,
    0xA02,
    0x1404,
    0x2808,
    0x5010,
    0xA020,
    0x4040,
    0x20002,
    0x50105,
    0xA020A,
    0x140414,
    0x280828,
    0x501050,
    0xA020A0,
    0x404040,
    0x2000200,
    0x5010500,
    0xA020A00,
    0x14041400,
    0x28082800,
    0x50105000,
    0xA020A000,
    0x40404000,
    0x200020000,
    0x501050000,
    0xA020A0000,
    0x1404140000,
    0x2808280000,
    0x5010500000,
    0xA020A00000,
    0x4040400000,
    0x20002000000,
    0x50105000000,
    0xA020A000000,
    0x140414000000,
    0x280828000000,
    0x501050000000,
    0xA020A0000000,
    0x404040000000,
    0x2000200000000,
    0x5010500000000,
    0xA020A00000000,
    0x14041400000000,
    0x28082800000000,
    0x50105000000000,
    0xA020A000000000,
    0x40404000000000,
    0x200020000000000,
    0x501050000000000,
    0xA020A0000000000,
    0x1404140000000000,
    0x2808280000000000,
    0x5010500000000000,
    0xA020A00000000000,
    0x4040400000000000,
    0x2000000000000,
    0x105000000000000,
    0x20A000000000000,
    0x414000000000000,
    0x828000000000000,
    0x1050000000000000,
    0x20A0000000000000,
    0x4040000000000000,
];
#[cfg(not(feature = "straight_board"))]
const LOOKUP_MUSCHELN: [u64; 128] = [
    0x200,
    0x400,
    0x800,
    0x1000,
    0x2000,
    0x4000,
    0x8000,
    0x0,
    0x20002,
    0x40004,
    0x80008,
    0x100010,
    0x200020,
    0x400040,
    0x800080,
    0x0,
    0x2000200,
    0x4000400,
    0x8000800,
    0x10001000,
    0x20002000,
    0x40004000,
    0x80008000,
    0x0,
    0x200020000,
    0x400040000,
    0x800080000,
    0x1000100000,
    0x2000200000,
    0x4000400000,
    0x8000800000,
    0x0,
    0x20002000000,
    0x40004000000,
    0x80008000000,
    0x100010000000,
    0x200020000000,
    0x400040000000,
    0x800080000000,
    0x0,
    0x2000200000000,
    0x4000400000000,
    0x8000800000000,
    0x10001000000000,
    0x20002000000000,
    0x40004000000000,
    0x80008000000000,
    0x0,
    0x200020000000000,
    0x400040000000000,
    0x800080000000000,
    0x1000100000000000,
    0x2000200000000000,
    0x4000400000000000,
    0x8000800000000000,
    0x0,
    0x2000000000000,
    0x4000000000000,
    0x8000000000000,
    0x10000000000000,
    0x20000000000000,
    0x40000000000000,
    0x80000000000000,
    0x0,
    0x0,
    0x100,
    0x200,
    0x400,
    0x800,
    0x1000,
    0x2000,
    0x4000,
    0x0,
    0x10001,
    0x20002,
    0x40004,
    0x80008,
    0x100010,
    0x200020,
    0x400040,
    0x0,
    0x1000100,
    0x2000200,
    0x4000400,
    0x8000800,
    0x10001000,
    0x20002000,
    0x40004000,
    0x0,
    0x100010000,
    0x200020000,
    0x400040000,
    0x800080000,
    0x1000100000,
    0x2000200000,
    0x4000400000,
    0x0,
    0x10001000000,
    0x20002000000,
    0x40004000000,
    0x80008000000,
    0x100010000000,
    0x200020000000,
    0x400040000000,
    0x0,
    0x1000100000000,
    0x2000200000000,
    0x4000400000000,
    0x8000800000000,
    0x10001000000000,
    0x20002000000000,
    0x40004000000000,
    0x0,
    0x100010000000000,
    0x200020000000000,
    0x400040000000000,
    0x800080000000000,
    0x1000100000000000,
    0x2000200000000000,
    0x4000400000000000,
    0x0,
    0x1000000000000,
    0x2000000000000,
    0x4000000000000,
    0x8000000000000,
    0x10000000000000,
    0x20000000000000,
    0x40000000000000,
];

#[cfg(not(feature = "straight_board"))]
#[inline]
pub fn seestern_gen_moves(seestern: Bitboard, player: Team) -> Bitboard {
    let seestern_loc = seestern.bits;

    let clip_file_a = seestern_loc & NOT_FILE_H;
    let clip_file_h = seestern_loc & NOT_FILE_A;

    let spot_1 = clip_file_a << 9;
    let spot_3 = clip_file_h << 7;
    let spot_5 = clip_file_h >> 9;
    let spot_7 = clip_file_a >> 7;

    let spot_4 = (clip_file_h >> 1) * (player as u64);
    let spot_8 = (clip_file_a << 1) * (!player as u64);

    let moves = spot_1 | spot_3 | spot_5 | spot_7 | spot_4 | spot_8;

    bitboard!(moves)
}

#[cfg(not(feature = "straight_board"))]
#[inline]
pub fn muschel_gen_moves(muschel: Bitboard, player: Team) -> Bitboard {
    let muschel_loc = muschel.bits;

    let clip_file_a = muschel_loc & NOT_FILE_H;
    let clip_file_h = muschel_loc & NOT_FILE_A;

    let spot_1 = (clip_file_a << 9) * (!player as u64);
    let spot_3 = (clip_file_h << 7) * (player as u64);

    let spot_5 = (clip_file_h >> 9) * (player as u64);
    let spot_7 = (clip_file_a >> 7) * (!player as u64);

    let moves = spot_1 | spot_3 | spot_5 | spot_7;

    bitboard!(moves)
}

//------------------------------------------------------------------------------------------------//

#[cfg(feature = "straight_board")]
const LOOKUP_SEESTERN: [u64; 128] = [
    0x200,
    0x500,
    0xA00,
    0x1400,
    0x2800,
    0x5000,
    0xA000,
    0x4000,
    0x20003,
    0x50007,
    0xA000E,
    0x14001C,
    0x280038,
    0x500070,
    0xA000E0,
    0x4000C0,
    0x2000300,
    0x5000700,
    0xA000E00,
    0x14001C00,
    0x28003800,
    0x50007000,
    0xA000E000,
    0x4000C000,
    0x200030000,
    0x500070000,
    0xA000E0000,
    0x14001C0000,
    0x2800380000,
    0x5000700000,
    0xA000E00000,
    0x4000C00000,
    0x20003000000,
    0x50007000000,
    0xA000E000000,
    0x14001C000000,
    0x280038000000,
    0x500070000000,
    0xA000E0000000,
    0x4000C0000000,
    0x2000300000000,
    0x5000700000000,
    0xA000E00000000,
    0x14001C00000000,
    0x28003800000000,
    0x50007000000000,
    0xA000E000000000,
    0x4000C000000000,
    0x200030000000000,
    0x500070000000000,
    0xA000E0000000000,
    0x14001C0000000000,
    0x2800380000000000,
    0x5000700000000000,
    0xA000E00000000000,
    0x4000C00000000000,
    0x3000000000000,
    0x7000000000000,
    0xE000000000000,
    0x1C000000000000,
    0x38000000000000,
    0x70000000000000,
    0xE0000000000000,
    0xC0000000000000,
    0x300,
    0x700,
    0xE00,
    0x1C00,
    0x3800,
    0x7000,
    0xE000,
    0xC000,
    0x30002,
    0x70005,
    0xE000A,
    0x1C0014,
    0x380028,
    0x700050,
    0xE000A0,
    0xC00040,
    0x3000200,
    0x7000500,
    0xE000A00,
    0x1C001400,
    0x38002800,
    0x70005000,
    0xE000A000,
    0xC0004000,
    0x300020000,
    0x700050000,
    0xE000A0000,
    0x1C00140000,
    0x3800280000,
    0x7000500000,
    0xE000A00000,
    0xC000400000,
    0x30002000000,
    0x70005000000,
    0xE000A000000,
    0x1C0014000000,
    0x380028000000,
    0x700050000000,
    0xE000A0000000,
    0xC00040000000,
    0x3000200000000,
    0x7000500000000,
    0xE000A00000000,
    0x1C001400000000,
    0x38002800000000,
    0x70005000000000,
    0xE000A000000000,
    0xC0004000000000,
    0x300020000000000,
    0x700050000000000,
    0xE000A0000000000,
    0x1C00140000000000,
    0x3800280000000000,
    0x7000500000000000,
    0xE000A00000000000,
    0xC000400000000000,
    0x2000000000000,
    0x5000000000000,
    0xA000000000000,
    0x14000000000000,
    0x28000000000000,
    0x50000000000000,
    0xA0000000000000,
    0x40000000000000,
];
#[cfg(feature = "straight_board")]
const LOOKUP_MUSCHELN: [u64; 128] = [
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x2,
    0x5,
    0xA,
    0x14,
    0x28,
    0x50,
    0xA0,
    0x40,
    0x200,
    0x500,
    0xA00,
    0x1400,
    0x2800,
    0x5000,
    0xA000,
    0x4000,
    0x20000,
    0x50000,
    0xA0000,
    0x140000,
    0x280000,
    0x500000,
    0xA00000,
    0x400000,
    0x2000000,
    0x5000000,
    0xA000000,
    0x14000000,
    0x28000000,
    0x50000000,
    0xA0000000,
    0x40000000,
    0x200000000,
    0x500000000,
    0xA00000000,
    0x1400000000,
    0x2800000000,
    0x5000000000,
    0xA000000000,
    0x4000000000,
    0x20000000000,
    0x50000000000,
    0xA0000000000,
    0x140000000000,
    0x280000000000,
    0x500000000000,
    0xA00000000000,
    0x400000000000,
    0x2000000000000,
    0x5000000000000,
    0xA000000000000,
    0x14000000000000,
    0x28000000000000,
    0x50000000000000,
    0xA0000000000000,
    0x40000000000000,
    0x200,
    0x500,
    0xA00,
    0x1400,
    0x2800,
    0x5000,
    0xA000,
    0x4000,
    0x20000,
    0x50000,
    0xA0000,
    0x140000,
    0x280000,
    0x500000,
    0xA00000,
    0x400000,
    0x2000000,
    0x5000000,
    0xA000000,
    0x14000000,
    0x28000000,
    0x50000000,
    0xA0000000,
    0x40000000,
    0x200000000,
    0x500000000,
    0xA00000000,
    0x1400000000,
    0x2800000000,
    0x5000000000,
    0xA000000000,
    0x4000000000,
    0x20000000000,
    0x50000000000,
    0xA0000000000,
    0x140000000000,
    0x280000000000,
    0x500000000000,
    0xA00000000000,
    0x400000000000,
    0x2000000000000,
    0x5000000000000,
    0xA000000000000,
    0x14000000000000,
    0x28000000000000,
    0x50000000000000,
    0xA0000000000000,
    0x40000000000000,
    0x200000000000000,
    0x500000000000000,
    0xA00000000000000,
    0x1400000000000000,
    0x2800000000000000,
    0x5000000000000000,
    0xA000000000000000,
    0x4000000000000000,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
];

#[cfg(feature = "straight_board")]
#[inline]
pub fn seestern_gen_moves(seestern: Bitboard, player: Team) -> Bitboard {
    let seestern_loc = seestern.bits;

    let clip_file_a = seestern_loc & NOT_FILE_A;
    let clip_file_h = seestern_loc & NOT_FILE_H;

    let spot_1 = clip_file_a << 7;
    let spot_3 = clip_file_h << 9;
    let spot_5 = clip_file_h >> 7;
    let spot_7 = clip_file_a >> 9;

    let spot_2 = (seestern_loc << 8) * (player as u64);
    let spot_6 = (seestern_loc >> 8) * (!player as u64);

    let moves = spot_1 | spot_2 | spot_3 | spot_5 | spot_6 | spot_7;

    bitboard!(moves)
}

#[cfg(feature = "straight_board")]
#[inline]
pub fn muschel_gen_moves(muschel: Bitboard, player: Team) -> Bitboard {
    let muschel_loc = muschel.bits;

    let clip_file_a = muschel_loc & NOT_FILE_A;
    let clip_file_h = muschel_loc & NOT_FILE_H;

    let spot_1 = (clip_file_a << 7) * (player as u64);
    let spot_3 = (clip_file_h << 9) * (player as u64);

    let spot_5 = (clip_file_h >> 7) * (!player as u64);
    let spot_7 = (clip_file_a >> 9) * (!player as u64);

    let moves = spot_1 | spot_3 | spot_5 | spot_7;

    bitboard!(moves)
}

//------------------------------------------------------------------------------------------------//

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::Team;

    const CRITICAL_POSITIONS: [Bitboard; 3] = [
        bitboard!(0x800000000),   //Center
        bitboard!(0x20),          //Bottom edge
        bitboard!(0x10000000000), //Left edge
    ];

    #[test]
    fn test_robbe_gen_moves_center() {
        let to_one = bitboard!(0x14220022140000);
        assert_eq!(robbe_gen_moves(CRITICAL_POSITIONS[0]), to_one);
    }

    #[test]
    fn test_robbe_gen_moves_edge() {
        let to_one = bitboard!(0x508800);
        assert_eq!(robbe_gen_moves(CRITICAL_POSITIONS[1]), to_one);
    }

    #[test]
    fn test_robbe_gen_moves_below() {
        let to_one = bitboard!(0x204000402000000);
        assert_eq!(robbe_gen_moves(CRITICAL_POSITIONS[2]), to_one);
    }

    #[test]
    fn test_moewe_gen_moves_center() {
        let to_one = bitboard!(0x81408000000);
        assert_eq!(moewe_gen_moves(CRITICAL_POSITIONS[0]), to_one);
    }

    #[test]
    fn test_moewe_gen_moves_edge() {
        let to_one = bitboard!(0x2050);
        assert_eq!(moewe_gen_moves(CRITICAL_POSITIONS[1]), to_one);
    }

    #[test]
    fn test_moewe_gen_moves_below() {
        let to_one = bitboard!(0x1020100000000);
        assert_eq!(moewe_gen_moves(CRITICAL_POSITIONS[2]), to_one);
    }

    #[test]
    fn test_seestern_gen_moves_center() {
        let to_one = bitboard!(0x141014000000);
        let to_two = bitboard!(0x140414000000);
        assert_eq!(seestern_gen_moves(CRITICAL_POSITIONS[0], Team::ONE), to_one);
        assert_eq!(seestern_gen_moves(CRITICAL_POSITIONS[0], Team::TWO), to_two);
    }

    #[test]
    fn test_seestern_gen_moves_edge() {
        let to_one = bitboard!(0x5040);
        let to_two = bitboard!(0x5010);
        assert_eq!(seestern_gen_moves(CRITICAL_POSITIONS[1], Team::ONE), to_one);
        assert_eq!(seestern_gen_moves(CRITICAL_POSITIONS[1], Team::TWO), to_two);
    }

    #[test]
    fn test_seestern_gen_moves_below() {
        let to_one = bitboard!(0x2020200000000);
        let to_two = bitboard!(0x2000200000000);
        assert_eq!(seestern_gen_moves(CRITICAL_POSITIONS[2], Team::ONE), to_one);
        assert_eq!(seestern_gen_moves(CRITICAL_POSITIONS[2], Team::TWO), to_two);
    }

    #[test]
    fn test_muschel_gen_moves_center() {
        let to_one = bitboard!(0x100010000000);
        let to_two = bitboard!(0x40004000000);
        assert_eq!(muschel_gen_moves(CRITICAL_POSITIONS[0], Team::ONE), to_one);
        assert_eq!(muschel_gen_moves(CRITICAL_POSITIONS[0], Team::TWO), to_two);
    }

    #[test]
    fn test_muschel_gen_moves_edge() {
        let to_one = bitboard!(0x4000);
        let to_two = bitboard!(0x1000);
        assert_eq!(muschel_gen_moves(CRITICAL_POSITIONS[1], Team::ONE), to_one);
        assert_eq!(muschel_gen_moves(CRITICAL_POSITIONS[1], Team::TWO), to_two);
    }

    #[test]
    fn test_muschel_gen_moves_below() {
        let to_one = bitboard!(0x2000200000000);
        let to_two = bitboard!(0x0);
        assert_eq!(muschel_gen_moves(CRITICAL_POSITIONS[2], Team::ONE), to_one);
        assert_eq!(muschel_gen_moves(CRITICAL_POSITIONS[2], Team::TWO), to_two);
    }

    #[test]
    fn test_lookups_moewe() {
        for i in 0..64 {
            assert_eq!(moewe_gen_moves(bitboard!(1 << i)), moewe_lookup_moves(i))
        }
    }

    #[test]
    fn test_lookups_robbe() {
        for i in 0..64 {
            assert_eq!(robbe_gen_moves(bitboard!(1 << i)), robbe_lookup_moves(i))
        }
    }

    #[test]
    fn test_lookups_seestern() {
        for i in 0..64 {
            assert_eq!(
                seestern_gen_moves(bitboard!(1 << i), Team::ONE),
                seestern_lookup_moves(i, Team::ONE)
            );
            assert_eq!(
                seestern_gen_moves(bitboard!(1 << i), Team::TWO),
                seestern_lookup_moves(i, Team::TWO)
            );
        }
    }

    #[test]
    fn test_lookups_muschel() {
        for i in 0..64 {
            assert_eq!(
                muschel_gen_moves(bitboard!(1 << i), Team::ONE),
                muschel_lookup_moves(i, Team::ONE)
            );
            assert_eq!(
                muschel_gen_moves(bitboard!(1 << i), Team::TWO),
                muschel_lookup_moves(i, Team::TWO)
            );
        }
    }
}
