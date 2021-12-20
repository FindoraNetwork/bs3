use core::{cell, fmt::Debug};

use alloc::boxed::Box;
use alloc::string::String;

#[derive(Debug)]
pub enum Error {
    StoreError(Box<dyn Debug>),

    #[cfg(feature = "cbor")]
    CborDeError(ciborium::de::Error<core::convert::Infallible>),

    #[cfg(feature = "cbor")]
    CborSerError(ciborium::ser::Error<core::convert::Infallible>),

    #[cfg(feature = "cbor")]
    CborDeIoError(String),
    #[cfg(feature = "cbor")]
    CborSerIoError(String),

    HeightError,
    BorrowMutError(cell::BorrowMutError),
    BorrowError(cell::BorrowError),
    LockReadError,
    /// When you load a store.
    TypeMissMatch,

    #[cfg(feature = "sled-backend")]
    SledError(sled::Error),

    #[cfg(feature = "sled-backend")]
    StdIoError(std::io::Error),

    JsonError(serde_json::Error),
}

#[cfg(feature = "sled-backend")]
impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        self::Error::StdIoError(e)
    }
}

#[cfg(feature = "cbor")]
impl From<ciborium::de::Error<core::convert::Infallible>> for Error {
    fn from(e: ciborium::de::Error<core::convert::Infallible>) -> Self {
        self::Error::CborDeError(e)
    }
}

#[cfg(feature = "cbor")]
impl From<ciborium::ser::Error<core::convert::Infallible>> for Error {
    fn from(e: ciborium::ser::Error<core::convert::Infallible>) -> Self {
        self::Error::CborSerError(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::JsonError(e)
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
