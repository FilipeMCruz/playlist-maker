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

#[cfg(test)]
mod tests {
    use crate::tag::details::TagDetailsMapper;
    use id3::Tag;
    use std::path::Path;

    #[test]
    fn basic_id3_v23_tags_can_be_extracted() {
        let path = Path::new("test-data/songs/1.mp3");
        let tag = Tag::read_from_path(path);
        assert!(tag.is_ok());
        let info = tag.unwrap().to_details("test-data/songs/1.mp3");
        assert_eq!(info.path, "test-data/songs/1.mp3");
        assert_eq!(info.title.unwrap(), "Passionfruit");
        assert_eq!(info.artist.unwrap(), "Drake");
        assert_eq!(info.album.unwrap(), "More Life");
        assert_eq!(info.album_artist.unwrap(), "Drake");
        assert_eq!(info.year.unwrap(), "2017");
        assert_eq!(info.genre.unwrap(), "Rap");
        assert_eq!(info.disc.unwrap(), "1");
    }
}
