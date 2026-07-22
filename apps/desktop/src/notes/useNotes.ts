import { useState, useCallback, useEffect } from 'react';
import {
  invokeScanNotes,
  invokeReadNoteFile,
  invokeWriteNoteFile,
  invokeCreateNoteFile,
  invokeDeleteNote,
  invokeRenameNote,
} from '../lib/invoke';
import { listen } from '@tauri-apps/api/event';
import type { NoteFile } from '../types';

export function useNotes() {
  const [notes, setNotes] = useState<NoteFile[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // notify captures file changes and emits events
  useEffect(() => {
    const unlisten1 = listen<NoteFile>('note-changed', (event) => {
      const changed = event.payload;
      setNotes((prev) => {
        const idx = prev.findIndex((n) => n.path === changed.path);
        let next: NoteFile[];
        if (idx >= 0) {
          next = [...prev];
          next[idx] = changed;
        } else {
          next = [...prev, changed];
        }
        return next.sort((a, b) => a.title.localeCompare(b.title));
      });
    });
    const unlisten2 = listen<string>('note-removed', (event) => {
      const path = event.payload;
      setNotes((prev) => prev.filter((n) => n.path !== path));
    });
    return () => {
      unlisten1.then((fn) => fn());
      unlisten2.then((fn) => fn());
    };
  }, []);

  const scanDir = useCallback(async (dir: string): Promise<NoteFile[]> => {
    setLoading(true);
    setError(null);
    try {
      const result = await invokeScanNotes(dir);
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
    return await invokeReadNoteFile(path);
  }, []);

  const saveFile = useCallback(async (path: string, content: string): Promise<void> => {
    await invokeWriteNoteFile(path, content);
  }, []);

  const createFile = useCallback(async (dir: string, name: string): Promise<string> => {
    return await invokeCreateNoteFile(dir, name);
  }, []);

  const deleteNote = useCallback(async (path: string): Promise<void> => {
    await invokeDeleteNote(path);
  }, []);

  const renameNote = useCallback(async (oldPath: string, newPath: string): Promise<void> => {
    await invokeRenameNote(oldPath, newPath);
  }, []);

  return { notes, loading, error, scanDir, readFile, saveFile, createFile, deleteNote, renameNote };
}
