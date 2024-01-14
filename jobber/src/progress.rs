use crate::Stats;

pub trait Progress {
    fn report(&self, report: ProgressReport);
}

#[derive(Debug)]
pub struct ProgressReport<'a> {
    pub stats: &'a Stats,
}

impl Progress for () {
    fn report(&self, _report: ProgressReport) {
        // Do nothing
    }
}

pub struct DebugPrintProgress;

impl Progress for DebugPrintProgress {
    fn report(&self, report: ProgressReport) {
        println!("{report:?}");
    }
}
