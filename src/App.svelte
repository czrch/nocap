<script lang="ts">
  import { viewer } from './lib/stores/viewer';
  
  // Placeholder for components - will be added in Phase 4
  let showWelcome = true;
  
  $: if ($viewer.currentImage) {
    showWelcome = false;
  }
</script>

<div class="app-container">
  <!-- Toolbar will be added in Phase 4.9 -->
  <div class="toolbar-placeholder">
    <div class="toolbar-content">
      <span>nocap</span>
      <span>Minimal Image Viewer</span>
    </div>
  </div>

  <!-- Main viewer area -->
  <div class="viewer-container">
    {#if showWelcome}
      <div class="welcome">
        <h1>nocap</h1>
        <p>Open an image or folder to get started</p>
        <p class="hint">Use File menu or keyboard shortcuts</p>
      </div>
    {:else}
      <!-- ImageViewer component will be added in Phase 4.8 -->
      <div class="image-placeholder">
        Image viewer coming soon...
      </div>
    {/if}
  </div>

  <!-- Navigation controls will be added in Phase 4.10 -->
</div>

<style>
  .app-container {
    display: flex;
    flex-direction: column;
    width: 100%;
    height: 100vh;
    overflow: hidden;
  }

  .toolbar-placeholder {
    background: #1a1a1a;
    border-bottom: 1px solid #333;
    padding: 0.75rem 1rem;
    flex-shrink: 0;
  }

  .toolbar-content {
    display: flex;
    justify-content: space-between;
    align-items: center;
    font-size: 0.9rem;
    color: #aaa;
  }

  .viewer-container {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    overflow: hidden;
    background: #0a0a0a;
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

  .image-placeholder {
    color: #666;
  }
</style>
