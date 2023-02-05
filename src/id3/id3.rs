use crate::tag::details::{TagDetails, TagDetailsMapper};
use id3::{Tag, TagLike};

impl TagDetailsMapper for Tag {
    fn to_details(&self, path: &str) -> TagDetails {
        TagDetails {
            path: path.to_string(),
            title: self.title().map(|e| e.to_string()),
            artist: self.artist().map(|e| e.to_string()),
            album: self.album().map(|e| e.to_string()),
            album_artist: self.album_artist().map(|e| e.to_string()),
            year: self.year().map(|e| e.to_string()),
            genre: self.genre().map(|e| e.to_string()),
            disc: self.disc().map(|e| e.to_string()),
        }
    }
}
