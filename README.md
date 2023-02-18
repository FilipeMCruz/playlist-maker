# Playlist Maker

_Playlist Maker_ is a fast and simple console application that allows users to create playlists using a query like:

```none
Play((AlbumArtist("Joji") | C_Artist("Tom Misch")) & !InPlaylist("old_loved_songs"))
```

The program extracts information from Id3 tags and verifies if they match the query issued.

## Query Features

The query can be build using the following tokens:

- Main options:
  - Play (creates a playlist);
  - Index (creates an index, csv with song details, of all matching songs to speed up following queries).
- Song tag frames (any case):
  - path; 
  - title;
  - artist;
  - album;
  - albumartist;
  - year | date;
  - beforeyear | beforedate (only for literal tags);
  - afteryear | afterdate (only for literal tags);
  - genre;
  - track | tracknumber;
  - discnumber | disc.
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
Index(Afteryear("100"))
```

Creates an index with all music, assuming you don't have any song made before the year 100.
This index can then be used instead of the normal folder with songs. 

```none
Play((AlbumArtist("Joji") | C_Artist("Tom Misch")) & !InPlaylist("old_loved_songs"))
```

Creates a playlist where all songs have the album artist _Joji_ or the artist contains the string _Tom Misch_ and aren't
in the _old_loved_songs_ playlist.

## Command-line options

``` none
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

## Installation

The only OS where this application was properly tested was archlinux.

### Direct via Cargo

If `rust` and `cargo` are installed in the machine run:

``` sh
git pull https://github.com/FilipeMCruz/playlist-maker pl-maker
cd pl-maker
cargo build --release
```

### Arch Linux via AUR

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

## Integration

I use this script to directly play the songs collected by pl-mker:

```sh
# Invoke example: <script_name> 'C_Artist("Drake")'
mpc clear
# command cut is there to normalise the path. For some reason my mpd only accepts paths starting inside $MUSIC
pl-mker --query "Play($1)" -i $MUSIC/index | cut -d '/' -f 5- | mpc add

mpc play
```

## Tips

I usually index my entire music folder with:

```none
pl-mker --query 'Index(C_Artist("a") | !C_Artist("a"))' -i $MUSIC -o $MUSIC/index
```

And then use the index to create playlists faster:

```none
pl-mker --query 'Play(<your query>)' -i $MUSIC/index
```

## Small Caveats

- The id3 crate in use reads the id3v2.4 "year" tag as a TYER frame instead of a TDRC frame, as i was expecting.

## Future work

- Document the application;
