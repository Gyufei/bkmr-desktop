import { NoteSaveQueue } from './note-save-queue';
import { writeNoteContentApi } from './notes.api';

export const sharedNoteSaveQueue = new NoteSaveQueue((path, content) =>
  writeNoteContentApi({ path, content }),
);
