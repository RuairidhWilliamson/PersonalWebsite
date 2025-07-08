use std::{
    any::Any,
    collections::hash_map::RandomState,
    num::NonZeroUsize,
    sync::{Arc, Mutex},
    time::Instant,
};

use anyhow::Result;
use panic_lock::MutexExt as _;

use crate::{
    LeafHash, Progress,
    ctx::JobCtx,
    jobs::JobId,
    leaf_set::LeafSet,
    stats::{CompleteStats, LeafStats, Stats},
};

#[derive(Debug, Clone)]
pub struct Cache {
    pub internal: Arc<Mutex<InternalCache>>,
    pub hasher: RandomState,
}

impl Cache {
    pub fn new(cache_size: NonZeroUsize) -> Self {
        Self {
            internal: Arc::new(Mutex::new(InternalCache::new(cache_size))),
            hasher: RandomState::default(),
        }
    }

    /// # Panics
    /// Can panic if the internal lock is poisoned
    pub fn get_generation(&self) -> Option<usize> {
        let guard = self.internal.plock();
        guard.generation
    }

    fn increment_generation(&self) -> usize {
        let generation = &mut self.internal.plock().generation;
        if let Some(generation) = generation {
            *generation += 1;
            *generation
        } else {
            *generation = Some(0);
            0
        }
    }

    pub fn root_ctx<'a, P: Progress>(&'a self, generation: usize, progress: &'a P) -> JobCtx<'a> {
        JobCtx::root(self, generation, progress)
    }

    pub fn root_job<T, F>(&self, id: JobId, f: F) -> Result<RootJobOutput<T>>
    where
        T: Clone + Send + Sync + 'static,
        F: FnOnce(&mut JobCtx<'_>) -> Result<T>,
    {
        self.root_job_with_progress(id, &(), f)
    }

    pub fn root_job_with_progress<'a, T, P, F>(
        &'a self,
        id: JobId,
        progress: &'a P,
        f: F,
    ) -> Result<RootJobOutput<T>>
    where
        T: Clone + Send + Sync + 'static,
        P: Progress,
        F: FnOnce(&mut JobCtx<'_>) -> Result<T>,
    {
        let generation = self.increment_generation();
        let start_time = Instant::now();
        let mut ctx = self.root_ctx(generation, progress);
        let output = ctx.job(id, f)?;
        let hash = ctx.leaf_hash();
        let stats = ctx.stats();
        let completed_stats = CompleteStats {
            leaves: ctx.leaf_count(),
            unique_leaves: ctx.unique_leaf_count(),
            total_time: start_time.elapsed(),
            runtime_execution_time: ctx.runtime_execution_time(),
        };
        Ok(RootJobOutput {
            generation,
            output,
            hash,
            stats,
            completed_stats,
        })
    }
}

#[derive(Debug)]
pub struct RootJobOutput<T> {
    pub generation: usize,
    pub output: T,
    pub hash: u64,
    pub stats: Stats,
    pub completed_stats: CompleteStats,
}

#[derive(Debug)]
pub struct InternalCache {
    cache: lru::LruCache<JobId, JobStore>,
    generation: Option<usize>,
}

impl InternalCache {
    fn new(cache_size: NonZeroUsize) -> Self {
        Self {
            cache: lru::LruCache::new(cache_size),
            generation: None,
        }
    }

    pub fn get(
        &mut self,
        id: &JobId,
        hasher: &RandomState,
        stats: &mut LeafStats,
    ) -> JobCacheOutput {
        let Some(store) = self.cache.get(id) else {
            log::debug!("{id:?} cache miss not present");
            return JobCacheOutput::NotCached;
        };

        if let Some(leaf) = store.calc_is_dirty(hasher, stats) {
            log::debug!("{id:?} cache miss dirty {leaf:?}");
            JobCacheOutput::CacheDirty
        } else {
            log::debug!("{id:?} cache hit");
            JobCacheOutput::Cached(store)
        }
    }

    pub fn put(&mut self, id: JobId, store: JobStore) {
        self.cache.put(id, store);
    }
}

pub enum JobCacheOutput<'a> {
    Cached(&'a JobStore),
    CacheDirty,
    NotCached,
}

#[derive(Debug)]
pub struct JobStore {
    pub leaf_deps: LeafSet,
    pub output: Box<dyn Any + Send + Sync>,
}

impl JobStore {
    pub fn calc_is_dirty(&self, hasher: &RandomState, stats: &mut LeafStats) -> Option<&LeafHash> {
        self.leaf_deps
            .iter()
            .inspect(|_| {
                stats.leaves_checked += 1;
            })
            .find(|l| !matches!(l.is_dirty(hasher), Ok(false)))
    }

    pub fn get_output<T: Clone + 'static>(&self) -> Option<T> {
        self.output.downcast_ref::<T>().cloned()
    }
}
