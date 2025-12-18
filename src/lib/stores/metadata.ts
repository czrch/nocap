import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';
import type { ExifSummary } from '../types';

export type MetadataState = {
  path: string | null;
  exif: ExifSummary | null;
  loading: boolean;
  error: string | null;
};

const initialState: MetadataState = {
  path: null,
  exif: null,
  loading: false,
  error: null,
};

function createMetadataStore() {
  const { subscribe, set, update } = writable<MetadataState>(initialState);

  return {
    subscribe,

    loadExif: (path: string | null) => {
      if (!path) {
        set(initialState);
        return;
      }

      update((state) => ({
        ...state,
        path,
        loading: true,
        error: null,
      }));

      const requestedPath = path;
      invoke<ExifSummary | null>('get_exif_summary', { path: requestedPath })
        .then((exif) => {
          update((state) => {
            if (state.path !== requestedPath) return state;
            return {
              ...state,
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
              exif: null,
              loading: false,
              error: String(err),
            };
          });
        });
    },

    reset: () => set(initialState),
  };
}

export const metadata = createMetadataStore();
