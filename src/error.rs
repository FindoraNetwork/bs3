use core::{cell, fmt::Debug};

use alloc::boxed::Box;

#[derive(Debug)]
pub enum Error {
    StoreError(Box<dyn Debug>),

    HeightError,
    BorrowMutError(cell::BorrowMutError),
    BorrowError(cell::BorrowError),
    LockReadError,
    /// When you load a store.
    TypeMissMatch,
    BytesLengthError,
    TypeCodeError,

    TryFromIntError(core::num::TryFromIntError),

    TryFromSliceError(core::array::TryFromSliceError),

    #[cfg(feature = "sled-backend")]
    SledError(sled::Error),

    #[cfg(feature = "sled-backend")]
    StdIoError(std::io::Error),
}

impl From<core::num::TryFromIntError> for Error {
    fn from(e: core::num::TryFromIntError) -> Self {
        Error::TryFromIntError(e)
    }
}

impl From<core::array::TryFromSliceError> for Error {
    fn from(e: core::array::TryFromSliceError) -> Self {
        Self::TryFromSliceError(e)
    }
}

#[cfg(feature = "sled-backend")]
impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        self::Error::StdIoError(e)
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

#[cfg(feature = "sled-backend")]
impl From<sled::Error> for Error {
    fn from(e: sled::Error) -> Self {
        self::Error::SledError(e)
    }
}

pub type Result<T> = core::result::Result<T, Error>;
