<div class="Playing">
  <div class="Info">
    <img src={song.album.cover} alt="{song.album.name} artwork" />	
    <b class="song-name">{song.title}</b>
    <p class="artist-name">
	  {#if song.artists.length == 1}
		{song.artists[0].name}
	  {:else}
		{#each song.artists as {name}}
		  {name},
		{/each}
	  {/if}
	</p>
  </div>

  <div class="Player">
	<div class="transport">
	  <img src="/ui/player/skip-backward-large-symbolic.svg" alt="Previous song" title="Previous song">

	  <img src="/ui/player/play-large-symbolic.svg" alt="Play/Pause" title="Play/Pause">

	  <img src="/ui/player/skip-forward-large-symbolic.svg" alt="Next song" title="Next song">		
	</div>
	<div class="status">
	  <progress class="progress w-56"></progress>
	</div>
  </div>

  <div class="Controls">
	<img src="/ui/player/playlist-shuffle-symbolic.svg" alt="Shuffle" title="Shuffle">

	<img src="/ui/player/playlist-repeat-symbolic.svg" alt="Loop" title="Loop">

	<img src="/ui/player/music-queue-symbolic.svg" alt="Queue" title="Queue">

	<label for="volume"><img src="/ui/player/speakers-symbolic.svg" alt="Volume" title="Volume"></label>

	<input type="range" id="volume">
  </div>
</div>

<script lang="ts">
  import {invoke} from "@tauri-apps/api/tauri"
  import type { Song } from "../types/song.type";

  export let song: Song;
  // export let player;
</script>

<style>
  .Playing {
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
	display: grid; 
	grid-auto-columns: 1fr; 
	grid-template-columns: auto 1fr; 
	grid-template-rows: max-content max-content; 
	gap: 0px 2em; 
	grid-template-areas: 
	  "img song-name"
	  "img artist-name"; 
	& img {
	  display: inline;
	  justify-self: start; 
	  align-self: center; 
	  grid-area: img; 
	}
	& .song-name {
	  justify-self: center; 
	  align-self: center; 
	  grid-area: song-name; 
	}
	& .artist-name {
	  justify-self: center; 
	  align-self: center; 
	  grid-area: artist-name;
	}
  }

  .Player {
    grid-area: Player;
	display: grid;
	grid-template-columns: auto;
	grid-template-rows: 1fr 1fr;
	gap: 0px 0px;
	grid-template-areas:
	  "transport"
	  "status";
	& .transport {
	  grid-area: transport;
	  & img {
		display: inline;
	  }
	}

	& .status {
	  grid-area: status;
	}
  }

  .Controls {
    grid-area: Controls;
	justify-self: end;
	display: flex;
	& img {
	  display: inline;
	}
  }
</style>
