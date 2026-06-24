use axum::{Router, routing::post, extract::State, http::StatusCode};
use std::net::TcpListener;
use std::sync::Arc;
use tauri::{AppHandle, Manager};
use tokio::sync::Mutex;

pub struct FenServerPort(pub u16);

pub fn start_fen_server(app: AppHandle) -> u16 {
    let listener = TcpListener::bind("127.0.0.1:39521")
        .unwrap_or_else(|_| TcpListener::bind("127.0.0.1:0").unwrap());
    let port = listener.local_addr().unwrap().port();

    tokio::spawn(async move {
        let router = Router::new()
            .route("/fen", post(set_fen))
            .with_state(Arc::new(Mutex::new(app)));

        axum::Server::from_tcp(listener)
            .unwrap()
            .serve(router.into_make_service())
            .await
            .unwrap();
    });

    log::info!("FEN server started on port {port}");
    port
}

async fn set_fen(
    State(app): State<Arc<Mutex<AppHandle>>>,
    body: String,
) -> StatusCode {
    let fen = body.trim().to_string();
    if fen.is_empty() {
        return StatusCode::BAD_REQUEST;
    }
    let app = app.lock().await;
    app.emit_all("set-fen", fen).ok();
    StatusCode::OK
}
