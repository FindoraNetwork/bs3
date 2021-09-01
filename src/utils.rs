#[cfg(feature = "cbor")]
mod cbor {
    use crate::Result;
    use alloc::vec::Vec;
    use minicbor::Encode;

    pub fn cbor_encode(t: impl Encode) -> Result<Vec<u8>> {
        let mut value = Vec::new();
        minicbor::encode(t, &mut value)?;
        Ok(value)
    }
}

#[cfg(feature = "cbor")]
pub use cbor::cbor_encode;
