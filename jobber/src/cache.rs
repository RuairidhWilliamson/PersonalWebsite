use std::{
    any::Any,
    collections::hash_map::RandomState,
    hash::{BuildHasher, Hash},
    num::NonZeroUsize,
    path::Path,
    sync::{Arc, Mutex},
};

use anyhow::Result;

use crate::{
    jobs::JobId,
    leaf::{Leaf, LeafHash},
};

pub struct JobCtx<'a> {
    cache: &'a Cache,
    deps_acc: Vec<LeafHash>,
}

impl Hash for JobCtx<'_> {
    fn hash<H: std::hash::Hasher>(&self, _state: &mut H) {}
}

impl JobCtx<'_> {
    pub fn depends_file<P>(&mut self, path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        self.depends(Leaf::File(path.as_ref().to_path_buf()))
    }

    pub fn depends(&mut self, leaf: Leaf) -> Result<()> {
        let leaf_hash = leaf.into_hash(&self.cache.hasher)?;
        self.deps_acc.push(leaf_hash);
        Ok(())
    }

    fn child_ctx(&self) -> Self {
        Self {
            cache: self.cache,
            deps_acc: Default::default(),
        }
    }

    pub fn job<T, F>(&mut self, id: JobId, job: F) -> Result<T>
    where
        T: Clone + Send + Sync + 'static,
        F: FnOnce(&mut Self) -> Result<T>,
    {
        {
            let mut guard = self.cache.internal.lock().unwrap();
            if let Some(store) = guard.get_cached_job(&id, &self.cache.hasher) {
                self.deps_acc.extend(store.leaf_deps.clone());
                return Ok(store.get_output().expect("output type mismatch"));
            }
        }
        let mut ctx = self.child_ctx();

        let result = job(&mut ctx);

        let leaf_deps = ctx.deps_acc;
        if let Ok(result) = result.as_ref() {
            let mut guard = self.cache.internal.lock().unwrap();
            let job_store = JobStore {
                leaf_deps: leaf_deps.clone(),
                output: Box::new(result.clone()),
            };
            guard.cache.put(id, job_store);
        }
        self.deps_acc.extend(leaf_deps.clone());
        result
    }
}

#[derive(Debug, Clone)]
pub struct Cache {
    internal: Arc<Mutex<InternalCache>>,
    hasher: RandomState,
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

    pub fn root_ctx(&self) -> JobCtx<'_> {
        JobCtx {
            cache: self,
            deps_acc: Default::default(),
        }
    }

    pub fn root_job<T, F>(&self, id: JobId, f: F) -> Result<(T, u64)>
    where
        T: Clone + Send + Sync + 'static,
        F: FnOnce(&mut JobCtx<'_>) -> Result<T>,
    {
        let mut ctx = self.root_ctx();
        let output = ctx.job(id, f)?;
        Ok((output, self.hasher.hash_one(&ctx.deps_acc)))
    }
}

#[derive(Debug)]
struct InternalCache {
    cache: lru::LruCache<JobId, JobStore>,
    generation: usize,
}

impl InternalCache {
    fn new(cache_size: NonZeroUsize) -> Self {
        Self {
            cache: lru::LruCache::new(cache_size),
            generation: 0,
        }
    }

    fn get_cached_job(&mut self, id: &JobId, hasher: &RandomState) -> Option<&JobStore> {
        let store = self.cache.get(id)?;
        if store.calc_is_dirty(hasher) {
            return None;
        }
        Some(store)
    }
}

#[derive(Debug)]
struct JobStore {
    leaf_deps: Vec<LeafHash>,
    output: Box<dyn Any + Send + Sync>,
}

impl JobStore {
    fn calc_is_dirty(&self, hasher: &RandomState) -> bool {
        self.leaf_deps
            .iter()
            .any(|l| !matches!(l.is_dirty(hasher), Ok(false)))
    }

    fn get_output<T: Clone + 'static>(&self) -> Option<T> {
        self.output.downcast_ref::<T>().cloned()
    }
}
