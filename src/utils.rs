/// Serialize
/// T => Vec<u8>
#[cfg(feature = "cbor")]
mod cbor {
    use crate::{Error, Result};
    use alloc::string::ToString;
    use alloc::vec::Vec;
    use ciborium::de::from_reader;
    use ciborium::ser::into_writer;

    use serde::Serialize;

    pub fn cbor_encode<T: Serialize>(t: &T) -> Result<Vec<u8>> {
        let mut value = Vec::new();
        cbor_encode_writer(t, &mut value)?;
        Ok(value)
    }

    pub fn cbor_encode_writer<T: Serialize>(t: &T, writer: &mut Vec<u8>) -> Result<()> {
        into_writer(&t, writer).map_err(|e| Error::CborSerIoError(e.to_string()))?;
        Ok(())
    }
}

#[cfg(feature = "cbor")]
pub use cbor::cbor_encode;
pub use cbor::cbor_encode_writer;
