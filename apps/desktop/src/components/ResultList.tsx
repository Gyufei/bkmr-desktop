import { useRef, useEffect, useCallback, useState } from "react";
import { Trash2 } from "lucide-react";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
  DialogFooter,
} from "./ui/dialog";
import { Button } from "./ui/button";
import { tagColor } from "../utils/tagColor";
import { open } from "@tauri-apps/plugin-shell";
import type { Bookmark } from "../types";

interface Props {
  bookmarks: Bookmark[];
  loading: boolean;
  error: string | null;
  hasMore: boolean;
  onLoadMore: () => void;
  onDeleteBookmark: (id: number) => void;
}

export default function ResultList({ bookmarks, loading, error, hasMore, onLoadMore, onDeleteBookmark }: Props) {
  const sentinelRef = useRef<HTMLDivElement>(null);
  const [deleteTarget, setDeleteTarget] = useState<Bookmark | null>(null);

  const handleIntersect = useCallback(
    (entries: IntersectionObserverEntry[]) => {
      if (entries[0]?.isIntersecting && hasMore && !loading) {
        onLoadMore();
      }
    },
    [hasMore, loading, onLoadMore]
  );

  useEffect(() => {
    const sentinel = sentinelRef.current;
    if (!sentinel) return;

    const observer = new IntersectionObserver(handleIntersect, {
      rootMargin: "200px",
    });
    observer.observe(sentinel);
    return () => observer.disconnect();
  }, [handleIntersect]);

  if (error) {
    return (
      <div className="flex items-center justify-center h-48 text-sm text-danger dark:text-danger-dark">
        {error}
      </div>
    );
  }

  if (!loading && bookmarks.length === 0) {
    return (
      <div className="flex items-center justify-center h-48 text-sm text-text-secondary dark:text-text-dark-secondary">
        输入关键词搜索书签
      </div>
    );
  }

  return (
    <div className="space-y-1">
      {bookmarks.map((bm) => (
        <BookmarkRow key={bm.id} bookmark={bm} onRequestDelete={setDeleteTarget} />
      ))}

      {/* Delete confirmation dialog */}
      <Dialog open={deleteTarget !== null} onOpenChange={(open) => { if (!open) setDeleteTarget(null); }}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>确认删除</DialogTitle>
            <DialogDescription>
              确定要删除书签「{deleteTarget?.title || deleteTarget?.url}」吗？此操作不可撤销。
            </DialogDescription>
          </DialogHeader>
          <DialogFooter>
            <Button variant="outline" onClick={() => setDeleteTarget(null)}>取消</Button>
            <Button
              variant="destructive"
              onClick={() => {
                if (deleteTarget) {
                  onDeleteBookmark(deleteTarget.id);
                  setDeleteTarget(null);
                }
              }}
            >
              删除
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>

      {/* Sentinel for infinite scroll */}
      <div ref={sentinelRef} className="h-4" />

      {/* Loading indicator */}
      {loading && bookmarks.length > 0 && (
        <div className="flex items-center justify-center py-4 text-sm text-text-secondary dark:text-text-dark-secondary">
          <div className="w-4 h-4 mr-2 border-2 border-accent dark:border-accent-dark border-t-transparent rounded-full animate-spin" />
          加载中...
        </div>
      )}

      {/* All loaded */}
      {!hasMore && bookmarks.length > 0 && (
        <div className="text-center py-4 text-sm text-text-secondary dark:text-text-dark-secondary">
          已显示全部 {bookmarks.length} 条结果
        </div>
      )}
    </div>
  );
}

function BookmarkRow({ bookmark, onRequestDelete }: { bookmark: Bookmark; onRequestDelete: (bm: Bookmark) => void }) {
  const handleClick = () => {
    open(bookmark.url);
  };

  return (
    <div className="group relative">
      <div
        onClick={handleClick}
        className="block px-4 py-3 rounded-card hover:bg-accent-bg dark:hover:bg-accent-dark-bg cursor-pointer transition-colors"
      >
        <div className="text-base font-medium text-text-primary dark:text-text-dark-primary group-hover:text-accent dark:group-hover:text-accent-dark transition-colors truncate pr-6">
          {bookmark.title || bookmark.url}
        </div>
        <div className="text-xs text-text-secondary dark:text-text-dark-secondary truncate mt-0.5">
          {bookmark.url}
        </div>
        {bookmark.description && (
          <div className="text-xs text-text-secondary dark:text-text-dark-secondary mt-1 line-clamp-2">
            {bookmark.description}
          </div>
        )}
        {bookmark.tags.length > 0 && (
          <div className="flex flex-wrap gap-1 mt-1.5">
            {bookmark.tags.map((tag) => (
              <span
                key={tag}
                className="inline-block px-2 py-0.5 text-xs rounded-chip"
                style={tagColor(tag)}
              >
                {tag}
              </span>
            ))}
          </div>
        )}
      </div>
      <button
        onClick={(e) => { e.stopPropagation(); onRequestDelete(bookmark); }}
        className="absolute right-2 top-2 opacity-0 group-hover:opacity-100 transition-opacity duration-150 p-1.5 rounded-md text-text-secondary dark:text-text-dark-secondary hover:text-danger dark:hover:text-danger-dark hover:bg-danger/10 dark:hover:bg-danger-dark/10"
        title="删除书签"
      >
        <Trash2 className="h-4 w-4" />
      </button>
    </div>
  );
}
