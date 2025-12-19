import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';
import type { ImageFile, ViewerState } from '../types';

const initialState: ViewerState = {
  currentImage: null,
  imageList: [],
  currentIndex: -1,
  zoomLevel: 1.0,
  fitToWindow: true,
};

function createViewerStore() {
  const { subscribe, update } = writable<ViewerState>(initialState);

  return {
    subscribe,

    loadImage: (file: ImageFile) => {
      update((state) => ({
        ...state,
        currentImage: file,
        zoomLevel: 1.0,
        fitToWindow: true,
      }));

      // Load all images in the same directory as the selected image.
      // Guard against out-of-order responses (e.g., user rapidly changes images).
      invoke<ImageFile[]>('get_adjacent_images', { currentPath: file.path })
        .then((images) => {
          const index = images.findIndex((img) => img.path === file.path);
          update((state) => {
            if (state.currentImage?.path !== file.path) return state;
            return {
              ...state,
              imageList: images,
              currentIndex: index,
            };
          });
        })
        .catch((err) => console.error('Failed to load adjacent images:', err));
    },

    loadFolder: async (files: ImageFile[]) => {
      update((state) => ({
        ...state,
        imageList: files,
        currentImage: files.length > 0 ? files[0] : null,
        currentIndex: files.length > 0 ? 0 : -1,
        zoomLevel: 1.0,
        fitToWindow: true,
      }));
    },

    nextImage: () => {
      update((state) => {
        if (state.imageList.length === 0) return state;

        const nextIndex = (state.currentIndex + 1) % state.imageList.length;
        return {
          ...state,
          currentImage: state.imageList[nextIndex],
          currentIndex: nextIndex,
          zoomLevel: 1.0,
          fitToWindow: true,
        };
      });
    },

    previousImage: () => {
      update((state) => {
        if (state.imageList.length === 0) return state;

        const prevIndex =
          state.currentIndex === 0 ? state.imageList.length - 1 : state.currentIndex - 1;

        return {
          ...state,
          currentImage: state.imageList[prevIndex],
          currentIndex: prevIndex,
          zoomLevel: 1.0,
          fitToWindow: true,
        };
      });
    },

    setZoom: (level: number) => {
      update((state) => ({
        ...state,
        zoomLevel: Math.max(0.1, Math.min(10, level)),
        fitToWindow: false,
      }));
    },

    zoomIn: () => {
      update((state) => ({
        ...state,
        zoomLevel: Math.min(10, state.zoomLevel * 1.2),
        fitToWindow: false,
      }));
    },

    zoomOut: () => {
      update((state) => ({
        ...state,
        zoomLevel: Math.max(0.1, state.zoomLevel / 1.2),
        fitToWindow: false,
      }));
    },

    resetZoom: () => {
      update((state) => ({
        ...state,
        zoomLevel: 1.0,
        fitToWindow: false,
      }));
    },

    toggleFitToWindow: () => {
      update((state) => ({
        ...state,
        fitToWindow: !state.fitToWindow,
        zoomLevel: state.fitToWindow ? 1.0 : state.zoomLevel,
      }));
    },
  };
}

export const viewer = createViewerStore();
