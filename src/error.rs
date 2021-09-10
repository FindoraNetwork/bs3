use core::{cell, fmt::Debug};

use alloc::boxed::Box;

#[derive(Debug)]
pub enum Error {
    StoreError(Box<dyn Debug>),

    #[cfg(feature = "cbor")]
    CborDeIoError,

    #[cfg(feature = "cbor")]
    CborSerIoError(ciborium::ser::Error<core::convert::Infallible>),

    HeightError,
    BorrowMutError(cell::BorrowMutError),
    BorrowError(cell::BorrowError),
    LockReadError,
    /// When you load a store.
    TypeMissMatch,

    #[cfg(feature = "sled-backend")]
    SledError(sled::Error),
}

// #[cfg(feature = "cbor")]
// impl From<ciborium::de::Error<core::convert::Infallible>> for Error {
//     fn from(e: ciborium::de::Error<core::convert::Infallible>) -> Self {
//         self::Error::CborDeIoError(e)
//     }
// }

#[cfg(feature = "cbor")]
impl From<ciborium::ser::Error<core::convert::Infallible>> for Error {
    fn from(e: ciborium::ser::Error<core::convert::Infallible>) -> Self {
        self::Error::CborSerIoError(e)
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
