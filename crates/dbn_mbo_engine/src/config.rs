use chrono::NaiveDate;
use std::path::PathBuf;

use crate::helper;

/// Config gives the engine specific details on where and what to run.
/// 
/// This struct holds information about the directory the engine will look at, 
/// the latency to consider for slippage, 
/// and what files it should consider in regards to start and end.
#[derive(Debug)]
pub struct Config {
    pub dir: PathBuf,
    pub start: NaiveDate,
    pub end: NaiveDate,
    pub latency: u16,
}

impl Config {
    /// Creates a new config with given dir, start, end, and latency (ms)
    pub fn new(dir: PathBuf, start: NaiveDate, end: NaiveDate, latency: u16) -> Self {
        Self { dir, start, end, latency }
    }

    /// Returns a reference to the dir.
    pub fn dir(&self) -> &PathBuf {
        &self.dir
    }

    /// Returns a reference to start.
    pub fn start(&self) -> &NaiveDate {
        &self.start
    }

    /// Returns a reference to end.
    pub fn end(&self) -> &NaiveDate {
        &self.end
    }

    /// Returns a reference to latency.
    pub fn latency(&self) -> &u16 {
        &self.latency
    }

    /// Returns a start as unix_nanos.
    pub fn start_unix(&self) -> anyhow::Result<u64> {
        helper::to_unix(self.start())
    }

    /// Returns a end as unix_nanos.
    pub fn end_unix(&self) -> anyhow::Result<u64> {
        helper::to_unix(self.end())
    }
}
