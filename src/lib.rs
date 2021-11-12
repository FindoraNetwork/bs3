//!
//! ## The project structure is divided into four levels
//!
//! * store /*Application Layer*/
//!   * map
//!   * vec
//!   * value
//! * snapshot /*Middle layer with transactional operations*/
//!   * storage
//!   * transaction
//! * model /*Cache Layer*/
//!   * map
//!     * Btree<K,Operation<V>>
//!   * vec
//!     * Btree<usize,Operation<V>>
//!   * value
//!     * Operation<V>
//! * backend /*Storage Layer*/
//!   * memory
//!   * sled
//!
//! ## Store Logic
//!
//! Data is recorded by block height, For example:
//! bytes():Indicates that the data is of type bytes
//! ```json
//! insert {1:1},{2:2},{3:3}; /*height=0*/
//! commit; /*height=1,cache is nil*/
//! {
//!     bytes({name_space}-ty): bytes(StoreType(3)),
//!     bytes({name_space}-ch): bytes(StoreHeight(1)),
//!     bytes({name_space}-kw-{hex(1)}-{:00000000000000000001}): bytes(Operation::Update(1)),
//!     bytes({name_space}-kw-{hex(2)}-{:00000000000000000001}): bytes(Operation::Update(2)),
//!     bytes({name_space}-kw-{hex(3)}-{:00000000000000000001}): bytes(Operation::Update(3)),
//! }
//! commit; /*height=2*/
//! {
//!     bytes({name_space}-ty): bytes(StoreType(3)),
//!     bytes({name_space}-ch): bytes(StoreHeight(1)),
//!     bytes({name_space}-kw-{hex(1)}-{:00000000000000000001}): bytes(Operation::Update(1)),
//!     bytes({name_space}-kw-{hex(2)}-{:00000000000000000001}): bytes(Operation::Update(2)),
//!     bytes({name_space}-kw-{hex(3)}-{:00000000000000000001}): bytes(Operation::Update(3)),
//!     bytes({name_space}-ch): bytes(StoreHeight(2)),
//! }
//! remove key=1; /*Invalid, cache is nil*/
//! insert {4:4}; /*height=2*/
//! commit; /*height=3*/
//! {
//!     bytes({name_space}-ty): bytes(StoreType(3)),
//!     bytes({name_space}-ch): bytes(StoreHeight(1)),
//!     bytes({name_space}-kw-{hex(1)}-{:00000000000000000001}): bytes(Operation::Update(1)),
//!     bytes({name_space}-kw-{hex(2)}-{:00000000000000000001}): bytes(Operation::Update(2)),
//!     bytes({name_space}-kw-{hex(3)}-{:00000000000000000001}): bytes(Operation::Update(3)),
//!     bytes({name_space}-ch): bytes(StoreHeight(2)),
//!     bytes({name_space}-ch): bytes(StoreHeight(3)),
//!     bytes({name_space}-kw-{hex(4)}-{:00000000000000000003}): bytes(Operation::Update(4)),
//! }
//! ```

#![feature(generic_associated_types)]
#![no_std]

/// For features and alloc.
extern crate alloc;
#[cfg(any(feature = "std", test))]
extern crate std;

mod operation;
pub use operation::{Operation, OperationBytes};

mod cow_lite;
pub use cow_lite::{Cow, CowBytes};

mod error;
pub use error::{Error, Result};

pub mod model;
// pub use model::Value;

pub mod prelude;

mod snapshot;
pub use snapshot::{utils::merkle_key, SnapshotableStorage, Transaction};

pub mod backend;
pub use backend::Store;

mod store;
pub use store::{DoubleKeyMapStore, MapStore, ValueStore, VecStore};

mod utils;

pub mod merkle;
