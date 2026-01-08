<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import type { FsEntry } from '../types';

  export let entry: FsEntry;
  export let level = 0;
  export let expanded: Set<string>;

  const dispatch = createEventDispatcher<{ toggle: string; select: FsEntry }>();

  const isExpanded = () => expanded.has(entry.path);

  function handleActivate() {
    if (entry.kind === 'directory') {
      dispatch('toggle', entry.path);
      return;
    }

    dispatch('select', entry);
  }
</script>

<button
  type="button"
  class="node"
  style={`padding-left: ${Math.max(0, level) * 12}px`}
  aria-expanded={entry.kind === 'directory' ? isExpanded() : undefined}
  on:click={handleActivate}
>
  <span class="toggle spacer">{entry.kind === 'directory' ? (isExpanded() ? 'v' : '>') : ''}</span>
  <span class="name" title={entry.name}>{entry.name}</span>
</button>

{#if entry.kind === 'directory' && isExpanded()}
  <div class="children">
    {#each entry.children as child (child.path)}
      <svelte:self entry={child} level={level + 1} {expanded} on:toggle on:select />
    {:else}
      <div class="empty" style={`padding-left: ${(level + 1) * 12}px`}>
        empty
      </div>
    {/each}
  </div>
{/if}

<style>
  .node {
    width: 100%;
    display: flex;
    align-items: center;
    gap: 0.35rem;
    height: 24px;
    padding: 0;
    border: 0;
    background: transparent;
    color: #cfcfcf;
    font-size: 0.85rem;
    cursor: pointer;
    user-select: none;
    text-align: left;
  }

  .node:hover {
    background: rgba(255, 255, 255, 0.04);
  }

  .toggle {
    width: 18px;
    height: 18px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    color: #9a9a9a;
    font-size: 0.75rem;
    line-height: 1;
  }

  .spacer {
    display: inline-block;
  }

  .name {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .children {
    display: flex;
    flex-direction: column;
  }

  .empty {
    height: 22px;
    display: flex;
    align-items: center;
    color: #666;
    font-size: 0.75rem;
  }
</style>
