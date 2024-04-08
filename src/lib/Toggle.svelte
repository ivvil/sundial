<script lang="ts">
  import { createEventDispatcher } from 'svelte'
  
  export let src: string;
  export let title: string;

  const dispatch = createEventDispatcher();

  let toggled = false;

  function toggle() {
    toggled = !toggled
	dispatch('toggle');
  }

  
</script>

<button on:click={toggle} class="toggle" class:toggled>
  <img src={src} alt={title} title={title}>
</button>

<style>
  .toggle {
	height: auto;
	border: none;
	width: auto;
	background-color: transparent;
	border: none;
	outline: none;
	box-shadow: none;
  }

  .toggle.toggled {
	filter: invert(91%) sepia(24%) saturate(1317%) hue-rotate(54deg) brightness(94%) contrast(86%);
  }
</style>
