# Playlist Maker

Allow users to create playlists using a query like:

``` none
Play((Artist('Joji) | Artist('Tom Misch')) & !InPlaylist('old_loved_songs'))
```

## Functional Requirements

Support for:

- Query objects:
    - song tags:
        - Artist;
        - Album;
        - Genre;
        - AlbumArtist;
        - Year;
        - Any other (if not found discard song).
    - regex in song tags:
        - Artist;
        - Album;
        - Genre;
        - AlbumArtist;
        - Year;
        - Any other (if not found discard song).
    - m3u playlists;
- Basic lang support:
    - `and` operator (`&`);
    - `or` operator (`|`);
    - `not` operator (`!`);
    - parenthesis (`()`);

## Non-Functional Requirements

- Written in Rust so that it can be compiled to assembly and run anywhere;
- Expected to be a simple Console App;
- Support only id3 tags, for now;
- Multithreaded;
- Support local stored songs only, for now;
- Decouple everything so that it is easier to extend (spotify playlists, different music tags, work as a plugin in some
  music app, etc)

## Console Args

- `-h` / `--help` : menu with some information;
- `-t` / `--type` : local or spotify/soundcloud, etc (support local only for now, default to local)
- `-o` / `--output` : file to write the playlist to (if not specified send to stdout);
- `-q` / `--query` : query to execute;
- `-i` / `--input` : directory with songs/playlists to query from (can be repeated if needed)
- `-p` / `--playlist` : path to playlist to be used in the query

## Progress

- Implemented command line parser with clap;
- Implemented grammar to validate query;
