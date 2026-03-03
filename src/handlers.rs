use crate::{
    state::AppState,
    template::PAGE,
    util::{file_hash, fmt_size, html_escape, is_valid_session_id, local_ip, resolve_dest, secure_filename},
    CHUNK_SIZE_JS, IO_BUF, MAX_MB, PORT,
};
use axum::{
    body::{Body, Bytes},
    extract::{Multipart, Path, Request, State},
    http::{header, StatusCode},
    response::{Html, IntoResponse, Redirect, Response},
};
use std::{path::PathBuf, sync::Arc};
use tokio::{
    fs::File,
    io::{AsyncWriteExt, BufWriter},
};
use tower::ServiceExt;
use tower_http::services::ServeFile;
use uuid::Uuid;

// Maps a filename's extension to a CSS icon class and a short label.
fn file_icon(name: &str) -> (&'static str, String) {
    let ext = name.rfind('.').map_or("", |i| &name[i + 1..]);
    let ext_low = ext.to_ascii_lowercase();
    let class = match ext_low.as_str() {
        "jpg" | "jpeg" | "png" | "gif" | "webp" | "heic" | "heif"
        | "bmp" | "svg" | "avif" | "ico" | "tiff" | "tif" => "fi-img",

        "mp4" | "mov" | "avi" | "mkv" | "webm" | "m4v"
        | "wmv" | "flv" | "ts" | "3gp" => "fi-vid",

        "mp3" | "aac" | "wav" | "flac" | "m4a" | "ogg"
        | "opus" | "wma" | "aiff" | "alac" => "fi-aud",

        "pdf" | "doc" | "docx" | "xls" | "xlsx" | "ppt" | "pptx"
        | "txt" | "md" | "csv" | "rtf" | "pages" | "numbers" | "key" => "fi-doc",

        "zip" | "rar" | "7z" | "tar" | "gz" | "bz2"
        | "xz" | "zst" | "lz4" | "tgz" | "tbz2" => "fi-arc",

        _ => "fi-gen",
    };
    let label = if ext.is_empty() {
        "FILE".to_string()
    } else {
        ext.chars().take(4).collect::<String>().to_ascii_uppercase()
    };
    (class, label)
}

