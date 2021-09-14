/// Serialize
/// T => Vec<u8>
#[cfg(feature = "cbor")]
mod cbor {
    use crate::{Error, Result};
    use alloc::string::ToString;
    use alloc::vec::Vec;
    use ciborium::ser::into_writer;
    use serde::Serialize;

    pub fn cbor_encode(t: impl Serialize) -> Result<Vec<u8>> {
        let mut value = Vec::new();
        into_writer(&t, &mut value).map_err(|e| Error::CborSerIoError(e.to_string()))?;
        Ok(value)
    }
}

#[cfg(feature = "cbor")]
pub use cbor::cbor_encode;
