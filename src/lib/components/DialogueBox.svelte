<script lang="ts">
  import type { ChoiceOptionDto, StepEventDto } from "$lib/types";
  import { createEventDispatcher } from "svelte";

  export let event: StepEventDto | null = null;
  export let mode: "adv" | "nvl" = "adv";
  export let nvlPage: string[] = [];

  const dispatch = createEventDispatcher<{
    advance: void;
    choice: string;
  }>();

  function handleKeydown(e: KeyboardEvent) {
    if (e.code === "Space" || e.code === "Enter") {
      if (event?.type === "Narration" || event?.type === "Dialogue") {
        dispatch("advance");
      }
    }
  }
</script>

<svelte:window on:keydown={handleKeydown} />

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div 
  class="dialogue-container" 
  class:nvl-mode={mode === 'nvl'} 
  on:click={() => {
    if (event?.type === "Narration" || event?.type === "Dialogue") dispatch("advance");
  }}
>
  {#if mode === "adv"}
    <div class="adv-box">
      {#if event?.type === "Dialogue"}
        <div class="speaker-name">{event.payload.speaker_display}</div>
        <div class="dialogue-text">{event.payload.text}</div>
      {:else if event?.type === "Narration"}
        <div class="narration-text">{event.payload.text}</div>
      {:else if event?.type === "Ended"}
        <div class="end-card">
          <h2>{event.payload.title}</h2>
          <p>Result: {event.payload.result}</p>
        </div>
      {/if}
    </div>
  {:else}
    <div class="nvl-overlay">
      {#each nvlPage as paragraph}
        <p class="nvl-paragraph">{paragraph}</p>
      {/each}
      {#if event?.type === "Dialogue"}
        <p class="nvl-paragraph"><strong>{event.payload.speaker_display}:</strong> {event.payload.text}</p>
      {:else if event?.type === "Narration"}
        <p class="nvl-paragraph">{event.payload.text}</p>
      {/if}
    </div>
  {/if}

  {#if event?.type === "Choice"}
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="choice-modal" on:click|stopPropagation>
      <div class="choice-list">
        {#each event.payload.options as opt}
          <button class="choice-button" on:click={() => dispatch("choice", opt.id)}>
            {opt.label}
          </button>
        {/each}
      </div>
    </div>
  {/if}
</div>

<style>
  .dialogue-container {
    position: absolute;
    inset: 0;
    z-index: 50;
    display: flex;
    flex-direction: column;
    justify-content: flex-end;
    user-select: none;
    cursor: pointer;
  }

  .adv-box {
    margin: 0 auto 2.5rem;
    width: 85%;
    max-width: 900px;
    background: rgba(18, 18, 24, 0.88);
    border: 1px solid rgba(255, 255, 255, 0.15);
    border-radius: 8px;
    padding: 1.5rem 2rem;
    color: #f1f1f1;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4);
    min-height: 120px;
  }

  .speaker-name {
    font-weight: 700;
    font-size: 1.1rem;
    color: #86b7ff;
    margin-bottom: 0.5rem;
  }

  .dialogue-text, .narration-text {
    font-size: 1.2rem;
    line-height: 1.6;
  }

  .narration-text {
    font-style: italic;
    color: #e0e0e0;
  }

  .nvl-overlay {
    background: rgba(10, 10, 15, 0.92);
    width: 100%;
    height: 100%;
    padding: 4rem 15%;
    overflow-y: auto;
    color: #f1f1f1;
    font-size: 1.25rem;
    line-height: 1.8;
  }

  .choice-modal {
    position: absolute;
    inset: 0;
    background: rgba(0, 0, 0, 0.6);
    display: flex;
    justify-content: center;
    align-items: center;
  }

  .choice-list {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    width: 60%;
    max-width: 500px;
  }

  .choice-button {
    background: #1e2430;
    border: 1px solid #4f5d75;
    color: #ffffff;
    padding: 1rem 1.5rem;
    font-size: 1.1rem;
    border-radius: 6px;
    cursor: pointer;
    transition: background 0.2s, border-color 0.2s;
  }

  .choice-button:hover {
    background: #2d3748;
    border-color: #86b7ff;
  }
</style>