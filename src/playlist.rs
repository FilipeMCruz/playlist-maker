use std::path::PathBuf;
use crate::song::Song;

#[derive(Clone)]
pub struct Playlist {
    pub name: String,
    pub songs: Vec<PathBuf>,
}

impl Playlist {
    pub fn filter(&self, vec: &[Song]) -> Vec<Song> {
        return vec.iter()
            .filter(|song| self.songs.contains(&song.path()))
            .map(|song| song.to_owned())
            .collect::<Vec<Song>>();
    }
}
