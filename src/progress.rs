use jobber::{Progress, ProgressReport, RootJobOutput};

pub trait SiteBuildProgress: Progress + Send {
    fn report_built(&self, output: &RootJobOutput<()>);
}

pub struct DefaultSiteBuildProgress;

#[expect(clippy::print_stdout)]
impl SiteBuildProgress for DefaultSiteBuildProgress {
    fn report_built(
        &self,
        RootJobOutput {
            generation,
            hash,
            stats,
            completed_stats,
            ..
        }: &RootJobOutput<()>,
    ) {
        let elapsed = completed_stats.total_time;
        print!("{}c", 27 as char);
        println!();
        println!(" 🚀 Built {hash:x} ");
        println!(" Generation = {generation}");
        println!(
            " Jobs = {} / {} = {:.1}%",
            stats.jobs_run(),
            stats.total_jobs(),
            stats.jobs_cache_percent()
        );
        println!(" ⏱️  {elapsed:.1?}");
    }
}

#[expect(clippy::print_stdout)]
impl Progress for DefaultSiteBuildProgress {
    fn report(
        &self,
        ProgressReport {
            generation, stats, ..
        }: ProgressReport,
    ) {
        print!("{}c", 27 as char);
        println!();
        println!(" 🔨 Building...");
        println!(" Generation = {generation}");
        println!(
            " Jobs = {} / {} = {:.1}%",
            stats.jobs_run(),
            stats.total_jobs(),
            stats.jobs_cache_percent()
        );
    }
}

pub struct NoSiteBuildProgress;

impl SiteBuildProgress for NoSiteBuildProgress {
    fn report_built(&self, _output: &RootJobOutput<()>) {}
}

impl Progress for NoSiteBuildProgress {
    fn report(&self, _report: ProgressReport) {}
}
