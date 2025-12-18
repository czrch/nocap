<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { message, open } from '@tauri-apps/plugin-dialog';
  import { viewer } from '../stores/viewer';
  import { ui } from '../stores/ui';
  import type { ImageFile } from '../types';

  export let closeMenu: () => void;

  async function openFile() {
    closeMenu();
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
    closeMenu();
    try {
      const selected = await open({
        directory: true,
        multiple: false,
      });

      if (selected && typeof selected === 'string') {
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
    closeMenu();
    viewer.zoomIn();
  }

  function zoomOut() {
    closeMenu();
    viewer.zoomOut();
  }

  function resetZoom() {
    closeMenu();
    viewer.resetZoom();
  }

  function toggleFit() {
    closeMenu();
    viewer.toggleFitToWindow();
  }

  function openSettings() {
    closeMenu();
    ui.openSettings();
  }
</script>

<div class="menu-list" role="menu">
  <button
    type="button"
    class="menu-item"
    role="menuitem"
    on:click={openFile}
    title="Open Image (Ctrl+O)"
  >
    Open File…
  </button>
  <button
    type="button"
    class="menu-item"
    role="menuitem"
    on:click={openFolder}
    title="Open Folder (Ctrl+Shift+O)"
  >
    Open Folder…
  </button>

  <div class="menu-separator" role="separator"></div>

  <button
    type="button"
    class="menu-item"
    role="menuitem"
    on:click={zoomIn}
    disabled={!$viewer.currentImage}
    title="Zoom In (+)"
  >
    Zoom In
  </button>
  <button
    type="button"
    class="menu-item"
    role="menuitem"
    on:click={zoomOut}
    disabled={!$viewer.currentImage}
    title="Zoom Out (-)"
  >
    Zoom Out
  </button>
  <button
    type="button"
    class="menu-item"
    role="menuitem"
    on:click={resetZoom}
    disabled={!$viewer.currentImage}
    title="Reset Zoom (0)"
  >
    Reset Zoom
  </button>
  <button
    type="button"
    class="menu-item"
    role="menuitem"
    on:click={toggleFit}
    disabled={!$viewer.currentImage}
    title="Fit to Window (F)"
  >
    {$viewer.fitToWindow ? 'Disable Fit to Window' : 'Fit to Window'}
  </button>

  <div class="menu-separator" role="separator"></div>

  <button
    type="button"
    class="menu-item"
    role="menuitem"
    on:click={openSettings}
    title="Settings"
  >
    Settings…
  </button>
</div>

<style>
  .menu-list {
    display: flex;
    flex-direction: column;
    min-width: 180px;
  }

  .menu-item {
    width: 100%;
    text-align: left;
    border: 0;
    border-radius: 4px;
    padding: 0.6rem 0.8rem;
    background: transparent;
    color: #ddd;
    font-size: 0.85rem;
    cursor: pointer;
    transition: background 0.1s ease;
  }

  .menu-item:hover:not(:disabled) {
    background: #2a2a2a;
  }

  .menu-item:active:not(:disabled) {
    background: #242424;
  }

  .menu-item:disabled {
    color: #555;
    cursor: not-allowed;
  }

  .menu-separator {
    height: 1px;
    background: #333;
    margin: 0.3rem 0;
  }
</style>
