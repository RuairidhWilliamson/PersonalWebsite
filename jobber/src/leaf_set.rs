use std::collections::{hash_set::Iter, HashSet};

use crate::LeafHash;

#[derive(Debug, Default, Clone)]
pub struct LeafSet {
    leaves: HashSet<LeafHash>,
}

impl LeafSet {
    pub fn push(&mut self, leaf_hash: LeafHash) {
        self.leaves.insert(leaf_hash);
    }

    pub fn extend(&mut self, other: LeafSet) {
        self.leaves.extend(other.leaves)
    }

    pub fn iter(&self) -> impl Iterator<Item = &LeafHash> {
        self.leaves.iter()
    }

    pub fn len(&self) -> usize {
        self.leaves.len()
    }
}

impl<'a> IntoIterator for &'a LeafSet {
    type Item = &'a LeafHash;

    type IntoIter = Iter<'a, LeafHash>;

    fn into_iter(self) -> Self::IntoIter {
        self.leaves.iter()
    }
}
