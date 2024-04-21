<script lang="ts">
  import Greet from './lib/Greet.svelte'
  import type { Song } from "./types/song.type";
  import type { Artist } from './types/artist.type';
  import type { Album } from './types/album.type';
  import Player from './lib/Player.svelte';
  import PlayablePreview from './lib/PlayablePreview.svelte';
  import { invoke } from '@tauri-apps/api';

  let album: Album = { cover: "/svelte.svg", name: "Album", artist: { name: "Artist", albums: [] }, songs: [] };
  let artist: Artist = { name: "Artist", albums: [ album ] };
  let song: Song = { title: "Song", artists: [ artist ], album: album, length: 1500};

  console.log(invoke("plugin:awesome|get_library"))
</script>

<main class="container">

  <PlayablePreview
	src="/svelte.svg"
	title="Test"
	author="Yo"></PlayablePreview>


  <div class="btm-nav">
	<Player {song} />
  </div>

</main>

<style>
  .logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
  }

  .logo.svelte:hover {
    filter: drop-shadow(0 0 2em #ff3e00);
  }

</style>
