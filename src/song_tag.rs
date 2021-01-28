use std::path::{Path, PathBuf};

use id3::Tag;

struct SongTag {
    metadata: String,
    tag_type: String,
    is_regex: bool,
}

impl SongTag {
    fn check_tag(&self, path: &Path) -> bool {
        let metadata_tag = Tag::read_from_path(path).unwrap();
        if !self.is_regex {
            return match self.tag_type.to_lowercase().as_ref() {
                "artist" => metadata_tag.artist().unwrap_or("") == self.metadata,
                "album" => metadata_tag.album().unwrap_or("") == self.metadata,
                "albumartist" => metadata_tag.album_artist().unwrap_or("") == self.metadata,
                "year" => metadata_tag.year().unwrap_or(0).to_string() == self.metadata,
                "genre" => metadata_tag.genre().unwrap_or("") == self.metadata,
                "disknumber" => metadata_tag.disc().unwrap_or(0).to_string() == self.metadata,
                _ => false
            };
        } else {
            unimplemented!("No Regex support for now")
        }
    }

    pub fn filter_tag(&self, vec: &Vec<PathBuf>) -> Vec<PathBuf> {
        let mut filter = Vec::new();
        for song in vec {
            if self.check_tag(song.as_path()) {
                filter.push(song.to_owned());
            }
        }
        filter
    }
}
