mod handlers;
mod state;
mod template;
mod util;

use axum::{
    extract::DefaultBodyLimit,
    routing::{get, post},
    Router,
};
use dashmap::DashMap;
use state::AppState;
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tokio::net::TcpSocket;

pub const PORT: u16 = 5000;
pub const MAX_MB: usize = 512;
pub const CHUNK_SIZE_JS: usize = 2 * 1024 * 1024; // 2 MB
pub const IO_BUF: usize = 256 * 1024;             // 256 KB

#[tokio::main]
async fn main() {
    let upload_dir = std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join("uploads");
    tokio::fs::create_dir_all(&upload_dir)
        .await
        .expect("cannot create uploads dir");

    let state = Arc::new(AppState {
        upload_dir: upload_dir.clone(),
        sessions: DashMap::new(),
        hash_cache: DashMap::new(),
    });

    let app = Router::new()
        .route("/", get(handlers::index))
        .route("/upload", post(handlers::upload))
        .route("/upload/init", post(handlers::upload_init))
        .route("/upload/chunk/:id/:seq", post(handlers::upload_chunk))
        .route("/upload/complete/:id", post(handlers::upload_complete))
        .route("/download/:filename", get(handlers::download))
        .layer(DefaultBodyLimit::max(MAX_MB * 1024 * 1024))
        .with_state(state);

    let ip = util::local_ip();
    println!();
    println!("================================================");
    println!(" --File upload and download-- ");
    println!();
    println!("  Local:    http://127.0.0.1:{PORT}");
    println!("  Network:  http://{ip}:{PORT}  <- open in a new tab");
    println!("  Uploads:  {}", upload_dir.display());
    println!();
    println!("  Press Ctrl+C to stop.");
    println!("================================================");
    println!();

    // bump socket buffers for faster transfers
    let addr = SocketAddr::from(([0, 0, 0, 0], PORT));
    let socket = TcpSocket::new_v4().expect("failed to create socket");
    socket.set_send_buffer_size(1024 * 1024).expect("SO_SNDBUF");
    socket.set_recv_buffer_size(1024 * 1024).expect("SO_RCVBUF");
    socket.bind(addr).expect("failed to bind");
    let listener = socket.listen(1024).expect("failed to listen");

    axum::serve(listener, app).await.expect("server error");
}
