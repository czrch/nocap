<script lang="ts">
  import { viewer } from './lib/stores/viewer';
  import ImageViewer from './lib/components/ImageViewer.svelte';
  import Toolbar from './lib/components/Toolbar.svelte';
  import NavigationControls from './lib/components/NavigationControls.svelte';
  
  let showWelcome = true;
  
  $: if ($viewer.currentImage) {
    showWelcome = false;
  }
</script>

<div class="app-container">
  <Toolbar />

  <!-- Main viewer area -->
  <div class="viewer-container">
    {#if showWelcome}
      <div class="welcome">
        <h1>nocap</h1>
        <p>Open an image or folder to get started</p>
        <p class="hint">Use File menu or keyboard shortcuts</p>
      </div>
    {:else}
      <ImageViewer />
      <NavigationControls />
    {/if}
  </div>
</div>

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
</style>
