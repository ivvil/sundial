<div class="container">
  <div class="Info">
    <img src={song.album.cover} alt="{song.album.name} artwork" />
    <b>{song.title}</b>
    <p>
	  {#if song.artists.length == 1}
		{song.artists[0].name}
	  {:else}
		{#each song.artists as {name}}
		  {name},
		{/each}
	  {/if}
	</p>
  </div>
</div>

<script lang="ts">
  import {invoke} from "@tauri-apps/api/tauri"
  import type { Song } from "../types/song.type";

  export let song: Song;
  // export let player;
</script>

<style>
  .container {
    display: grid;
    grid-template-columns: 1fr 1fr 1fr;
    grid-template-rows: auto;
    gap: 0px 0px;
    grid-auto-flow: row;
    grid-template-areas: "Info Player Controls";
  }

  .Info {
	margin-left: 2%;
    grid-area: Info;
	justify-self: start; 
	& img {
	  display: inline;
	}
  }

  .Player {
    grid-area: Player;
  }

  .Controls {
    grid-area: Controls;
  }
</style>
