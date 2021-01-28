use std::path::PathBuf;

struct Playlist {
    songs: Vec<PathBuf>
}

impl Playlist {
    fn filter(&self, vec: &Vec<PathBuf>) -> Vec<PathBuf> {
        let mut filter = Vec::new();
        for song in vec {
            if self.songs.contains(song) {
                filter.push(song.to_owned());
            }
        }
        filter
    }
}
