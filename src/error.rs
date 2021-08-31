use core::{cell, fmt::Debug};

use alloc::{boxed::Box};

#[derive(Debug)]
pub enum Error {
    StoreError(Box<dyn Debug>),

    #[cfg(feature = "cbor")]
    CborEncodeError(minicbor::encode::Error<core::convert::Infallible>),

    #[cfg(feature = "cbor")]
    CborDecodeError(minicbor::decode::Error),

    HeightError,
    BorrowMutError(cell::BorrowMutError),
    BorrowError(cell::BorrowError),
    LockReadError,
    /// When you load a store.
    TypeMissMatch,
}

#[cfg(feature = "cbor")]
impl From<minicbor::encode::Error<core::convert::Infallible>> for Error {
    fn from(e: minicbor::encode::Error<core::convert::Infallible>) -> Self {
        Self::CborEncodeError(e)
    }
}

#[cfg(feature = "cbor")]
impl From<minicbor::decode::Error> for Error {
    fn from(e: minicbor::decode::Error) -> Self {
        Self::CborDecodeError(e)
    }
}

impl From<cell::BorrowMutError> for Error {
    fn from(e: cell::BorrowMutError) -> Self {
        Self::BorrowMutError(e)
    }
}

impl From<cell::BorrowError> for Error {
    fn from(e: cell::BorrowError) -> Self {
        Self::BorrowError(e)
    }
}

pub type Result<T> = core::result::Result<T, Error>;
