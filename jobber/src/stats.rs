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
