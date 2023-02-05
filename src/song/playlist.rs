use crate::tag::details::TagDetails;

#[derive(Clone)]
pub struct Playlist {
    pub name: String,
    pub songs: Vec<String>,
}

impl Playlist {
    pub fn filter(&self, vec: &[TagDetails]) -> Vec<TagDetails> {
        return vec
            .iter()
            .filter(|song| self.songs.contains(&song.path))
            .map(|song| song.to_owned())
            .collect::<Vec<TagDetails>>();
    }
}
