use std::sync::Arc;

use super::{
    AppResult, Bookmark, BookmarkPage, BookmarkPageRequest, BookmarkRepository, BookmarkSearch,
    CreateBookmark, SqliteBookmarkRepository, SqliteFtsSearch, TagSummary, UpdateBookmark,
};

type ChangeNotifier = Arc<dyn Fn() + Send + Sync>;

pub struct BookmarkService<R, S> {
    repository: R,
    search: S,
    notify_changed: ChangeNotifier,
}

impl<R, S> BookmarkService<R, S> {
    pub fn new(repository: R, search: S) -> Self {
        Self {
            repository,
            search,
            notify_changed: Arc::new(|| {}),
        }
    }

    pub fn with_change_notifier(mut self, notify_changed: ChangeNotifier) -> Self {
        self.notify_changed = notify_changed;
        self
    }
}

impl<R: BookmarkRepository, S: BookmarkSearch> BookmarkService<R, S> {
    pub fn query(&self, request: BookmarkPageRequest) -> AppResult<BookmarkPage> {
        let hits = self.search.search(&request)?;
        let items = self.repository.get_by_ids_ordered(&hits.bookmark_ids)?;
        Ok(BookmarkPage {
            items,
            next_cursor: hits.next_cursor,
        })
    }

    pub fn create(&self, input: CreateBookmark) -> AppResult<Bookmark> {
        let bookmark = self.repository.create(input)?;
        (self.notify_changed)();
        Ok(bookmark)
    }

    pub fn update(&self, id: i64, input: UpdateBookmark) -> AppResult<Bookmark> {
        let bookmark = self.repository.update(id, input)?;
        (self.notify_changed)();
        Ok(bookmark)
    }

    pub fn delete_many(&self, ids: Vec<i64>) -> AppResult<u64> {
        let deleted = self.repository.delete_many(&ids)?;
        if deleted > 0 {
            (self.notify_changed)();
        }
        Ok(deleted)
    }

    pub fn get_by_id(&self, id: i64) -> AppResult<Bookmark> {
        self.repository
            .get_by_id(id)?
            .ok_or_else(|| super::AppError::bookmark_not_found(id))
    }

    pub fn get_by_url(&self, url: String) -> AppResult<Option<Bookmark>> {
        self.repository.get_by_url(url.trim())
    }

    pub fn get_tags(&self) -> AppResult<Vec<TagSummary>> {
        self.repository.get_tags()
    }

    pub fn record_access(&self, id: i64) -> AppResult<Bookmark> {
        let bookmark = self.repository.record_access(id)?;
        (self.notify_changed)();
        Ok(bookmark)
    }
}

pub type SharedBookmarkService = Arc<BookmarkService<SqliteBookmarkRepository, SqliteFtsSearch>>;
