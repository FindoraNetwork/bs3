#[cfg(feature = "cbor")]

mod cbor {
    use crate::Result;
    use alloc::vec::Vec;
    use ciborium::ser::into_writer;
    use serde::{Deserialize, Serialize};

    pub fn cbor_encode(t: impl Serialize) -> Result<Vec<u8>> {
        let mut value = Vec::new();
        into_writer(&t, &mut value)?;
        Ok(value)
    }
}

#[cfg(feature = "cbor")]
pub use cbor::cbor_encode;
