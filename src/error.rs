use core::{cell, fmt::Debug};

use alloc::boxed::Box;

#[derive(Debug)]
pub enum Error {
    StoreError(Box<dyn Debug>),
    #[cfg(feature = "cbor")]
    CborError(serde_cbor::Error),
    HeightError,
    BorrowMutError(cell::BorrowMutError),
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

pub type Result<T> = core::result::Result<T, Error>;