pub async fn index(State(state): State<Arc<AppState>>) -> Response {
    let mut files: Vec<(String, String)> = Vec::new();
    if let Ok(mut rd) = tokio::fs::read_dir(&state.upload_dir).await {
        while let Ok(Some(entry)) = rd.next_entry().await {
            if entry.path().is_file() {
                let name = entry.file_name().to_string_lossy().to_string();
                let size = entry
                    .metadata()
                    .await
                    .map(|m| fmt_size(m.len()))
                    .unwrap_or_else(|_| "?".to_string());
                files.push((name, size));
            }
        }
    }
    files.sort_by(|a, b| a.0.cmp(&b.0));

    let count = files.len();
    let file_count_label = match count {
        0 => "0 files".to_string(),
        1 => "1 file".to_string(),
        n => format!("{n} files"),
    };
    let files_html = if files.is_empty() {
        r#"<div class="empty-state">
  <svg width="64" height="56" viewBox="0 0 64 56" fill="none">
    <path d="M2 20C2 13.373 7.373 8 14 8H26L30 4H50C56.627 4 62 9.373 62 16V44C62 50.627 56.627 56 50 56H14C7.373 56 2 50.627 2 44V20Z"
          stroke="white" stroke-width="2.5" stroke-linejoin="round"/>
  </svg>
  <div class="empty-title">No Files Yet</div>
  <div class="empty-sub">Drop files into the uploads folder on your PC to share them here</div>
</div>"#
            .to_string()
    } else {
        files
            .iter()
            .map(|(name, size)| {
                let e = html_escape(name);
                let (icon_class, icon_label) = file_icon(name);
                format!(
                    r#"<div class="file-row">
  <div class="fi {icon_class}">{icon_label}</div>
  <div class="finfo">
    <span class="fname" title="{e}">{e}</span>
    <span class="fsize">{size}</span>
  </div>
  <a class="btn-dl" href="/download/{e}" download="{e}">
    <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
      <path d="M8 3v8M4.5 8.5l3.5 3 3.5-3" stroke="white" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"/>
    </svg>
  </a>
</div>"#
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    };

    let ip = local_ip();
    let html = PAGE
        .replace("TMPL_IP", &ip)
        .replace("TMPL_PORT", &PORT.to_string())
        .replace("TMPL_MAX_MB", &MAX_MB.to_string())
        .replace("TMPL_CHUNK_SIZE", &CHUNK_SIZE_JS.to_string())
        .replace("TMPL_FILE_COUNT", &file_count_label)
        .replace("TMPL_FILES", &files_html);

    let mut response = Html(html).into_response();
    response
        .headers_mut()
        .insert(header::CACHE_CONTROL, "no-cache".parse().unwrap());
    response
}

// old-style multipart upload, kept around for desktop drag-and-drop
pub async fn upload(
    State(state): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    loop {
        let mut field = match multipart.next_field().await {
            Ok(Some(f)) => f,
            _ => break,
        };
        if field.name() != Some("files") {
            continue;
        }
        let filename = match field.file_name().map(|s| s.to_string()) {
            Some(n) if !n.is_empty() => secure_filename(&n),
            _ => continue,
        };
        let dest = resolve_dest(&state.upload_dir, &filename);
        let file = match File::create(&dest).await {
            Ok(f) => f,
            Err(_) => continue,
        };
        let mut writer = BufWriter::with_capacity(IO_BUF, file);
        let mut ok = true;
        loop {
            match field.chunk().await {
                Ok(Some(chunk)) => {
                    if writer.write_all(&chunk).await.is_err() {
                        ok = false;
                        break;
                    }
                }
                Ok(None) => break,
                Err(_) => {
                    ok = false;
                    break;
                }
            }
        }
        if ok {
            let _ = writer.flush().await;
        } else {
            let _ = tokio::fs::remove_file(&dest).await;
        }
    }
    Redirect::to("/")
}

// creates a temp dir for the session and returns its UUID
pub async fn upload_init(
    State(state): State<Arc<AppState>>,
    body: Bytes,
) -> impl IntoResponse {
    let raw = std::str::from_utf8(&body).unwrap_or("upload");
    let filename = secure_filename(raw.trim());
    let id = Uuid::new_v4().to_string();
    let temp_dir = state.upload_dir.join(&id);
    if tokio::fs::create_dir_all(&temp_dir).await.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create session").into_response();
    }
    state.sessions.insert(id.clone(), filename);
    (StatusCode::OK, id).into_response()
}

// saves one chunk to uploads/{id}/{seq}.part
pub async fn upload_chunk(
    State(state): State<Arc<AppState>>,
    Path((id, seq)): Path<(String, u64)>,
    body: Bytes,
) -> impl IntoResponse {
    if !is_valid_session_id(&id) || !state.sessions.contains_key(&id) {
        return (StatusCode::BAD_REQUEST, "Invalid session").into_response();
    }
    let part = state.upload_dir.join(&id).join(format!("{seq}.part"));
    match tokio::fs::write(&part, &body).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Write failed").into_response(),
    }
}

// joins all the parts in order, hashes the result, cleans up the temp dir
pub async fn upload_complete(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    if !is_valid_session_id(&id) {
        return (StatusCode::BAD_REQUEST, "Invalid session").into_response();
    }
    let filename = match state.sessions.remove(&id) {
        Some((_, f)) => f,
        None => return (StatusCode::NOT_FOUND, "Session not found").into_response(),
    };
    let temp_dir = state.upload_dir.join(&id);

    let mut parts: Vec<(u64, PathBuf)> = Vec::new();
    match tokio::fs::read_dir(&temp_dir).await {
        Ok(mut rd) => {
            while let Ok(Some(entry)) = rd.next_entry().await {
                let name = entry.file_name().to_string_lossy().to_string();
                if let Some(seq_str) = name.strip_suffix(".part") {
                    if let Ok(seq) = seq_str.parse::<u64>() {
                        parts.push((seq, entry.path()));
                    }
                }
            }
        }
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "Cannot read session dir").into_response()
        }
    }
    parts.sort_by_key(|(seq, _)| *seq);

    let dest = resolve_dest(&state.upload_dir, &filename);
    let file = match File::create(&dest).await {
        Ok(f) => f,
        Err(_) => {
            let _ = tokio::fs::remove_dir_all(&temp_dir).await;
            return (StatusCode::INTERNAL_SERVER_ERROR, "Cannot create file").into_response();
        }
    };
    let mut writer = BufWriter::with_capacity(IO_BUF, file);
    let mut hasher = blake3::Hasher::new();

    for (_, part_path) in &parts {
        match tokio::fs::read(part_path).await {
            Ok(data) => {
                hasher.update(&data);
                if writer.write_all(&data).await.is_err() {
                    let _ = tokio::fs::remove_dir_all(&temp_dir).await;
                    return (StatusCode::INTERNAL_SERVER_ERROR, "Write failed").into_response();
                }
            }
            Err(_) => {
                let _ = tokio::fs::remove_dir_all(&temp_dir).await;
                return (StatusCode::INTERNAL_SERVER_ERROR, "Read part failed").into_response();
            }
        }
    }

    if writer.flush().await.is_err() {
        let _ = tokio::fs::remove_dir_all(&temp_dir).await;
        return (StatusCode::INTERNAL_SERVER_ERROR, "Flush failed").into_response();
    }

    let hash = hasher.finalize().to_hex().to_string();

    if let Some(fname) = dest.file_name().and_then(|n| n.to_str()) {
        state.hash_cache.insert(fname.to_string(), hash.clone());
    }
    let _ = tokio::fs::remove_dir_all(&temp_dir).await;

    let mut response = (StatusCode::OK, "").into_response();
    response
        .headers_mut()
        .insert("x-file-hash", hash.parse().unwrap());
    response
}

