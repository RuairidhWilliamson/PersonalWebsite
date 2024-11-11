use std::{
    collections::HashSet,
    hash::{BuildHasher as _, Hash},
    path::Path,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use anyhow::Result;

use crate::{
    cache::{JobCacheOutput, JobStore},
    leaf_set::LeafSet,
    Cache, JobId, Leaf, Progress, ProgressReport, Stats,
};

pub struct JobCtx<'a> {
    generation: usize,
    cache: &'a Cache,
    progress: &'a dyn Progress,
    stats: Arc<Mutex<Stats>>,
    leaves: LeafSet,
    runtime_execution_time: Duration,
}

impl Hash for JobCtx<'_> {
    fn hash<H: std::hash::Hasher>(&self, _state: &mut H) {}
}

impl<'a> JobCtx<'a> {
    pub fn root<P: Progress>(cache: &'a Cache, generation: usize, progress: &'a P) -> Self {
        Self {
            generation,
            cache,
            progress,
            stats: Arc::default(),
            leaves: Default::default(),
            runtime_execution_time: Duration::default(),
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

    #[cfg(feature = "glob")]
    pub fn depends_glob(&mut self, glob: &str) -> Result<()> {
        self.depends(Leaf::Glob(glob.to_owned()))
    }

    pub fn depends(&mut self, leaf: Leaf) -> Result<()> {
        let leaf_hash = leaf.into_hash(&self.cache.hasher)?;
        self.leaves.push(leaf_hash);
        Ok(())
    }

    fn child_ctx(&self) -> Self {
        Self {
            generation: self.generation,
            cache: self.cache,
            progress: self.progress,
            stats: Arc::clone(&self.stats),
            leaves: Default::default(),
            runtime_execution_time: Duration::default(),
        }
    }

    /// # Panics
    /// Can panic if the internal lock or the stats lock is poisoned.
    /// Can panic if the output type does not match the cached type
    pub fn job<T, F>(&mut self, id: JobId, job: F) -> Result<T>
    where
        T: Clone + Send + Sync + 'static,
        F: FnOnce(&mut Self) -> Result<T>,
    {
        let runtime_start_time = Instant::now();
        {
            let mut cache_guard = self.cache.internal.lock().unwrap();
            let mut stats_guard = self.stats.lock().unwrap();
            match cache_guard.get(&id, &self.cache.hasher, &mut stats_guard.leaf_stats) {
                JobCacheOutput::Cached(store) => {
                    stats_guard.jobs_cache_hit += 1;
                    self.leaves.extend(store.leaf_deps.clone());
                    self.report_progress(&stats_guard);
                    let output = store.get_output().expect("output type mismatch");
                    self.runtime_execution_time += runtime_start_time.elapsed();
                    return Ok(output);
                }
                JobCacheOutput::CacheDirty => {
                    stats_guard.jobs_cache_hit_dirty += 1;
                }
                JobCacheOutput::NotCached => {
                    stats_guard.jobs_cache_miss += 1;
                }
            }
            self.report_progress(&stats_guard);
        }
        let mut ctx = self.child_ctx();
        // Exclude the job itself from runtime_execution_time
        self.runtime_execution_time += runtime_start_time.elapsed();

        let result = job(&mut ctx);

        // Include the runtime execution time of child jobs
        self.runtime_execution_time += ctx.runtime_execution_time;
        // Restart the timer after job complete
        let jobber_start_time = Instant::now();

        let leaf_deps = ctx.leaves;
        if let Ok(result) = result.as_ref() {
            let mut guard = self.cache.internal.lock().unwrap();
            let job_store = JobStore {
                leaf_deps: leaf_deps.clone(),
                output: Box::new(result.clone()),
            };
            guard.put(id, job_store);
        }
        self.leaves.extend(leaf_deps);
        self.runtime_execution_time += jobber_start_time.elapsed();
        result
    }

    fn report_progress(&self, stats: &Stats) {
        self.progress.report(ProgressReport {
            generation: self.generation,
            stats,
        });
    }

    /// # Panics
    /// Can panic if the stats lock is poisoned
    pub fn stats(&self) -> Stats {
        self.stats.lock().unwrap().clone()
    }

    pub fn leaf_hash(&self) -> u64 {
        use std::hash::Hasher as _;
        let mut h = self.cache.hasher.build_hasher();
        for leaf in &self.leaves {
            h.write_u64(leaf.hash);
        }
        h.finish()
    }

    pub fn leaf_count(&self) -> usize {
        self.leaves.len()
    }

    /// Maybe a lot slower compared to [`Self::leaf_count`]
    pub fn unique_leaf_count(&self) -> usize {
        let h: HashSet<_> = self.leaves.iter().collect();
        h.len()
    }

    pub fn runtime_execution_time(&self) -> Duration {
        self.runtime_execution_time
    }
}
