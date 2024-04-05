import type { Song } from "./song.type"
import type { Artist } from "./artist.type"

export type Album = {
  cover: string;
  name: string;
  artist: Artist;
  songs: Song[];
}
