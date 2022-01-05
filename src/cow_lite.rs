//!
//! like cow
//!

use core::ops::Deref;

use alloc::vec::Vec;

#[derive(Debug, PartialEq)]
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

impl<'a, T> AsRef<T> for Cow<'a, T> {
    fn as_ref(&self) -> &T {
        match self {
            Cow::Owned(t) => t,
            Cow::Borrowed(t) => *t,
        }
    }
}

pub type CowBytes<'a> = Cow<'a, Vec<u8>>;
