import { useState, useCallback, useEffect, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type { NoteFile } from "../types";

export function useNotes() {
  const [notes, setNotes] = useState<NoteFile[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const lastDirRef = useRef<string | null>(null);

  // When background polling detects file changes, re-scan
  useEffect(() => {
    const unlisten = listen("notes-refreshed", () => {
      const dir = lastDirRef.current;
      if (dir) {
        scanDir(dir).catch(() => {});
      }
    });
    return () => { unlisten.then(fn => fn()); };
  }, []);

  const scanDir = useCallback(async (dir: string): Promise<NoteFile[]> => {
    lastDirRef.current = dir;
    setLoading(true);
    setError(null);
    try {
      const result = await invoke<NoteFile[]>("scan_notes", { dir });
      setNotes(result);
      return result;
    } catch (e) {
      setError(String(e));
      setNotes([]);
      return [];
    } finally {
      setLoading(false);
    }
  }, []);

  const readFile = useCallback(async (path: string): Promise<string> => {
    return await invoke<string>("read_note_file", { path });
  }, []);

  const saveFile = useCallback(async (path: string, content: string): Promise<void> => {
    await invoke("write_note_file", { path, content });
  }, []);

  const createFile = useCallback(async (dir: string, name: string): Promise<string> => {
    return await invoke<string>("create_note_file", { dir, name });
  }, []);

  const deleteNote = useCallback(async (path: string): Promise<void> => {
    await invoke("delete_note", { path });
  }, []);

  const renameNote = useCallback(async (oldPath: string, newPath: string): Promise<void> => {
    await invoke("rename_note", { oldPath, newPath });
  }, []);

  return { notes, loading, error, scanDir, readFile, saveFile, createFile, deleteNote, renameNote };
}
