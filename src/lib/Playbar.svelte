<script lang="ts">
  export let pos: number;
  export let length: number;
  
  function handleMove(evt: any) {
	const clientX: number = evt.type === 'touchmove' ? evt.touches[0].clientX : evt.clientX;
	const { left, right }: DOMRect = this.getBoundingClientRect();
	pos = (length * (clientX - left)) / (right - left);
  }

  function formatSeconds(sec: number) {
	return new Date(sec * 1000)
	  .toISOString()
	  .slice(14, 19);
  }
</script>

{formatSeconds(pos)}

<!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
<progress class="progress w-56" value={pos} max={length} on:mousedown={handleMove} on:touchmove|preventDefault={handleMove}></progress>

{formatSeconds(length)}
