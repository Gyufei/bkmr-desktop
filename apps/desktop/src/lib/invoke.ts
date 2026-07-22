import { invoke } from '@tauri-apps/api/core';
import type { Bookmark, Tag, NoteFile } from '../types';

/* ───── Bookmarks ───── */

export async function invokeLoadAllBookmarks(): Promise<Bookmark[]> {
  return await invoke<Bookmark[]>('load_all_bookmarks');
}

export async function invokeGetAllTags(): Promise<Tag[]> {
  return await invoke<Tag[]>('get_all_tags');
}

export async function invokeBackupBookmarks(dir: string): Promise<string> {
  return await invoke<string>('backup_bookmarks', { dir });
}

export async function invokeAddBookmark(
  url: string,
  title: string,
  tags: string[],
  description?: string,
): Promise<number> {
  return await invoke<number>('add_bookmark', { url, title, tags, description });
}

export async function invokeHybridSearchBookmarks(
  query: string,
  tags: string[],
): Promise<Bookmark[]> {
  return await invoke<Bookmark[]>('hybrid_search_bookmarks', { query, tags });
}

export async function invokeDeleteBookmarks(ids: number[]): Promise<number> {
  return await invoke<number>('delete_bookmarks', { ids });
}

export async function invokeCheckBookmark(url: string): Promise<Bookmark | null> {
  return await invoke<Bookmark | null>('check_bookmark', { url });
}

export async function invokeUpdateBookmark(
  id: number,
  title: string,
  tags: string[],
  description?: string | null,
): Promise<void> {
  await invoke('update_bookmark', { id, title, tags, description: description ?? null });
}

export async function invokeRecordBookmarkAccess(id: number): Promise<void> {
  await invoke('record_bookmark_access', { id });
}

/* ───── Server ───── */

export async function invokeGetServerStatus(): Promise<{ running: boolean }> {
  return await invoke<{ running: boolean }>('get_server_status');
}

/* ───── Settings ───── */

export interface AppSettings {
  backup_dir: string | null;
  notes_dir: string | null;
}

export async function invokeGetSettings(): Promise<AppSettings> {
  return await invoke<AppSettings>('get_settings');
}

export async function invokeUpdateSettings(settings: AppSettings): Promise<void> {
  await invoke('update_settings', { settings });
}

/* ───── System ───── */

export interface SystemInfo {
  bkmr_config_path: string;
  sqlite_db_path: string;
  onnx_available: boolean;
  bkmr_version: string;
  bkmr_repo: string;
  app_version: string;
}

export async function invokeGetSystemInfo(): Promise<SystemInfo> {
  return await invoke<SystemInfo>('get_system_info');
}

/* ───── Notes ───── */

export async function invokeScanNotes(dir: string): Promise<NoteFile[]> {
  return await invoke<NoteFile[]>('scan_notes', { dir });
}

export async function invokeReadNoteFile(path: string): Promise<string> {
  return await invoke<string>('read_note_file', { path });
}

export async function invokeWriteNoteFile(path: string, content: string): Promise<void> {
  await invoke('write_note_file', { path, content });
}

export async function invokeCreateNoteFile(dir: string, name: string): Promise<string> {
  return await invoke<string>('create_note_file', { dir, name });
}

export async function invokeDeleteNote(path: string): Promise<void> {
  await invoke('delete_note', { path });
}

export async function invokeRenameNote(oldPath: string, newPath: string): Promise<void> {
  await invoke('rename_note', { oldPath, newPath });
}
