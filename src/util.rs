use crate::state::AppState;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

pub fn local_ip() -> String {
    use std::net::UdpSocket;
    UdpSocket::bind("0.0.0.0:0")
        .and_then(|s| {
            s.connect("8.8.8.8:80")?;
            s.local_addr()
        })
        .map(|a| a.ip().to_string())
        .unwrap_or_else(|_| "127.0.0.1".to_string())
}

pub fn fmt_size(bytes: u64) -> String {
    const UNITS: &[(&str, u64)] = &[
        ("TB", 1 << 40),
        ("GB", 1 << 30),
        ("MB", 1 << 20),
        ("KB", 1 << 10),
    ];
    for &(unit, threshold) in UNITS {
        if bytes >= threshold {
            return format!("{:.1} {}", bytes as f64 / threshold as f64, unit);
        }
    }
    format!("{} B", bytes)
}

pub fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

// keep just the filename part and swap out any sketchy characters
pub fn secure_filename(name: &str) -> String {
    let base = Path::new(name)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("upload");
    let result: String = base
        .chars()
        .map(|c| if c.is_alphanumeric() || "._- ".contains(c) { c } else { '_' })
        .collect();
    if result.trim().is_empty() {
        "upload".to_string()
    } else {
        result
    }
}

// appends _1, _2, ... until it finds a free slot
pub fn resolve_dest(dir: &Path, filename: &str) -> PathBuf {
    let dest = dir.join(filename);
    if !dest.exists() {
        return dest;
    }
    let p = Path::new(filename);
    let stem = p.file_stem().and_then(|s| s.to_str()).unwrap_or(filename);
    let ext = p
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| format!(".{e}"))
        .unwrap_or_default();
    let mut n = 1u32;
    loop {
        let candidate = dir.join(format!("{stem}_{n}{ext}"));
        if !candidate.exists() {
            return candidate;
        }
        n += 1;
    }
}

// only accept bare UUIDs to block path traversal in session ids
pub fn is_valid_session_id(id: &str) -> bool {
    id.len() == 36 && id.chars().all(|c| c.is_ascii_hexdigit() || c == '-')
}

// returns the cached hash or computes it fresh and caches it
pub async fn file_hash(state: &Arc<AppState>, filename: &str, path: &Path) -> String {
    if let Some(h) = state.hash_cache.get(filename) {
        return h.clone();
    }
    let path = path.to_path_buf();
    let hash = tokio::task::spawn_blocking(move || -> Option<String> {
        let data = std::fs::read(&path).ok()?;
        Some(blake3::hash(&data).to_hex().to_string())
    })
    .await
    .ok()
    .flatten()
    .unwrap_or_default();

    if !hash.is_empty() {
        state.hash_cache.insert(filename.to_string(), hash.clone());
    }
    hash
}
