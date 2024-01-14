use std::{
    hash::Hash,
    path::Path,
    sync::{Arc, Mutex},
};

use anyhow::Result;

use crate::{cache::JobStore, stats::Stats, Cache, JobId, Leaf, LeafHash};

pub struct JobCtx<'a> {
    pub(crate) cache: &'a Cache,
    pub(crate) stats: Arc<Mutex<Stats>>,
    pub(crate) deps_acc: Vec<LeafHash>,
}

impl Hash for JobCtx<'_> {
    fn hash<H: std::hash::Hasher>(&self, _state: &mut H) {}
}

impl<'a> JobCtx<'a> {
    pub(crate) fn new(cache: &'a Cache) -> Self {
        Self {
            cache,
            stats: Default::default(),
            deps_acc: Default::default(),
        }
    }
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
            stats: self.stats.clone(),
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
                {
                    let mut stats_guard = self.stats.lock().unwrap();
                    stats_guard.jobs_cached += 1;
                }
                self.deps_acc.extend(store.leaf_deps.clone());
                return Ok(store.get_output().expect("output type mismatch"));
            }
        }
        {
            let mut stats_guard = self.stats.lock().unwrap();
            stats_guard.jobs_run += 1;
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
