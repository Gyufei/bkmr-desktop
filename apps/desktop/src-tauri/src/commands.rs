use serde::Serialize;
use tauri::State;

use crate::bookmarks::{
    AppResult, Bookmark, BookmarkPage, BookmarkPageRequest, CreateBookmark, ImportPreview,
    SharedBookmarkService, TagSummary, UpdateBookmark,
};

#[tauri::command]
pub fn query_bookmarks(
    service: State<'_, SharedBookmarkService>,
    request: BookmarkPageRequest,
) -> AppResult<BookmarkPage> {
    service.query(request)
}

#[tauri::command]
pub fn create_bookmark(
    service: State<'_, SharedBookmarkService>,
    input: CreateBookmark,
) -> AppResult<Bookmark> {
    service.create(input)
}

#[tauri::command]
pub fn update_bookmark(
    service: State<'_, SharedBookmarkService>,
    id: i64,
    input: UpdateBookmark,
) -> AppResult<Bookmark> {
    service.update(id, input)
}

#[tauri::command]
pub fn delete_bookmarks(
    service: State<'_, SharedBookmarkService>,
    ids: Vec<i64>,
) -> AppResult<u64> {
    service.delete_many(ids)
}

#[tauri::command]
pub fn get_bookmark_by_url(
    service: State<'_, SharedBookmarkService>,
    url: String,
) -> AppResult<Option<Bookmark>> {
    service.get_by_url(url)
}

#[tauri::command]
pub fn get_tags(service: State<'_, SharedBookmarkService>) -> AppResult<Vec<TagSummary>> {
    service.get_tags()
}

#[tauri::command]
pub fn record_bookmark_access(
    service: State<'_, SharedBookmarkService>,
    id: i64,
) -> AppResult<Bookmark> {
    service.record_access(id)
}

#[tauri::command]
pub fn export_bookmarks(
    service: State<'_, SharedBookmarkService>,
    directory: String,
) -> AppResult<String> {
    service
        .export_bookmarks(directory)
        .map(|path| path.to_string_lossy().into_owned())
}

#[tauri::command]
pub fn preview_bookmark_import(
    service: State<'_, SharedBookmarkService>,
    path: String,
) -> AppResult<ImportPreview> {
    service.preview_bookmark_import(path)
}

#[tauri::command]
pub fn apply_bookmark_import(
    service: State<'_, SharedBookmarkService>,
    path: String,
    file_hash: String,
) -> AppResult<ImportPreview> {
    service.apply_bookmark_import(path, &file_hash)
}

#[tauri::command]
pub async fn scan_notes(dir: String) -> Result<Vec<crate::notes::NoteFile>, String> {
    crate::notes::scan_notes(&dir)
}

#[tauri::command]
pub async fn read_note_file(path: String) -> Result<String, String> {
    crate::notes::read_note_file(&path)
}

#[tauri::command]
pub async fn write_note_file(path: String, content: String) -> Result<(), String> {
    crate::notes::write_note_file(&path, &content)
}

#[tauri::command]
pub async fn create_note_file(dir: String, name: String) -> Result<String, String> {
    crate::notes::create_note_file(&dir, &name)
}

#[tauri::command]
pub async fn get_settings() -> Result<crate::settings::Settings, String> {
    Ok(crate::settings::load())
}

#[tauri::command]
pub async fn update_settings(settings: crate::settings::Settings) -> Result<(), String> {
    crate::settings::save(&settings)
}

#[tauri::command]
pub async fn get_server_status() -> Result<crate::http_server::ServerStatus, String> {
    Ok(crate::http_server::status())
}

#[tauri::command]
pub async fn delete_note(path: String) -> Result<(), String> {
    crate::notes::delete_note_file(&path)
}

#[tauri::command]
pub async fn rename_note(old_path: String, new_path: String) -> Result<(), String> {
    crate::notes::rename_note_file(&old_path, &new_path)
}

#[derive(Debug, Clone, Serialize)]
pub struct SystemInfo {
    pub bkmr_config_path: String,
    pub sqlite_db_path: String,
    pub onnx_available: bool,
    pub bkmr_version: String,
    pub bkmr_repo: String,
    pub app_version: String,
}

#[tauri::command]
pub async fn get_system_info() -> Result<SystemInfo, String> {
    let config_path = bkmr_lib::config::get_config_file_path()
        .map(|path| path.to_string_lossy().to_string())
        .unwrap_or_else(|| "unknown".to_owned());
    let db_path = crate::container::get_db_path().to_owned();
    let onnx_available = crate::container::is_embedding_available();

    Ok(SystemInfo {
        bkmr_config_path: config_path,
        sqlite_db_path: db_path,
        onnx_available,
        bkmr_version: "7.6.7".to_owned(),
        bkmr_repo: "https://github.com/sysid/bkmr".to_owned(),
        app_version: env!("CARGO_PKG_VERSION").to_owned(),
    })
}
