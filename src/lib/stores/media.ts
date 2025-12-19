import { writable, derived, get } from 'svelte/store';
import type * as types from '../types';
import * as mediaApi from '../api/media';

export const mediaFiles = writable<types.MediaFile[]>([]);

export const currentMedia = writable<types.MediaFileWithDetails | null>(null);

export const selectedMediaIds = writable<number[]>([]);

export const allTags = writable<types.Tag[]>([]);

export const filters = writable<types.FilterOptions>({
  mediaTypes: [],
  minRating: 0,
  maxRating: 5,
  tags: [],
  favorite: null,
  searchQuery: '',
});

export const sortOptions = writable<types.SortOptions>({ field: 'createdAt', direction: 'desc' });

export const isLoadingMedia = writable(false);

export const loadMediaFiles = async () => {
  isLoadingMedia.set(true);
  try {
    const $filters = get(filters);
    const $sort = get(sortOptions);
    const files = await mediaApi.getMediaFiles($filters, $sort);
    mediaFiles.set(files);
  } catch (error) {
    console.error('Failed to load media files:', error);
    mediaFiles.set([]);
  } finally {
    isLoadingMedia.set(false);
  }
};

export const loadAllTags = async () => {
  try {
    const tags = await mediaApi.listTags();
    allTags.set(tags);
  } catch (error) {
    console.error('Failed to load tags:', error);
    allTags.set([]);
  }
};

export const loadMediaDetails = async (id: number) => {
  try {
    const details = await mediaApi.getMediaDetails(id);
    currentMedia.set(details);
  } catch (error) {
    console.error('Failed to load media details:', error);
    currentMedia.set(null);
  }
};

export const filteredMediaFiles = derived(
  [mediaFiles],
  ([$mediaFiles]) => $mediaFiles,
);

// Guard against stale data: reload if older than 30 seconds (example)
let lastLoadTime = 0;
export const reloadMediaIfStale = async () => {
  const now = Date.now();
  if (now - lastLoadTime > 30000) {
    lastLoadTime = now;
    await loadMediaFiles();
  }
};
