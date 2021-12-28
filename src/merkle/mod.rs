use digest::{Digest, Output};

// pub mod append_only;一个一个更新
// pub mod empty;
// pub mod sparse_merkle_tree;
// mod utils;
// mod value;
//
// pub use utils::min;

pub trait Merkle: Default + Clone {
    type Digest: Digest;

    fn root(&self) -> Output<Self::Digest>;
}
