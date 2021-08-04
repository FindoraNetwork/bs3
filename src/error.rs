use core::fmt::Debug;

use alloc::boxed::Box;

#[derive(Debug)]
pub enum Error {
    StoreError(Box<dyn Debug>),
}

pub type Result<T> = core::result::Result<T, Error>;
