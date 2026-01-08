<script lang="ts">
  import { viewer } from './lib/stores/viewer';
  import { ui } from './lib/stores/ui';
  import { settings } from './lib/stores/settings';
  import { fileTree } from './lib/stores/fileTree';
  import { pickFolderPath, pickImageFile, scanFolderForImages } from './lib/actions/open';
  import ImageViewer from './lib/components/ImageViewer.svelte';
  import TitleBar from './lib/components/TitleBar.svelte';
  import NavigationControls from './lib/components/NavigationControls.svelte';
  import ContextMenu from './lib/components/ContextMenu.svelte';
  import SettingsDialog from './lib/components/SettingsDialog.svelte';
  import Sidebar from './lib/components/Sidebar.svelte';
  
  let showWelcome = true;
  let contextMenuX = 0;
  let contextMenuY = 0;
  let showContextMenu = false;
  
  $: if ($viewer.currentImage) {
    showWelcome = false;
  }

  async function openFile() {
    try {
      const file = await pickImageFile();
      if (!file) return;
      viewer.loadImage(file);
    } catch (err) {
      console.error('Failed to open file:', err);
    }
  }

  async function openFolder() {
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

  function handleContextMenu(event: MouseEvent) {
    event.preventDefault();
    contextMenuX = event.clientX;
    contextMenuY = event.clientY;
    showContextMenu = true;
  }

  function handleKeydown(event: KeyboardEvent) {
    // Ignore if typing in an input field
    if (event.target instanceof HTMLInputElement || event.target instanceof HTMLTextAreaElement) {
      return;
    }

    switch (event.key) {
      case 'ArrowLeft':
        event.preventDefault();
        viewer.previousImage();
        break;
      case 'ArrowRight':
        event.preventDefault();
        viewer.nextImage();
        break;
      case '+':
      case '=':
        event.preventDefault();
        viewer.zoomIn();
        break;
      case '-':
      case '_':
        event.preventDefault();
        viewer.zoomOut();
        break;
      case '0':
        event.preventDefault();
        viewer.resetZoom();
        break;
      case 'f':
      case 'F':
        event.preventDefault();
        viewer.toggleFitToWindow();
        break;
    }
  }
</script>

<svelte:window on:keydown={handleKeydown} />

<div class="app-container" style="zoom: {$settings.uiScale}%">
  <TitleBar />

  <!-- Main viewer area -->
  <div class="content">
    <Sidebar />
    <div class="viewer-container" on:contextmenu={handleContextMenu} role="main">
      {#if showWelcome}
        <div class="welcome">
          <h1>nocap</h1>
          <p>Open an image or folder to get started</p>

          <div class="welcome-buttons">
            <button type="button" class="action-button" on:click={openFile}>
              <span class="button-icon">📄</span>
              <span>Open File</span>
            </button>
            <button type="button" class="action-button" on:click={openFolder}>
              <span class="button-icon">📁</span>
              <span>Open Folder</span>
            </button>
          </div>

          <p class="hint">Or use the menu (☰) or right-click anywhere</p>
        </div>
      {:else}
        <ImageViewer />
        <NavigationControls />
      {/if}
    </div>
  </div>
</div>

<ContextMenu bind:show={showContextMenu} x={contextMenuX} y={contextMenuY} />
<SettingsDialog open={$ui.settingsOpen} on:close={() => ui.closeSettings()} />

<style>
  .app-container {
    display: flex;
    flex-direction: column;
    width: 100%;
    height: 100vh;
    overflow: hidden;
  }

  .viewer-container {
    flex: 1;
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
    overflow: hidden;
    background: #0a0a0a;
  }

  .content {
    flex: 1;
    min-height: 0;
    display: flex;
    overflow: hidden;
  }

  .welcome {
    text-align: center;
    color: #888;
    user-select: none;
  }

  .welcome h1 {
    font-size: 3rem;
    font-weight: 300;
    margin-bottom: 1rem;
    color: #aaa;
  }

  .welcome p {
    font-size: 1.1rem;
    margin: 0.5rem 0;
  }

  .welcome .hint {
    font-size: 0.9rem;
    color: #666;
    margin-top: 1.5rem;
  }

  .welcome-buttons {
    display: flex;
    gap: 1rem;
    justify-content: center;
    margin: 2rem 0 1rem;
  }

  .action-button {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.5rem;
    padding: 1.5rem 2rem;
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 8px;
    color: #ddd;
    font-size: 0.95rem;
    cursor: pointer;
    transition: all 0.2s ease;
    min-width: 140px;
  }

  .action-button:hover {
    background: rgba(255, 255, 255, 0.08);
    border-color: rgba(255, 255, 255, 0.2);
    transform: translateY(-2px);
  }

  .action-button:active {
    transform: translateY(0);
    background: rgba(255, 255, 255, 0.06);
  }

  .button-icon {
    font-size: 2rem;
    line-height: 1;
  }
</style>
