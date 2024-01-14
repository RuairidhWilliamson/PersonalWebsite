use std::{
    any::Any,
    collections::hash_map::RandomState,
    hash::BuildHasher,
    num::NonZeroUsize,
    sync::{Arc, Mutex},
};

use anyhow::Result;

use crate::{ctx::JobCtx, jobs::JobId, leaf::LeafHash, stats::Stats};

#[derive(Debug, Clone)]
pub struct Cache {
    pub(crate) internal: Arc<Mutex<InternalCache>>,
    pub(crate) hasher: RandomState,
}

impl Cache {
    pub fn new(cache_size: NonZeroUsize) -> Self {
        Self {
            internal: Arc::new(Mutex::new(InternalCache::new(cache_size))),
            hasher: RandomState::default(),
        }
    }

    pub fn get_generation(&self) -> usize {
        let guard = self.internal.lock().unwrap();
        guard.generation
    }

    pub fn root_ctx(&self) -> JobCtx {
        JobCtx::new(self)
    }

    pub fn root_job<T, F>(&self, id: JobId, f: F) -> Result<RootJobOutput<T>>
    where
        T: Clone + Send + Sync + 'static,
        F: FnOnce(&mut JobCtx<'_>) -> Result<T>,
    {
        let mut ctx = self.root_ctx();
        let output = ctx.job(id, f)?;
        let hash = self.hasher.hash_one(&ctx.deps_acc);
        let stats = ctx.stats.lock().unwrap().clone();
        Ok(RootJobOutput {
            output,
            hash,
            stats,
        })
    }
}

pub struct RootJobOutput<T> {
    pub output: T,
    pub hash: u64,
    pub stats: Stats,
}

#[derive(Debug)]
pub(crate) struct InternalCache {
    pub(crate) cache: lru::LruCache<JobId, JobStore>,
    pub(crate) generation: usize,
}

impl InternalCache {
    fn new(cache_size: NonZeroUsize) -> Self {
        Self {
            cache: lru::LruCache::new(cache_size),
            generation: 0,
        }
    }

    pub(crate) fn get_cached_job(&mut self, id: &JobId, hasher: &RandomState) -> Option<&JobStore> {
        let store = self.cache.get(id)?;
        if store.calc_is_dirty(hasher) {
            return None;
        }
        Some(store)
    }
}

#[derive(Debug)]
pub(crate) struct JobStore {
    pub(crate) leaf_deps: Vec<LeafHash>,
    pub(crate) output: Box<dyn Any + Send + Sync>,
}

impl JobStore {
    fn calc_is_dirty(&self, hasher: &RandomState) -> bool {
        self.leaf_deps
            .iter()
            .any(|l| !matches!(l.is_dirty(hasher), Ok(false)))
    }

    pub(crate) fn get_output<T: Clone + 'static>(&self) -> Option<T> {
        self.output.downcast_ref::<T>().cloned()
    }
}
