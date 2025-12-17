import { writable } from 'svelte/store';

export type UiScale = 100 | 110 | 125 | 150 | 175 | 200;

export interface Settings {
  uiScale: UiScale;
}

const STORAGE_KEY = 'nocap_settings';

const defaultSettings: Settings = {
  uiScale: 100,
};

function loadSettings(): Settings {
  if (typeof window === 'undefined') return defaultSettings;

  try {
    const stored = localStorage.getItem(STORAGE_KEY);
    if (stored) {
      const parsed = JSON.parse(stored);
      return {
        ...defaultSettings,
        ...parsed,
      };
    }
  } catch (err) {
    console.error('Failed to load settings:', err);
  }

  return defaultSettings;
}

function saveSettings(settings: Settings): void {
  if (typeof window === 'undefined') return;

  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(settings));
  } catch (err) {
    console.error('Failed to save settings:', err);
  }
}

function createSettingsStore() {
  const { subscribe, set, update } = writable<Settings>(loadSettings());

  return {
    subscribe,
    setUiScale: (scale: UiScale) => {
      update(state => {
        const newState = { ...state, uiScale: scale };
        saveSettings(newState);
        return newState;
      });
    },
    reset: () => {
      set(defaultSettings);
      saveSettings(defaultSettings);
    },
  };
}

export const settings = createSettingsStore();