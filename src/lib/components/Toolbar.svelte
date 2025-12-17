<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { message, open } from '@tauri-apps/plugin-dialog';
  import { viewer } from '../stores/viewer';
  import type { ImageFile } from '../types';

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
  <div class="toolbar-section">
    <button on:click={openFile} title="Open Image (Ctrl+O)">
      📁 Open File
    </button>
    <button on:click={openFolder} title="Open Folder (Ctrl+Shift+O)">
      📂 Open Folder
    </button>
  </div>

  <div class="toolbar-section filename">
    {#if $viewer.currentImage}
      <span class="filename-text" title={$viewer.currentImage.path}>
        {$viewer.currentImage.filename}
      </span>
    {:else}
      <span class="filename-text no-image">No image loaded</span>
    {/if}
  </div>

  <div class="toolbar-section">
    <button
      on:click={zoomOut}
      disabled={!$viewer.currentImage}
      title="Zoom Out (-)"
    >
      🔍−
    </button>
    <button
      on:click={resetZoom}
      disabled={!$viewer.currentImage}
      title="Reset Zoom (0)"
    >
      {Math.round($viewer.zoomLevel * 100)}%
    </button>
    <button
      on:click={zoomIn}
      disabled={!$viewer.currentImage}
      title="Zoom In (+)"
    >
      🔍+
    </button>
    <button
      on:click={toggleFit}
      disabled={!$viewer.currentImage}
      class:active={$viewer.fitToWindow}
      title="Fit to Window (F)"
    >
      {$viewer.fitToWindow ? "⊡" : "⊞"}
    </button>
  </div>
</div>

<style>
  .toolbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
    background: #1a1a1a;
    border-bottom: 1px solid #333;
    padding: 0.5rem 1rem;
    flex-shrink: 0;
  }

  .toolbar-section {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .toolbar-section.filename {
    flex: 1;
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

  button.active {
    background: #3a5a8a;
    border-color: #4a6a9a;
  }

  button.active:hover {
    background: #4a6a9a;
  }
</style>
