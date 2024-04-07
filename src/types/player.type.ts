import type { Playlist } from "./playlist.type";
import type { Song } from "./song.type"

export type Player = {
  playlist: Playlist
  currentSong: Song;
  shuffle: boolean;
  loop: boolean;
  volume: number;
  position: number;
  playing: boolean;
}
