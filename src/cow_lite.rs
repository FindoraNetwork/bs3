pub enum Cow<'a, T> {
    Owned(T),
    Borrowed(&'a T),
}
