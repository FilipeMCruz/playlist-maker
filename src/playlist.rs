use std::path::PathBuf;

#[derive(Clone)]
pub struct Playlist {
    pub name: String,
    pub songs: Vec<PathBuf>,
}

impl Playlist {
    pub fn filter(&self, vec: &Vec<PathBuf>) -> Vec<PathBuf> {
        let mut filter = Vec::new();
        for song in vec {
            if self.songs.contains(song) {
                filter.push(song.to_owned());
            }
        }
        filter
    }
}
