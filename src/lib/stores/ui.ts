import { writable } from 'svelte/store';

export type UiState = {
  settingsOpen: boolean;
};

const initialState: UiState = {
  settingsOpen: false,
};

function createUiStore() {
  const { subscribe, update } = writable<UiState>(initialState);

  return {
    subscribe,
    openSettings: () => update(state => ({ ...state, settingsOpen: true })),
    closeSettings: () => update(state => ({ ...state, settingsOpen: false })),
    toggleSettings: () =>
      update(state => ({ ...state, settingsOpen: !state.settingsOpen })),
  };
}

export const ui = createUiStore();

