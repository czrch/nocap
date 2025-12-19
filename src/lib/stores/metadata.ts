import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';
import type { ExifSummary, ImageMetadata, UserMetadata } from '../types';

export type MetadataState = {
  path: string | null;
  image: ImageMetadata | null;
  exif: ExifSummary | null;
  loading: boolean;
  error: string | null;
  userSaved: UserMetadata;
  userDraft: UserMetadata;
  userDirty: boolean;
};

const defaultUserMetadata: UserMetadata = {
  description: '',
  tags: [],
  rating: null,
  author: '',
  date: null,
};

function makeDefaultUserMetadata(): UserMetadata {
  return { ...defaultUserMetadata };
}

const initialState: MetadataState = {
  path: null,
  image: null,
  exif: null,
  loading: false,
  error: null,
  userSaved: makeDefaultUserMetadata(),
  userDraft: makeDefaultUserMetadata(),
  userDirty: false,
};

const USER_METADATA_STORAGE_KEY = 'nocap_user_metadata_v1';

function normalizeTags(tags: string[]): string[] {
  const seen = new Set<string>();
  const normalized: string[] = [];

  for (const rawTag of tags) {
    const trimmed = rawTag.trim();
    if (!trimmed) continue;
    const key = trimmed.toLowerCase();
    if (seen.has(key)) continue;
    seen.add(key);
    normalized.push(trimmed);
  }

  return normalized;
}

function normalizeUserMetadata(user: UserMetadata): UserMetadata {
  const normalizedRating =
    user.rating === null ? null : Math.max(0, Math.min(5, Math.round(user.rating)));

  return {
    description: user.description.trim(),
    tags: normalizeTags(user.tags),
    rating: normalizedRating,
    author: user.author.trim(),
    date: user.date && user.date.trim() !== '' ? user.date : null,
  };
}

function userMetadataEquals(a: UserMetadata, b: UserMetadata): boolean {
  if (a.description !== b.description) return false;
  if (a.author !== b.author) return false;
  if (a.date !== b.date) return false;
  if (a.rating !== b.rating) return false;
  if (a.tags.length !== b.tags.length) return false;
  for (let i = 0; i < a.tags.length; i += 1) {
    if (a.tags[i] !== b.tags[i]) return false;
  }
  return true;
}

function coerceUserMetadata(value: unknown): UserMetadata {
  if (!value || typeof value !== 'object') return makeDefaultUserMetadata();
  const obj = value as Record<string, unknown>;

  const description = typeof obj.description === 'string' ? obj.description : '';
  const author = typeof obj.author === 'string' ? obj.author : '';
  const date = typeof obj.date === 'string' ? obj.date : null;

  const tags = Array.isArray(obj.tags)
    ? obj.tags.filter((t): t is string => typeof t === 'string')
    : [];

  const rating = typeof obj.rating === 'number' && Number.isFinite(obj.rating) ? obj.rating : null;

  return normalizeUserMetadata({
    description,
    tags,
    rating,
    author,
    date,
  });
}

function loadUserMetadataMap(): Record<string, unknown> {
  if (typeof window === 'undefined') return {};

  try {
    const stored = localStorage.getItem(USER_METADATA_STORAGE_KEY);
    if (!stored) return {};
    const parsed = JSON.parse(stored) as unknown;
    if (!parsed || typeof parsed !== 'object') return {};
    return parsed as Record<string, unknown>;
  } catch {
    return {};
  }
}

function saveUserMetadataMap(map: Record<string, UserMetadata>): void {
  if (typeof window === 'undefined') return;

  try {
    localStorage.setItem(USER_METADATA_STORAGE_KEY, JSON.stringify(map));
  } catch {
    // ignored
  }
}

function createMetadataStore() {
  const { subscribe, set, update } = writable<MetadataState>(initialState);

  return {
    subscribe,

    load: (path: string | null) => {
      if (!path) {
        set(initialState);
        return;
      }

      const map = loadUserMetadataMap();
      const normalizedStoredUser = coerceUserMetadata(map[path]);

      update((state) => ({
        ...state,
        path,
        loading: true,
        error: null,
        userSaved: normalizedStoredUser,
        userDraft: normalizedStoredUser,
        userDirty: false,
      }));

      const requestedPath = path;

      Promise.all([
        invoke<ImageMetadata>('get_image_metadata', { path: requestedPath }),
        invoke<ExifSummary | null>('get_exif_summary', { path: requestedPath }),
      ])
        .then(([image, exif]) => {
          update((state) => {
            if (state.path !== requestedPath) return state;
            return {
              ...state,
              image,
              exif,
              loading: false,
              error: null,
            };
          });
        })
        .catch((err) => {
          update((state) => {
            if (state.path !== requestedPath) return state;
            return {
              ...state,
              image: null,
              exif: null,
              loading: false,
              error: String(err),
            };
          });
        });
    },

    updateUserDraft: (patch: Partial<UserMetadata>) => {
      update((state) => {
        const nextDraft = normalizeUserMetadata({ ...state.userDraft, ...patch });
        const dirty = !userMetadataEquals(nextDraft, state.userSaved);
        return {
          ...state,
          userDraft: nextDraft,
          userDirty: dirty,
        };
      });
    },

    saveUserDraft: () => {
      update((state) => {
        if (!state.path) return state;
        const map = loadUserMetadataMap();
        const next = { ...(map as Record<string, UserMetadata>) };
        next[state.path] = state.userDraft;
        saveUserMetadataMap(next);
        return {
          ...state,
          userSaved: state.userDraft,
          userDirty: false,
        };
      });
    },

    discardUserDraft: () => {
      update((state) => ({
        ...state,
        userDraft: state.userSaved,
        userDirty: false,
      }));
    },

    reset: () => set(initialState),
  };
}

export const metadata = createMetadataStore();
