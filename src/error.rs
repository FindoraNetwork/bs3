use core::{cell, fmt::Debug};

use alloc::boxed::Box;

#[derive(Debug)]
pub enum Error {
    StoreError(Box<dyn Debug>),
    #[cfg(feature = "cbor")]
    CborError(serde_cbor::Error),
    HeightError,
    BorrowMutError(cell::BorrowMutError),
    BorrowError(cell::BorrowError),
}

#[cfg(feature = "cbor")]
impl From<serde_cbor::Error> for Error {
    fn from(e: serde_cbor::Error) -> Self {
        Self::CborError(e)
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
