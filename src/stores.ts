import { writable, get } from "svelte/store";

import type { Player } from "./types/player.type";
import type { Song } from "./types/song.type";

const initialPlayer: Player = {
	playlist: {
		songs: [],
	},
	currentSong: {
		title: "",
		artists: [],
		length: 0,
		album: {
			cover: "",
			name: "",
			artist: {
				name: "",
				albums: [],
			},
			songs: [],
		},
	},
	shuffle: false,
	loop: false,
	volume: 100,
	position: 0,
	playing: false
};

const playerStore = writable(initialPlayer);

function createPlayer() {
	return {
		subscribe: playerStore.subscribe,
		set: playerStore.set,
		addPlaylistSong: (song: Song) => {
			playerStore.update(player => {
				player.playlist.songs.push(song);
				return player;
			});
		},
	};
}


export const player = createPlayer();
