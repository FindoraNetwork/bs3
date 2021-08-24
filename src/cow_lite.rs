use core::ops::Deref;

use alloc::vec::Vec;

pub enum Cow<'a, T> {
    Owned(T),
    Borrowed(&'a T),
}

impl<'a, T> Deref for Cow<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Cow::Owned(v) => v,
            Cow::Borrowed(v) => *v,
        }
    }
}

pub type CowBytes<'a> = Cow<'a, Vec<u8>>;
