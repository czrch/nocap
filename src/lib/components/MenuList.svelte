<script lang="ts">
  import { pickFolderPath, pickImageFile, scanFolderForImages } from '../actions/open';
  import { fileTree } from '../stores/fileTree';
  import { viewer } from '../stores/viewer';
  import { ui } from '../stores/ui';

  export let closeMenu: () => void;

  async function openFile() {
    closeMenu();
    try {
      const file = await pickImageFile();
      if (!file) return;
      viewer.loadImage(file);
    } catch (err) {
      console.error('Failed to open file:', err);
    }
  }

  async function openFolder() {
    closeMenu();
    try {
      const folderPath = await pickFolderPath();
      if (!folderPath) return;
      fileTree.setRoot(folderPath);
      await fileTree.refreshRoot();
      await fileTree.startWatch();
      const images = await scanFolderForImages(folderPath);
      viewer.loadFolder(images);
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
