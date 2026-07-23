pub mod model;
pub mod repository;

pub use model::{
    AppError, AppResult, Bookmark, BookmarkPage, BookmarkPageRequest, CreateBookmark, TagSummary,
    UpdateBookmark,
};
pub use repository::{BookmarkRepository, SqliteBookmarkRepository};
