import SearchBar from "./SearchBar";
import TagPanel from "./TagPanel";
import ResultList from "./ResultList";
import { Button } from "./ui/button";
import { Plus } from "lucide-react";
import type { Bookmark, Tag } from "../types";

interface Props {
  tagVersion: number;
  fetchTags: () => Promise<Tag[]>;
  selectedTags: string[];
  onTagsChange: (tags: string[]) => void;
  onSearch: (query: string) => void;
  loading: boolean;
  visibleBookmarks: Bookmark[];
  error: string | null;
  hasMore: boolean;
  onLoadMore: () => void;
  onDeleteBookmark: (id: number) => void;
  onUpdateBookmark: (id: number, title: string, tags: string[], description?: string) => Promise<void>;
  onOpenAddDialog: () => void;
}

export default function BookmarkView({
  tagVersion, fetchTags, selectedTags, onTagsChange,
  onSearch, loading,
  visibleBookmarks, error, hasMore, onLoadMore,
  onDeleteBookmark, onUpdateBookmark,
  onOpenAddDialog,
}: Props) {
  return (
    <>
      <div className="shrink-0 px-4 py-3 border-b border-border dark:border-border-dark">
        <div className="flex items-center gap-2">
          <SearchBar onSearch={onSearch} loading={loading} />
          <Button variant="outline" className="h-10 w-10 shrink-0 !px-0 flex items-center justify-center" onClick={onOpenAddDialog} title="添加书签">
            <Plus className="h-5 w-5" />
          </Button>
        </div>
      </div>
      <div className="flex-1 flex overflow-hidden">
        <aside className="w-56 shrink-0 border-r border-border dark:border-border-dark bg-surface-sidebar dark:bg-surface-dark-sidebar p-3 flex flex-col">
          <TagPanel key={tagVersion}
            fetchTags={fetchTags}
            selectedTags={selectedTags}
            onTagsChange={onTagsChange}
          />
        </aside>
        <main className="flex-1 flex flex-col overflow-hidden">
          <div className="flex-1 overflow-y-auto p-3 thin-scrollbar">
            <ResultList
              bookmarks={visibleBookmarks}
              loading={loading}
              error={error}
              hasMore={hasMore}
              onLoadMore={onLoadMore}
              onDeleteBookmark={onDeleteBookmark}
              onUpdateBookmark={onUpdateBookmark}
            />
          </div>
        </main>
      </div>
    </>
  );
}
