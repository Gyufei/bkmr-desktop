import { describe, expect, it, vi } from 'vitest';

const writes: Array<{ path: string; content: string }> = [];
const releases: Array<() => void> = [];

vi.mock('./notes.api', () => ({
  writeNoteContentApi: vi.fn((input: { path: string; content: string }) => {
    writes.push(input);
    return new Promise<void>((resolve) => releases.push(resolve));
  }),
}));

import { sharedNoteSaveQueue } from './note-save';

describe('sharedNoteSaveQueue', () => {
  it('orders the same path across separate consumers', async () => {
    const firstConsumerSave = sharedNoteSaveQueue.enqueue('/shared.md', 'old');
    const secondConsumerSave = sharedNoteSaveQueue.enqueue('/shared.md', 'new');

    expect(writes).toEqual([{ path: '/shared.md', content: 'old' }]);
    releases.shift()!();
    await firstConsumerSave;
    await vi.waitFor(() =>
      expect(writes).toEqual([
        { path: '/shared.md', content: 'old' },
        { path: '/shared.md', content: 'new' },
      ]),
    );
    releases.shift()!();
    await secondConsumerSave;
  });
});
