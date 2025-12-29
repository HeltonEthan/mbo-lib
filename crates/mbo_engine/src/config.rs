use std::path::PathBuf;

pub struct Config {
    pub dir: PathBuf,
    pub start: u64,
    pub end: u64,
    pub workers: usize,
    pub qcap: usize,
}

impl Config {
    pub fn new(dir: PathBuf, start: u64, end: u64, workers: usize, qcap: usize) -> Self {
        Self {
            dir,
            start,
            end,
            workers,
            qcap,
        }
    }
}
