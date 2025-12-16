<script lang="ts">
  import { viewer } from '../stores/viewer';
  import { convertFileSrc } from '@tauri-apps/api/core';
  
  let containerEl: HTMLDivElement;
  let imageEl: HTMLImageElement;
  let isDragging = false;
  let panX = 0;
  let panY = 0;
  let startX = 0;
  let startY = 0;

  $: imageSrc = $viewer.currentImage 
    ? convertFileSrc($viewer.currentImage.path)
    : '';

  $: imageStyle = $viewer.fitToWindow
    ? 'max-width: 100%; max-height: 100%; width: auto; height: auto;'
    : `transform: translate(${panX}px, ${panY}px) scale(${$viewer.zoomLevel});`;

  function handleWheel(event: WheelEvent) {
    event.preventDefault();
    
    if (event.deltaY < 0) {
      viewer.zoomIn();
    } else {
      viewer.zoomOut();
    }
    
    // Reset pan when zooming
    panX = 0;
    panY = 0;
  }

  function handleMouseDown(event: MouseEvent) {
    if ($viewer.fitToWindow) return;
    
    isDragging = true;
    startX = event.clientX - panX;
    startY = event.clientY - panY;
    
    if (containerEl) {
      containerEl.style.cursor = 'grabbing';
    }
  }

  function handleMouseMove(event: MouseEvent) {
    if (!isDragging) return;
    
    panX = event.clientX - startX;
    panY = event.clientY - startY;
  }

  function handleMouseUp() {
    isDragging = false;
    if (containerEl) {
      containerEl.style.cursor = $viewer.fitToWindow ? 'default' : 'grab';
    }
  }

  function handleImageLoad() {
    // Reset pan position when new image loads
    panX = 0;
    panY = 0;
  }

  // Reset pan when fit-to-window changes
  $: if ($viewer.fitToWindow) {
    panX = 0;
    panY = 0;
  }
</script>

<div 
  class="image-viewer"
  bind:this={containerEl}
  on:wheel={handleWheel}
  on:mousedown={handleMouseDown}
  on:mousemove={handleMouseMove}
  on:mouseup={handleMouseUp}
  on:mouseleave={handleMouseUp}
  style="cursor: {$viewer.fitToWindow ? 'default' : 'grab'};"
  role="img"
  aria-label="Image viewer"
>
  {#if imageSrc}
    <img
      bind:this={imageEl}
      src={imageSrc}
      alt={$viewer.currentImage?.filename || 'Image'}
      style={imageStyle}
      on:load={handleImageLoad}
      draggable="false"
    />
  {/if}
</div>

<style>
  .image-viewer {
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    overflow: hidden;
    user-select: none;
  }

  img {
    display: block;
    object-fit: contain;
    transition: transform 0.1s ease-out;
    transform-origin: center;
  }
</style>
