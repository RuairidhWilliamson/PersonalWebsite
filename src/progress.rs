use jobber::Progress;

pub struct SiteBuildProgress;

impl Progress for SiteBuildProgress {
    fn report(&self, report: jobber::ProgressReport) {
        println!("{report:?}");
    }
}
