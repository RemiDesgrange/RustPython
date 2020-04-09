use crossbeam_utils::atomic::AtomicCell;
use std::fs::File;
use std::io::{Read, Result};
use std::path::Path;

/// Read a file at `path` into a String
pub fn read_file(path: &Path) -> Result<String> {
    info!("Loading file {:?}", path);
    let mut f = File::open(&path)?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;

    Ok(buffer)
}

pub(crate) fn atomic_iter_advance(
    pos: &AtomicCell<usize>,
    done: impl Fn(usize) -> bool,
    next: impl Fn(usize) -> usize,
) -> Option<usize> {
    let mut prev = pos.load();
    loop {
        if done(prev) {
            return None;
        }
        match pos.compare_exchange(prev, next(prev)) {
            Ok(new) => return Some(new),
            Err(p) => prev = p,
        }
    }
}
