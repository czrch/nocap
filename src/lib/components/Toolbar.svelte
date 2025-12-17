<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { message, open } from '@tauri-apps/plugin-dialog';
  import { viewer } from '../stores/viewer';
  import { ui } from '../stores/ui';
  import type { ImageFile } from '../types';

  import DropdownMenu from './DropdownMenu.svelte';
  import SettingsDialog from './SettingsDialog.svelte';

  async function openFile() {
    try {
      const selected = await open({
        multiple: false,
        filters: [
          {
            name: 'Images',
            extensions: ['jpg', 'jpeg', 'png', 'gif', 'bmp', 'webp', 'svg'],
          },
        ],
      });

      if (selected && typeof selected === 'string') {
        // Extract filename and extension from path
        const pathParts = selected.split(/[/\\]/);
        const filename = pathParts[pathParts.length - 1];
        const extensionMatch = filename.match(/\.([^.]+)$/);
        const extension = extensionMatch ? extensionMatch[1].toLowerCase() : '';

        const file: ImageFile = {
          path: selected,
          filename: filename,
          extension: extension,
        };

        viewer.loadImage(file);
      }
    } catch (err) {
      console.error('Failed to open file:', err);
    }
  }

  async function openFolder() {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
      });

      if (selected && typeof selected === 'string') {
        // Scan the folder for images using the backend
        const images = await invoke<ImageFile[]>('scan_folder_for_images', {
          folderPath: selected,
        });

        if (images && images.length > 0) {
          viewer.loadFolder(images);
        } else {
          await message('No images found in the selected folder.', {
            title: 'nocap',
            kind: 'info',
          });
        }
      }
    } catch (err) {
      console.error('Failed to open folder:', err);
    }
  }

  function zoomIn() {
    viewer.zoomIn();
  }

  function zoomOut() {
    viewer.zoomOut();
  }

  function resetZoom() {
    viewer.resetZoom();
  }

  function toggleFit() {
    viewer.toggleFitToWindow();
  }
</script>

<div class="toolbar">
  <div class="toolbar-left">
    <DropdownMenu menuAriaLabel="File menu" align="left">
      <svelte:fragment slot="trigger" let:open let:toggle>
        <button
          type="button"
          class="menu-button"
          on:click={toggle}
          aria-haspopup="menu"
          aria-expanded={open}
          title="File"
          aria-label="File menu"
        >
          📄 ▾
        </button>
      </svelte:fragment>

      <svelte:fragment slot="menu" let:close>
        <button
          type="button"
          class="menu-item"
          role="menuitem"
          on:click={() => {
            close();
            void openFile();
          }}
          title="Open Image (Ctrl+O)"
        >
          Open File…
        </button>
        <button
          type="button"
          class="menu-item"
          role="menuitem"
          on:click={() => {
            close();
            void openFolder();
          }}
          title="Open Folder (Ctrl+Shift+O)"
        >
          Open Folder…
        </button>
      </svelte:fragment>
    </DropdownMenu>

    <button
      type="button"
      class="icon-button"
      on:click={() => ui.openSettings()}
      title="Preferences"
      aria-label="Preferences"
    >
      ⚙
    </button>
  </div>

  <div class="toolbar-center">
    {#if $viewer.currentImage}
      <span class="filename-text" title={$viewer.currentImage.path}>
        {$viewer.currentImage.filename}
      </span>
    {:else}
      <span class="filename-text no-image">No image loaded</span>
    {/if}
  </div>

  <div class="toolbar-right">
    <DropdownMenu menuAriaLabel="Zoom menu" align="right">
      <svelte:fragment slot="trigger" let:open let:toggle>
        <button
          type="button"
          class="menu-button"
          on:click={toggle}
          aria-haspopup="menu"
          aria-expanded={open}
          disabled={!$viewer.currentImage}
          title={`Zoom (${Math.round($viewer.zoomLevel * 100)}%)`}
        >
          🔍 {Math.round($viewer.zoomLevel * 100)}% ▾
        </button>
      </svelte:fragment>

      <svelte:fragment slot="menu" let:close>
        <button
          type="button"
          class="menu-item"
          role="menuitem"
          on:click={() => {
            close();
            zoomIn();
          }}
          disabled={!$viewer.currentImage}
          title="Zoom In (+)"
        >
          Zoom In
        </button>
        <button
          type="button"
          class="menu-item"
          role="menuitem"
          on:click={() => {
            close();
            zoomOut();
          }}
          disabled={!$viewer.currentImage}
          title="Zoom Out (-)"
        >
          Zoom Out
        </button>
        <button
          type="button"
          class="menu-item"
          role="menuitem"
          on:click={() => {
            close();
            resetZoom();
          }}
          disabled={!$viewer.currentImage}
          title="Reset Zoom (0)"
        >
          Reset Zoom
        </button>
        <button
          type="button"
          class="menu-item"
          role="menuitem"
          on:click={() => {
            close();
            toggleFit();
          }}
          disabled={!$viewer.currentImage}
          title="Fit to Window (F)"
        >
          {$viewer.fitToWindow ? 'Disable Fit to Window' : 'Fit to Window'}
        </button>
      </svelte:fragment>
    </DropdownMenu>
  </div>
</div>

<SettingsDialog open={$ui.settingsOpen} on:close={() => ui.closeSettings()} />

<style>
  .toolbar {
    display: grid;
    grid-template-columns: auto 1fr auto;
    align-items: center;
    gap: 1rem;
    background: #1a1a1a;
    border-bottom: 1px solid #333;
    padding: 0.5rem 1rem;
    flex-shrink: 0;
  }

  .toolbar-left,
  .toolbar-right {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .toolbar-center {
    display: flex;
    align-items: center;
    justify-content: center;
    min-width: 0;
  }

  .filename-text {
    font-size: 0.9rem;
    color: #ddd;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 400px;
  }

  .filename-text.no-image {
    color: #666;
  }

  button {
    padding: 0.4rem 0.8rem;
    font-size: 0.85rem;
    white-space: nowrap;
  }

  .menu-button {
    padding: 0.4rem 0.7rem;
  }

  .menu-item {
    width: 100%;
    text-align: left;
    border: 0;
    border-radius: 4px;
    padding: 0.55rem 0.65rem;
    background: transparent;
  }

  .menu-item:hover {
    background: #2a2a2a;
  }

  .menu-item:active {
    background: #242424;
  }

  .icon-button {
    width: 34px;
    height: 34px;
    padding: 0;
    border-radius: 6px;
  }
</style>
