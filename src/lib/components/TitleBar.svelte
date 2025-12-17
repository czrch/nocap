<script lang="ts">
  import { isTauri } from '@tauri-apps/api/core';
  import { getCurrentWindow, type Window as TauriWindow } from '@tauri-apps/api/window';
  import { onDestroy, onMount } from 'svelte';
  import { viewer } from '../stores/viewer';
  import DropdownMenu from './DropdownMenu.svelte';
  import MenuList from './MenuList.svelte';

  import appIconUrl from '../../../src-tauri/icons/32x32.png?url';

  const isNative = isTauri();
  const appWindow: TauriWindow | null = isNative ? getCurrentWindow() : null;

  let isMaximized = false;
  let unlistenResize: (() => void) | null = null;

  onMount(async () => {
    if (!appWindow) return;

    try {
      await appWindow.setShadow(true);
    } catch {
      // setShadow is platform-specific (Linux unsupported); ignore.
    }

    try {
      isMaximized = await appWindow.isMaximized();
    } catch {
      isMaximized = false;
    }

    unlistenResize = await appWindow.listen('tauri://resize', async () => {
      try {
        isMaximized = await appWindow.isMaximized();
      } catch {
        isMaximized = false;
      }
    });
  });

  onDestroy(() => {
    unlistenResize?.();
  });

  async function minimize() {
    await appWindow?.minimize();
  }

  async function startDrag(event: PointerEvent) {
    if (event.button !== 0) return;
    try {
      await appWindow?.startDragging();
    } catch {
      // ignored
    }
  }

  async function toggleMaximize() {
    if (!appWindow) return;
    try {
      await appWindow.toggleMaximize();
    } catch {
      let maximized = false;
      try {
        maximized = await appWindow.isMaximized();
      } catch {
        maximized = false;
      }

      try {
        if (maximized) await appWindow.unmaximize();
        else await appWindow.maximize();
      } catch {
        // ignored
      }
    }

    try {
      isMaximized = await appWindow.isMaximized();
    } catch {
      isMaximized = false;
    }
  }

  async function close() {
    await appWindow?.close();
  }
</script>

{#if isNative}
  <div class="titlebar">
    <DropdownMenu menuAriaLabel="Application menu" align="left">
      <svelte:fragment slot="trigger" let:open let:toggle>
        <button
          type="button"
          class="hamburger-button"
          on:click={toggle}
          aria-haspopup="menu"
          aria-expanded={open}
          title="Menu"
          aria-label="Application menu"
        >
          ☰
        </button>
      </svelte:fragment>

      <svelte:fragment slot="menu" let:close>
        <MenuList closeMenu={close} />
      </svelte:fragment>
    </DropdownMenu>

    <div class="drag-region" data-tauri-drag-region on:pointerdown={startDrag}>
      <img class="app-icon" src={appIconUrl} alt="" aria-hidden="true" />
      <div class="title">
        {$viewer.currentImage?.filename ?? 'nocap'}
      </div>
    </div>

    <div class="window-controls" aria-label="Window controls">
      <button class="control" type="button" on:click={minimize} aria-label="Minimize">
        —
      </button>
      <button
        class="control"
        type="button"
        on:click={toggleMaximize}
        aria-label={isMaximized ? 'Restore' : 'Maximize'}
      >
        {isMaximized ? '❐' : '□'}
      </button>
      <button class="control close" type="button" on:click={close} aria-label="Close">
        ×
      </button>
    </div>
  </div>
{/if}

<style>
  .titlebar {
    display: flex;
    align-items: stretch;
    height: 32px;
    background: #141414;
    border-bottom: 1px solid #2a2a2a;
    flex-shrink: 0;
  }

  .hamburger-button {
    width: 38px;
    height: 32px;
    padding: 0;
    border: 0;
    border-right: 1px solid #2a2a2a;
    border-radius: 0;
    background: transparent;
    color: #cfcfcf;
    font-size: 1.1rem;
    line-height: 1;
    cursor: pointer;
    transition: background 0.15s ease;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .hamburger-button:hover {
    background: #2a2a2a;
  }

  .hamburger-button:active {
    background: #242424;
  }

  .drag-region {
    flex: 1;
    display: flex;
    align-items: center;
    min-width: 0;
    padding: 0 0.75rem;
    gap: 0.5rem;
  }

  .app-icon {
    width: 16px;
    height: 16px;
    flex: 0 0 auto;
    user-select: none;
    -webkit-user-drag: none;
  }

  .title {
    font-size: 0.85rem;
    color: #cfcfcf;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    user-select: none;
  }

  .window-controls {
    display: flex;
    align-items: stretch;
  }

  .control {
    width: 46px;
    padding: 0;
    border: 0;
    border-left: 1px solid #2a2a2a;
    border-radius: 0;
    background: transparent;
    color: #cfcfcf;
    font-size: 0.95rem;
    line-height: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: background 0.15s ease;
  }

  .control:hover {
    background: #2a2a2a;
  }

  .control:active {
    background: #242424;
  }

  .control.close:hover {
    background: #c42b1c;
    color: #fff;
  }

  .control.close:active {
    background: #a82315;
  }
</style>