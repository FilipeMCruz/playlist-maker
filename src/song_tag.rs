use std::path::{Path, PathBuf};

use id3::Tag;
use regex::Regex;

struct MetadataType {
    regex: Regex,
    exact: String,
}

#[derive(PartialEq)]
pub enum SearchType {
    REGEX,
    CONTAINS,
    LITERAL,
}

pub struct SongTag {
    metadata: MetadataType,
    tag_type: String,
    is_regex: SearchType,
}

impl SongTag {
    pub fn new(metadata_string: String, tag_type: String, is_regex: SearchType) -> Self {
        let metadata: MetadataType;
        if is_regex == SearchType::REGEX {
            metadata = MetadataType { regex: Regex::new(metadata_string.as_str()).unwrap(), exact: "".to_string() };
        } else {
            metadata = MetadataType { regex: Regex::new("").unwrap(), exact: metadata_string };
        }

        Self {
            metadata,
            tag_type,
            is_regex,
        }
    }
    fn check_tag(&self, path: &Path) -> bool {
        let metadata_tag_result = Tag::read_from_path(path);
        if metadata_tag_result.is_ok() {
            let metadata_tag = metadata_tag_result.unwrap();
            return match self.is_regex {
                SearchType::LITERAL => self.is_literal_match(metadata_tag),
                SearchType::CONTAINS => self.is_contains_match(metadata_tag),
                SearchType::REGEX => self.is_regex_match(metadata_tag)
            };
        } else {
            false
        }
    }

    fn is_regex_match(&self, metadata_tag: Tag) -> bool {
        match self.tag_type.to_lowercase().as_ref() {
            "title" => self.metadata.regex.is_match(metadata_tag.title().unwrap_or("")),
            "artist" => self.metadata.regex.is_match(metadata_tag.artist().unwrap_or("")),
            "album" => self.metadata.regex.is_match(metadata_tag.album().unwrap_or("")),
            "albumartist" => self.metadata.regex.is_match(metadata_tag.album_artist().unwrap_or("")),
            "year" | "date" => self.metadata.regex.is_match(metadata_tag.year().unwrap_or(0).to_string().as_str()),
            "genre" => self.metadata.regex.is_match(metadata_tag.genre().unwrap_or("")),
            "disknumber" => self.metadata.regex.is_match(metadata_tag.disc().unwrap_or(0).to_string().as_str()),
            _ => false
        }
    }

    fn is_contains_match(&self, metadata_tag: Tag) -> bool {
        let metadata = self.metadata.exact.as_str();
        match self.tag_type.to_lowercase().as_ref() {
            "title" => metadata_tag.title().unwrap_or("").contains(metadata),
            "artist" => metadata_tag.artist().unwrap_or("").contains(metadata),
            "album" => metadata_tag.album().unwrap_or("").contains(metadata),
            "albumartist" => metadata_tag.album_artist().unwrap_or("").contains(metadata),
            "year" | "date" => metadata_tag.year().unwrap_or(0).to_string().contains(metadata),
            "genre" => metadata_tag.genre().unwrap_or("").contains(metadata),
            "disknumber" => metadata_tag.disc().unwrap_or(0).to_string().contains(metadata),
            _ => false
        }
    }

    fn is_literal_match(&self, metadata_tag: Tag) -> bool {
        let metadata = self.metadata.exact.as_str();
        match self.tag_type.to_lowercase().as_ref() {
            "title" => metadata_tag.title().unwrap_or("") == metadata,
            "artist" => metadata_tag.artist().unwrap_or("") == metadata,
            "album" => metadata_tag.album().unwrap_or("") == metadata,
            "albumartist" => metadata_tag.album_artist().unwrap_or("") == metadata,
            "year" | "date" => metadata_tag.year().unwrap_or(0).to_string() == metadata,
            "genre" => metadata_tag.genre().unwrap_or("") == metadata,
            "disknumber" => metadata_tag.disc().unwrap_or(0).to_string() == metadata,
            _ => false
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
