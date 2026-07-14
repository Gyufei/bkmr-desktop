use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::{Html, Json},
    routing::{delete, get, post, put},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::OnceLock;
use tauri::Emitter;

static APP_HANDLE: OnceLock<tauri::AppHandle> = OnceLock::new();
static SERVER_URL: OnceLock<String> = OnceLock::new();
static SERVER_RUNNING: AtomicBool = AtomicBool::new(false);

#[derive(Debug, Clone, Serialize)]
pub struct ServerStatus {
    pub running: bool,
    pub url: String,
}

pub fn status() -> ServerStatus {
    ServerStatus {
        running: SERVER_RUNNING.load(Ordering::SeqCst),
        url: SERVER_URL.get().cloned().unwrap_or_else(|| "http://127.0.0.1:8733".to_string()),
    }
}

pub async fn start_server(app_handle: tauri::AppHandle, shutdown_rx: tokio::sync::oneshot::Receiver<()>) {
    APP_HANDLE.set(app_handle).ok();

    let app = Router::new()
        .route("/api/bookmarks", post(add_bookmark_handler))
        .route("/api/bookmarks/check", get(check_bookmark_handler))
        .route("/api/bookmarks/{id}", get(get_bookmark_handler))
        .route("/api/bookmarks/{id}", put(update_bookmark_handler))
        .route("/api/bookmarks/{id}", delete(delete_bookmark_handler))
        .route("/api/tags", get(get_tags_handler))
        .route("/api/docs", get(docs_handler));

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 8733));
    let listener = match tokio::net::TcpListener::bind(addr).await {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Failed to bind HTTP server on 127.0.0.1:8733: {e}");
            return;
        }
    };

    SERVER_URL.set("http://127.0.0.1:8733".to_string()).ok();
    SERVER_RUNNING.store(true, Ordering::SeqCst);

    axum::serve(listener, app)
        .with_graceful_shutdown(async { shutdown_rx.await.ok(); })
        .await
        .unwrap_or_else(|e| eprintln!("HTTP server error: {e}"));

    SERVER_RUNNING.store(false, Ordering::SeqCst);
}

// ─── Request/Response types ───

#[derive(Deserialize)]
struct AddBookmarkRequest {
    url: String,
    title: Option<String>,
    #[serde(default)]
    tags: Vec<String>,
    #[serde(default)]
    description: Option<String>,
}

#[derive(Deserialize)]
struct UpdateBookmarkRequest {
    title: Option<String>,
    #[serde(default)]
    tags: Vec<String>,
    #[serde(default)]
    description: Option<String>,
}

#[derive(Deserialize)]
struct CheckQuery {
    url: String,
}

#[derive(Serialize)]
struct CheckResponse {
    exists: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    bookmark: Option<serde_json::Value>,
}

// ─── Handlers ───

async fn add_bookmark_handler(
    Json(req): Json<AddBookmarkRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let title = req.title.unwrap_or_else(|| req.url.clone());
    let description = req.description.unwrap_or_default();
    let result = crate::bkmr::add_bookmark(&req.url, &title, &req.tags, &description).await;
    match result {
        Ok(id) => {
            notify_bookmarks_changed();
            Ok(Json(serde_json::json!({
                "id": id,
                "status": "created"
            })))
        }
        Err(e) => {
            let is_dup = e.contains("already exists");
            let status = if is_dup { StatusCode::CONFLICT } else { StatusCode::INTERNAL_SERVER_ERROR };
            Err((
                status,
                Json(serde_json::json!({
                    "error": e,
                    "duplicate": is_dup,
                })),
            ))
        }
    }
}

async fn check_bookmark_handler(
    Query(query): Query<CheckQuery>,
) -> Result<Json<CheckResponse>, (StatusCode, Json<serde_json::Value>)> {
    match crate::bkmr::check_bookmark(&query.url).await {
        Ok(Some(bm)) => Ok(Json(CheckResponse {
            exists: true,
            bookmark: Some(serde_json::to_value(&bm).unwrap()),
        })),
        Ok(None) => Ok(Json(CheckResponse {
            exists: false,
            bookmark: None,
        })),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e })),
        )),
    }
}

async fn update_bookmark_handler(
    Path(id): Path<u64>,
    Json(req): Json<UpdateBookmarkRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let title = req.title.unwrap_or_default();
    let description = req.description.unwrap_or_default();
    match crate::bkmr::update_bookmark(id, &title, &req.tags, &description).await {
        Ok(()) => {
            notify_bookmarks_changed();
            Ok(Json(serde_json::json!({
                "id": id,
                "status": "updated"
            })))
        }
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e })),
        )),
    }
}

async fn get_bookmark_handler(
    Path(id): Path<u64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    match crate::bkmr::show_bookmark(id).await {
        Ok(Some(bm)) => Ok(Json(serde_json::to_value(&bm).unwrap())),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": "Bookmark not found" })),
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e })),
        )),
    }
}

async fn delete_bookmark_handler(
    Path(id): Path<u64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    match crate::bkmr::delete_bookmarks(&[id]).await {
        Ok(_) => {
            notify_bookmarks_changed();
            Ok(Json(serde_json::json!({ "status": "deleted" })))
        }
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e })),
        )),
    }
}


async fn get_tags_handler() -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    match crate::bkmr::get_tags().await {
        Ok(tags) => Ok(Json(serde_json::to_value(tags).unwrap())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
    }
}

async fn docs_handler() -> Html<&'static str> {
    Html(include_str!("api_docs.html"))
}

// ─── Desktop app notification ───

fn notify_bookmarks_changed() {
    if let Some(handle) = APP_HANDLE.get() {
        let _ = handle.emit("bookmarks-changed", ());
    }
}
