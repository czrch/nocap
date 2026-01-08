use notify::RecommendedWatcher;
use std::sync::Mutex;

pub struct FsWatcher {
    pub watcher: Mutex<Option<RecommendedWatcher>>,
}

impl Default for FsWatcher {
    fn default() -> Self {
        Self {
            watcher: Mutex::new(None),
        }
    }
}
