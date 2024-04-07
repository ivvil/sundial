import type { Album } from "./album.type";
import type { Artist } from "./artist.type";

export type Song = {
  title: string;
  artists: Artist[];
  album: Album;
  length: number;
}
