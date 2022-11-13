use std::path::PathBuf;

#[derive(Clone)]
pub struct Playlist {
    pub name: String,
    pub songs: Vec<PathBuf>,
}

impl Playlist {
    pub fn filter(&self, vec: &Vec<PathBuf>) -> Vec<PathBuf> {
        return vec.iter()
            .filter(|song| self.songs.contains(song))
            .map(|song| song.to_owned())
            .collect::<Vec<PathBuf>>();
    }
}
