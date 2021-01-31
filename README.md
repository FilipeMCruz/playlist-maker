# Playlist Maker

Allow users to create playlists using a query like:

``` none
Play((Artist("Joji") | Artist("Tom Misch")) & !InPlaylist("old_loved_songs"))
```

## Functional Requirements

Query can be build using the following tokens:
- Song tags:
    - title;
    - artist;
    - album;
    - albumartist;
    - year | date;
    - genre;
    - disknumber.
- Query objects:
    - literal song tags;
    - regex in song tags (`R_`);
    - partial song tags (`C_`);
    - m3u playlists.
- Basic lang support:
    - `and` operator (`&`);
    - `or` operator (`|`);
    - `not` operator (`!`);
    - parenthesis (`()`).

## Non-Functional Requirements

- Written in Rust so that it can be compiled to assembly and run anywhere;
- Simple Console App;
- Support id3 tags and mp3 files (more can eventually be added);
- Multithreading support;
- Support local stored songs only (more can eventually be added);

## Console Args

- `-h` / `--help` : menu with some information;
- `-t` / `--type` : local or spotify/soundcloud, etc (support local only for now, default to local)
- `-o` / `--output` : file to write the playlist to (if not specified send to stdout);
- `-q` / `--query` : query to execute;
- `-i` / `--input` : directory with songs/playlists to query from (can be repeated if needed)
- `-p` / `--playlist` : path to playlist to be used in the query
