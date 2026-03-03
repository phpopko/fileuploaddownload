use dashmap::DashMap;
use std::path::PathBuf;

pub struct AppState {
    pub upload_dir: PathBuf,
    // in-progress chunked uploads: session id -> filename
    pub sessions: DashMap<String, String>,
    // blake3 hex per filename so downloads don't rehash every time
    pub hash_cache: DashMap<String, String>,
}
