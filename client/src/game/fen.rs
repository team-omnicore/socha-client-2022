use lazy_static::lazy_static;
use regex::Regex;

pub trait Fen: Sized {
    type Err;
    fn to_fen(&self) -> String;
    fn load_fen(fen: &str) -> Result<Self, Self::Err>;
}

static FEN_REGEX_STRING: &str = r"^(?P<pieces>(?P<r1>(?:[1-8]|[msrhMSRH]\*?){1,8})/(?P<r2>(?:[1-8]|[msrhMSRH]\*?){1,8})/(?P<r3>(?:[1-8]|[msrhMSRH]\*?){1,8})/(?P<r4>(?:[1-8]|[msrhMSRH]\*?){1,8})/(?P<r5>(?:[1-8]|[msrhMSRH]\*?){1,8})/(?P<r6>(?:[1-8]|[msrhMSRH]\*?){1,8})/(?P<r7>(?:[1-8]|[msrhMSRH]\*?){1,8})/(?P<r8>(?:[1-8]|[msrhMSRH]\*?){1,8})) (?P<round>(?:[1-5]?[0-9]|60)) (?P<points>(?P<pt_red>[0-3])/(?P<pt_blu>[0-3]))$";

lazy_static! {
    pub static ref FEN_REGEX: Regex = Regex::new(FEN_REGEX_STRING).unwrap();
}
