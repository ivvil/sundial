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

  let songs = invoke("plugin:library|get_files");

  // songs.then((sol) => console.log(sol))

</script>

<main class="container">

  <div class="library">
    {#await songs}
	  <p>Loading data...</p>
	{:then library}
	  {#each library as song}
		<PlayablePreview
		  src={song.cover}
		  title={song.title}
		  author={song.artists}></PlayablePreview>
		{/each}
	  {/await}
	</div>

  <div class="btm-nav">
	<Player {song} />
  </div>


</main>

<style>
  .library {
	display: flex;
	flex-wrap: wrap;
	justify-content: space-evenly;
	row-gap: 25px;
  }

</style>
