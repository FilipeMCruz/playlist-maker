# Playlist Maker

_Playlist Maker_ is a fast and simple console application that allows users to create playlists using a query like:

```none
Play((AlbumArtist("Joji") | C_Artist("Tom Misch")) & !InPlaylist("old_loved_songs"))
```

The program extracts information from Id3v2.3 tags and verifies if they match the query issued.  

## Query Features

Query can be build using the following tokens:

- Main options:
  - Play (creates a playlist);
  - Index (creates an index, csv with song details, of all matching songs to speed up following queries).
- Song tags (any case):
  - title;
  - artist;
  - album;
  - albumartist;
  - year | date;
  - beforeyear (only for literal tags);
  - afteryear (only for literal tags);
  - genre;
  - disknumber | disc.
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

## Query Examples

```none
Index(Afteryear("1000"))
```

Creates an index with all music, I'll assume.

```none
Play((AlbumArtist("Joji") | C_Artist("Tom Misch")) & !InPlaylist("old_loved_songs"))
```

Creates a playlist where all songs have the album artist _Joji_ or the artist contains the string _Tom Misch_ and aren't in the _old_loved_songs_ playlist.

## Command-line options

```
Create playlists using a query language

Usage: playlist-maker [OPTIONS] --query <QUERY>

Options:
  -i, --input <INPUT>        Directory with songs or file with indexed songs to query from (can be repeated if needed)
  -o, --output <OUTPUT>      File to write the query results to (if not specified send to stdout)
  -p, --playlist <PLAYLIST>  Path to m3u playlist to be used in the query (can be repeated if needed)
  -q, --query <QUERY>        Query to execute
  -h, --help                 Print help information
  -V, --version              Print version information
```

### Installation

The only OS where this application was properly tested was archlinux.

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

### Problems

- For some reason the id3 crate in use couldn't read id3v2.4 tags, only id3v2.3.

### Future work

- Test the application;
- Study how hard it is to do this for spotify or soundcloud songs/playlists.
