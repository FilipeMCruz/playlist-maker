# Playlist Maker

_Playlist Maker_ is a fast and simple console application that allows users to create playlists using a query like:

``` none
Play((AlbumArtist("Joji") | C_Artist("Tom Misch")) & !InPlaylist("old_loved_songs"))
```

## Query Features

Query can be build using the following tokens:

- Song tags (any case):
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

## Command-line options

```
USAGE:
    playlist-maker [OPTIONS] --input <INPUT>... --query <QUERY>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --input <INPUT>...          Directory with songs to query from (can be repeated if needed)
    -o, --output <OUTPUT>           File to write the playlist to (if not specified send to stdout)
    -p, --playlist <PLAYLIST>...    Path to playlist to be used in the query (can be repeated if needed)
    -q, --query <QUERY>             Query to execute
    -t, --type <TYPE>               Local or spotify/soundcloud, etc (support local only for now, default to local)
```

### Installation

If `rust` and `cargo` are installed in the machine run:

``` sh
git pull https://github.com/FilipeMCruz/playlist-maker pl-maker
cd pl-maker
cargo build --release
```

The only machine where this application was tested was archlinux running 5.10.11-arch1-1 kernel and rust 1.49 version.

### Future work

- Test the application;
- Simplify and optimize `playlist-maker`;
- Use github actions to test and build the app;
- Use github releases to publish major versions of `playlist-maker`;
- Publish `playlist-maker` and `playlist-maker-bin` in AUR (archlinux user repositories);
- Study how hard it is to do this for spotify or soundcloud songs/playlists;
- Add zsh autocomplete with options.
