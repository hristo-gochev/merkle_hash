use anyhow::Result;

use crate::MerkleTree;
use crate::tree::merkle_node::MerkleNode;
use crate::utils::algorithm::{HashingAlgorithm};

/// Utility builder pattern
pub struct MerkleTreeBuilder<const N: usize, A: HashingAlgorithm<N>> {
    /// Absolute root path of the tree
    pub(crate) absolute_root_path: String,
    /// Whether to include names in the hashes of files and directories, default is false
    pub(crate) hash_names: bool,
    /// Which hashing algorithm to use, default is blake3
    pub(crate) algorithm: A,
}

impl<const N: usize, A: HashingAlgorithm<N>> MerkleTreeBuilder<N, A> {
    /// Sets whether to include the names of the files and directories in the hashing process, default is **false**
    pub fn hash_names(mut self, hash_names: bool) -> Self {
        self.hash_names = hash_names;
        self
    }

    /// Sets the hashing algorithm to use, default is **blake3**
    pub fn algorithm<const N2: usize, A2: HashingAlgorithm<N2>>(self, algorithm: A2) -> MerkleTreeBuilder<N2, A2> {
        MerkleTreeBuilder {
            absolute_root_path: self.absolute_root_path,
            hash_names: self.hash_names,
            algorithm,
        }
    }

    /// Builds the hash tree by indexing all of its descendants
    pub fn build(self) -> Result<MerkleTree<N>> {
        let root = MerkleNode::root(&self.absolute_root_path, self.hash_names, self.algorithm)?;
        Ok(MerkleTree { root })
    }
}
