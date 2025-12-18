<script lang="ts">
  import { viewer } from '../stores/viewer';

  $: hasImages = $viewer.imageList.length > 0;
  $: counter = hasImages 
    ? `${$viewer.currentIndex + 1} / ${$viewer.imageList.length}`
    : '0 / 0';

  function handlePrevious() {
    viewer.previousImage();
  }

  function handleNext() {
    viewer.nextImage();
  }
</script>

{#if hasImages}
  <div class="navigation-controls">
    <button 
      on:click={handlePrevious} 
      disabled={!hasImages}
      title="Previous Image (←)"
      class="nav-button"
    >
      ◄
    </button>
    
    <span class="counter" title="Current image / Total images">
      {counter}
    </span>
    
    <button 
      on:click={handleNext} 
      disabled={!hasImages}
      title="Next Image (→)"
      class="nav-button"
    >
      ►
    </button>
  </div>
{/if}

<style>
  .navigation-controls {
    position: absolute;
    bottom: 1.5rem;
    left: 50%;
    transform: translateX(-50%);
    display: flex;
    align-items: center;
    gap: 1rem;
    background: rgba(26, 26, 26, 0.9);
    border: 1px solid #444;
    border-radius: 8px;
    padding: 0.75rem 1.5rem;
    backdrop-filter: blur(10px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.5);
    z-index: 10;
  }

  .nav-button {
    padding: 0.5rem 1rem;
    font-size: 1.2rem;
    min-width: 50px;
  }

  .counter {
    font-size: 0.95rem;
    color: #ddd;
    font-variant-numeric: tabular-nums;
    min-width: 80px;
    text-align: center;
    user-select: none;
  }
</style>
