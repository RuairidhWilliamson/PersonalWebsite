use std::time::Duration;

#[derive(Debug, Default, Clone)]
pub struct Stats {
    pub jobs_cache_hit: usize,
    pub jobs_cache_hit_dirty: usize,
    pub jobs_cache_miss: usize,
    pub leaf_stats: LeafStats,
}

impl Stats {
    pub fn jobs_run(&self) -> usize {
        self.jobs_cache_hit_dirty + self.jobs_cache_miss
    }

    pub fn total_jobs(&self) -> usize {
        self.jobs_cache_hit + self.jobs_run()
    }

    pub fn jobs_cache_percent(&self) -> f32 {
        self.jobs_cache_hit as f32 / self.total_jobs() as f32 * 100.0
    }
}

#[derive(Debug, Default, Clone)]
pub struct LeafStats {
    pub leaves_checked: usize,
}

#[derive(Debug, Clone)]
pub struct CompleteStats {
    pub leaves: usize,
    pub unique_leaves: usize,
    pub total_time: Duration,
    pub runtime_execution_time: Duration,
}
