pub trait FenString {
    fn to_fen(&self)->String;
    fn load_fen()->Self;
}