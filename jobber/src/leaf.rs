use std::{
    collections::hash_map::RandomState,
    hash::{BuildHasher, Hash},
    path::PathBuf,
};

use anyhow::Result;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum Leaf {
    File(PathBuf),
    #[cfg(feature = "glob")]
    Glob(String),
}

impl Leaf {
    pub fn hash(&self, hasher: &RandomState) -> Result<u64> {
        match self {
            Leaf::File(path) => {
                let contents = std::fs::read(path)?;
                Ok(hasher.hash_one(contents))
            }
            #[cfg(feature = "glob")]
            Leaf::Glob(pattern) => {
                use std::hash::Hasher;
                let mut hasher = hasher.build_hasher();
                for entry in glob::glob(&pattern)? {
                    let p = entry?;
                    let contents = std::fs::read(&p)?;
                    p.hash(&mut hasher);
                    contents.hash(&mut hasher);
                }
                Ok(hasher.finish())
            }
        }
    }

    pub fn into_hash(self, hasher: &RandomState) -> Result<LeafHash> {
        Ok(LeafHash {
            hash: self.hash(hasher)?,
            leaf: self,
        })
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct LeafHash {
    pub leaf: Leaf,
    pub hash: u64,
}

impl LeafHash {
    pub fn is_dirty(&self, hasher: &RandomState) -> Result<bool> {
        Ok(self.leaf.hash(hasher)? != self.hash)
    }
}
