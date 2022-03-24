pub trait ShortForm<T>: Sized {
    type Err;
    fn to_short_form(&self) -> T;
    fn from_short_form(_: &T) -> Result<Self, Self::Err>;
}
