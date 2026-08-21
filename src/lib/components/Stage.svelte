<script lang="ts">
  import type { StagedObject } from "$lib/types";

  export let background: string | undefined = undefined;
  export let objects: StagedObject[] = [];
  export let camera = { x: 0, y: 0, scale: 1, rotation: 0 };

  const positionMap: Record<string, string> = {
    "far-left": "5%",
    "left": "15%",
    "centre-left": "30%",
    "centre": "50%",
    "centre-right": "70%",
    "right": "85%",
    "far-right": "95%",
  };
</script>

<div class="stage-viewport" style="transform: scale({camera.scale}) rotate({camera.rotation}deg) translate({camera.x}px, {camera.y}px);">
  {#if background}
    <div class="stage-layer background-layer">
      <!-- Background resource key rendered via local asset protocol/base64[cite: 5] -->
      <img src={background} alt="Background" class="stage-bg" />
    </div>
  {/if}

  {#each objects as obj (obj.id)}
    <div
      class="staged-character {obj.layer}"
      style="left: {positionMap[obj.position || 'centre'] || '50%'}; opacity: {obj.opacity}; transform: translateX(-50%) scale({obj.scale});"
    >
      <img
        src="{obj.id}-{obj.expression || 'neutral'}.webp"
        alt="{obj.id}"
        class="character-sprite"
      />
    </div>
  {/each}
</div>

<style>
  .stage-viewport {
    position: relative;
    width: 100vw;
    height: 100vh;
    overflow: hidden;
    background-color: #000;
    transition: transform 0.5s ease-out;
  }

  .stage-layer {
    position: absolute;
    inset: 0;
  }

  .stage-bg {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .staged-character {
    position: absolute;
    bottom: 0;
    transition: left 0.4s ease-out, opacity 0.3s ease-in-out;
    pointer-events: none;
  }

  .rear { z-index: 10; }
  .stage { z-index: 20; }
  .front { z-index: 30; }
  .interface { z-index: 40; }

  .character-sprite {
    height: 80vh;
    object-fit: contain;
  }
</style>
