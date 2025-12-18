<script lang="ts">
  import { metadata } from '../stores/metadata';

  type InfoRow = {
    label: string;
    value: string;
  };

  $: exif = $metadata.exif;

  $: rows = ((): InfoRow[] => {
    if (!exif) return [];

    const candidates: Array<InfoRow | null> = [
      exif.date_taken ? { label: 'Date taken', value: exif.date_taken } : null,
      exif.camera_make ? { label: 'Camera make', value: exif.camera_make } : null,
      exif.camera_model ? { label: 'Camera model', value: exif.camera_model } : null,
      exif.lens_model ? { label: 'Lens model', value: exif.lens_model } : null,
      exif.author ? { label: 'Author', value: exif.author } : null,
      exif.description ? { label: 'Description', value: exif.description } : null,
      exif.software ? { label: 'Software', value: exif.software } : null,
      exif.copyright ? { label: 'Copyright', value: exif.copyright } : null,
      exif.orientation ? { label: 'Orientation', value: exif.orientation } : null,
    ];

    return candidates.filter((row): row is InfoRow => row !== null);
  })();
</script>

<aside class="inspector" aria-label="Inspector" on:contextmenu|stopPropagation>
  <div class="header">
    <div class="title">Inspector</div>
    <div class="subtitle">EXIF</div>
  </div>

  <div class="content">
    {#if $metadata.loading}
      <div class="state">Loading…</div>
    {:else if $metadata.error}
      <div class="state error">{$metadata.error}</div>
    {:else if rows.length === 0}
      <div class="state">No EXIF data found.</div>
    {:else}
      <dl class="rows">
        {#each rows as row (row.label)}
          <div class="row">
            <dt class="label">{row.label}</dt>
            <dd class="value">{row.value}</dd>
          </div>
        {/each}
      </dl>
    {/if}
  </div>
</aside>

<style>
  .inspector {
    width: 320px;
    flex: 0 0 auto;
    height: 100%;
    background: #0f0f0f;
    border-left: 1px solid #262626;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .header {
    padding: 0.85rem 0.85rem 0.75rem;
    border-bottom: 1px solid #262626;
  }

  .title {
    font-size: 0.9rem;
    color: #e6e6e6;
    user-select: none;
  }

  .subtitle {
    margin-top: 0.2rem;
    font-size: 0.75rem;
    color: #8a8a8a;
    user-select: none;
  }

  .content {
    flex: 1;
    overflow: auto;
    padding: 0.85rem;
  }

  .state {
    color: #8a8a8a;
    font-size: 0.85rem;
    user-select: none;
  }

  .state.error {
    color: #ff8a80;
    white-space: pre-wrap;
    word-break: break-word;
  }

  .rows {
    display: flex;
    flex-direction: column;
    gap: 0.65rem;
    margin: 0;
  }

  .row {
    display: grid;
    grid-template-columns: 1fr;
    gap: 0.2rem;
  }

  .label {
    color: #a8a8a8;
    font-size: 0.75rem;
    text-transform: uppercase;
    letter-spacing: 0.04em;
    user-select: none;
  }

  .value {
    color: #e2e2e2;
    font-size: 0.85rem;
    margin: 0;
    word-break: break-word;
  }
</style>
