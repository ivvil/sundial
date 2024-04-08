<div class="Playing">
  <div class="Info">
    <img src={song.album.cover} alt="{song.album.name} artwork" />	
    <b class="song-name">{song.title}</b>
    <p class="artist-name">
	  {#each song.artists as {name}, i}
		{name}
	  {#if i == song.artists.length}
		,
	  {/if}
	{/each}

	</p>
  </div>

  <div class="Player">
	<div class="transport">
	  <img src="/ui/player/skip-backward-large-symbolic.svg" alt="Previous song" title="Previous song">

	  <img src="/ui/player/play-large-symbolic.svg" alt="Play/Pause" title="Play/Pause">

	  <img src="/ui/player/skip-forward-large-symbolic.svg" alt="Next song" title="Next song">		
	</div>
	
	<div class="status">

	  {formatSeconds($player.position)}
	  
	  <progress class="progress w-56" value={$player.position} max={length}></progress>

	  {formatSeconds($player.currentSong.length)}
	</div>
  </div>

  <div class="Controls">
	<Toggle
	  on:toggle={() => $player.shuffle = !$player.shuffle}
	  src="/ui/player/playlist-shuffle-symbolic.svg"
	  title="Shuffle">
	</Toggle>
	
	<Toggle
	  on:toggle={() => $player.loop = !$player.loop}
	  src="/ui/player/playlist-repeat-symbolic.svg"
	  title="Loop">
	</Toggle>

	<img src="/ui/player/music-queue-symbolic.svg" alt="Queue" title="Queue">

	<img src="/ui/player/speakers-symbolic.svg" alt="Volume" title="Volume">

	<input type="range" min="0" max="100" bind:value={$player.volume}>
  </div>
</div>

<script lang="ts">
  import { player } from "../stores";
  import Toggle from './Toggle.svelte'
  import type { Song } from "../types/song.type";

  export let song: Song;

  function formatSeconds(sec: number) {
	return new Date(sec * 1000)
	  .toISOString()
	  .slice(14, 19);
  }
  // export let player;
</script>

<style>
  .Playing {
	margin: 1%;
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
	  display: flex;
	  justify-content: space-around;
	  margin-left: 30%;
	  margin-right: 30%;
	  & img {
		display: inline;
	  }
	}

	& .status {
	  grid-area: status;
	  display: flex;
	  justify-content: space-around;
	  align-items: center;
	  & progress {
		width: 70%;
	  }
	}
  }

  .Controls {
    grid-area: Controls;
	justify-self: end;
	display: flex;
	gap: 4%;
	& img {
	  display: inline;
	}
  }

  button {
	background-color: transparent;
	border: none;
	outline: none;
	box-shadow: none;
  }
</style>
