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
  - beforeyear (only for literal tags);
  - afteryear (only for literal tags);
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

The only machine where this application was properly tested was archlinux running 5.10.13-arch1-1 kernel and rust 1.49 version.

#### Direct via Cargo

If `rust` and `cargo` are installed in the machine run:

``` sh
git pull https://github.com/FilipeMCruz/playlist-maker pl-maker
cd pl-maker
cargo build --release
```

#### Arch Linux via AUR

Install [package](https://aur.archlinux.org/packages/playlist-maker-rs) by running:

``` sh
paru -S playlist-maker-rs
```

OR (for the binary version)

``` sh
paru -S playlist-maker-rs-bin
```

OR (for the latest commit version)

``` sh
paru -S playlist-maker-rs-git
```

### Future work

- Test the application;
- Simplify and optimize `playlist-maker`;
- Study how hard it is to do this for spotify or soundcloud songs/playlists;
