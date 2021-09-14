//!
//! backend is storage layer
//!

mod store;
pub use store::Store;

#[cfg(feature = "sled-backend")]
pub mod sled;
#[cfg(feature = "sled-backend")]
pub use self::sled::sled_db_open;
#[cfg(feature = "sled-backend")]
pub use self::sled::SledBackend;

// pub mod helper;

pub mod memory;
pub use memory::MemoryBackend;
