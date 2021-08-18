use alloc::vec::Vec;

pub enum BytesRef<'a> {
    Owned(Vec<u8>),
    Borrow(&'a [u8])
}

impl<'a> Into<Vec<u8>> for BytesRef<'a> {
    fn into(self) -> Vec<u8> {
        match self {
            Self::Owned(v) => v,
            Self::Borrow(v) => Vec::from(v),
        }
    }
}

