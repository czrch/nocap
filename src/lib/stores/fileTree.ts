import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { get, writable } from 'svelte/store';
import type { FsChangeEvent, FsEntry } from '../types';

interface FileTreeState {
  rootPath: string | null;
  tree: FsEntry | null;
  expanded: Set<string>;
  loaded: Set<string>;
  loading: boolean;
  requestId: number;
}

const initialState: FileTreeState = {
  rootPath: null,
  tree: null,
  expanded: new Set(),
  loaded: new Set(),
  loading: false,
  requestId: 0,
};

function findNode(tree: FsEntry | null, path: string): FsEntry | null {
  if (!tree) return null;
  if (tree.path === path) return tree;
  if (tree.kind !== 'directory') return null;

  for (const child of tree.children) {
    const match = findNode(child, path);
    if (match) return match;
  }

  return null;
}

function updateNode(tree: FsEntry, path: string, updater: (node: FsEntry) => FsEntry): FsEntry {
  if (tree.path === path) return updater(tree);
  if (tree.kind !== 'directory') return tree;

  let changed = false;
  const children = tree.children.map((child) => {
    const updated = updateNode(child, path, updater);
    if (updated !== child) changed = true;
    return updated;
  });

  if (!changed) return tree;
  return { ...tree, children };
}

function createFileTreeStore() {
  const { subscribe, update } = writable<FileTreeState>(initialState);
  let rootRequestId = 0;
  const childRequests = new Map<string, number>();
  let unlisten: UnlistenFn | null = null;
  let watchRoot: string | null = null;
  let refreshTimer: ReturnType<typeof setTimeout> | null = null;

  async function refreshRoot(): Promise<void> {
    const state = get({ subscribe });
    if (!state.rootPath) return;

    const requestId = ++rootRequestId;
    update((prev) => ({
      ...prev,
      loading: true,
      requestId,
    }));

    try {
      const tree = await invoke<FsEntry>('list_dir_tree', {
        rootPath: state.rootPath,
        depth: 1,
      });

      update((prev) => {
        if (requestId !== rootRequestId) return prev;
        const expanded = new Set(prev.expanded);
        expanded.add(state.rootPath ?? tree.path);
        const loaded = new Set(prev.loaded);
        loaded.add(state.rootPath ?? tree.path);
        return {
          ...prev,
          loading: false,
          tree,
          expanded,
          loaded,
        };
      });
    } catch (err) {
      console.error('Failed to load folder tree:', err);
      update((prev) => ({ ...prev, loading: false }));
    }
  }

  async function loadChildren(path: string): Promise<void> {
    const requestId = (childRequests.get(path) ?? 0) + 1;
    childRequests.set(path, requestId);

    try {
      const subtree = await invoke<FsEntry>('list_dir_tree', {
        rootPath: path,
        depth: 1,
      });

      update((prev) => {
        if (childRequests.get(path) !== requestId || !prev.tree) return prev;
        const tree = updateNode(prev.tree, path, (node) => ({
          ...node,
          children: subtree.children,
        }));
        const loaded = new Set(prev.loaded);
        loaded.add(path);
        return { ...prev, tree, loaded };
      });
    } catch (err) {
      console.error('Failed to load folder children:', err);
    }
  }

  function setRoot(rootPath: string | null): void {
    if (refreshTimer) {
      clearTimeout(refreshTimer);
      refreshTimer = null;
    }
    if (unlisten && watchRoot && watchRoot !== rootPath) {
      unlisten();
      unlisten = null;
      watchRoot = null;
    }
    update(() => ({
      ...initialState,
      rootPath,
    }));
  }

  function toggle(path: string): void {
    let shouldLoad = false;

    update((state) => {
      const expanded = new Set(state.expanded);
      const isExpanded = expanded.has(path);

      if (isExpanded) {
        expanded.delete(path);
      } else {
        expanded.add(path);
        const node = findNode(state.tree, path);
        if (node?.kind === 'directory' && !state.loaded.has(path)) {
          shouldLoad = true;
        }
      }

      return { ...state, expanded };
    });

    if (shouldLoad) {
      void loadChildren(path);
    }
  }

  function scheduleRefresh(): void {
    if (refreshTimer) clearTimeout(refreshTimer);
    refreshTimer = setTimeout(() => {
      void refreshRoot();
    }, 150);
  }

  async function startWatch(): Promise<void> {
    const state = get({ subscribe });
    if (!state.rootPath) return;

    if (watchRoot === state.rootPath && unlisten) return;

    watchRoot = state.rootPath;
    if (unlisten) {
      unlisten();
      unlisten = null;
    }

    try {
      await invoke('start_watch', { rootPath: state.rootPath });
      unlisten = await listen<FsChangeEvent>('fs://changed', () => {
        scheduleRefresh();
      });
    } catch (err) {
      console.error('Failed to start file watcher:', err);
    }
  }

  return {
    subscribe,
    setRoot,
    refreshRoot,
    toggle,
    startWatch,
  };
}

export const fileTree = createFileTreeStore();