pub async fn download(
    State(state): State<Arc<AppState>>,
    Path(filename): Path<String>,
    req: Request,
) -> Response {
    // block path traversal before touching the filesystem
    if filename.contains('/') || filename.contains('\\') || filename.contains("..") {
        return (StatusCode::BAD_REQUEST, "Invalid filename").into_response();
    }

    let path = state.upload_dir.join(&filename);
    let canonical = match path.canonicalize() {
        Ok(p) => p,
        Err(_) => return (StatusCode::NOT_FOUND, "Not found").into_response(),
    };
    let upload_canonical = match state.upload_dir.canonicalize() {
        Ok(p) => p,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Server error").into_response(),
    };
    if !canonical.starts_with(&upload_canonical) {
        return (StatusCode::FORBIDDEN, "Forbidden").into_response();
    }
    if !canonical.is_file() {
        return (StatusCode::NOT_FOUND, "Not found").into_response();
    }

    let hash = file_hash(&state, &filename, &canonical).await;

    match ServeFile::new(&canonical).oneshot(req).await {
        Ok(res) => {
            let (mut parts, body) = res.into_parts();
            let disposition = format!(r#"attachment; filename="{}""#, html_escape(&filename));
            parts
                .headers
                .insert(header::CONTENT_DISPOSITION, disposition.parse().unwrap());
            parts.headers.insert(
                header::CACHE_CONTROL,
                "public, max-age=3600".parse().unwrap(),
            );
            if !hash.is_empty() {
                parts.headers.insert(
                    header::ETAG,
                    format!(r#""{hash}""#).parse().unwrap(),
                );
            }
            Response::from_parts(parts, Body::new(body))
        }
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Read error").into_response(),
    }
}
