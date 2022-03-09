use util::bitboard::Bitboard;

const NOT_FILE_A: u64 = !0x0101010101010101;
const NOT_FILE_B: u64 = !0x0202020202020202;
const NOT_FILE_G: u64 = !0x4040404040404040;
const NOT_FILE_H: u64 = !0x8080808080808080;

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

    Bitboard::from(moves)
}

#[inline]
pub fn robbe_lookup_moves(pos: u8) -> Bitboard {
    Bitboard::from(LOOKUP_ROBBEN[pos as usize])
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

    Bitboard::from(moves)
}

#[inline]
pub fn moewe_lookup_moves(pos: u8) -> Bitboard {
    Bitboard::from(LOOKUP_MOEWEN[pos as usize])
}

#[inline]
pub fn seestern_gen_moves(seestern: Bitboard, player: bool) -> Bitboard {
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

    Bitboard::from(moves)
}

#[inline]
pub fn seestern_lookup_moves(pos: u8, player: bool) -> Bitboard {
    Bitboard::from(LOOKUP_SEESTERN[(((player as u8) << 6) + pos) as usize])
}

#[inline]
pub fn muschel_gen_moves(muschel: Bitboard, player: bool) -> Bitboard {
    let muschel_loc = muschel.bits;

    let clip_file_a = muschel_loc & NOT_FILE_A;
    let clip_file_h = muschel_loc & NOT_FILE_H;

    let spot_1 = (clip_file_a << 7) * (player as u64);
    let spot_3 = (clip_file_h << 9) * (player as u64);

    let spot_5 = (clip_file_h >> 7) * (!player as u64);
    let spot_7 = (clip_file_a >> 9) * (!player as u64);

    let moves = spot_1 | spot_3 | spot_5 | spot_7;

    Bitboard::from(moves)
}

#[inline]
pub fn muschel_lookup_moves(pos: u8, player: bool) -> Bitboard {
    Bitboard::from(LOOKUP_MUSCHELN[(((player as u8) << 6) + pos) as usize])
}
