use std::path::{PathBuf};

use id3::{Tag, TagLike};
use regex::Regex;

struct MetadataType {
    regex: Regex,
    exact: String,
}

#[derive(PartialEq)]
pub enum SearchType {
    Regex,
    Contains,
    Literal,
}

pub struct SongTag {
    metadata: MetadataType,
    tag_type: String,
    search_type: SearchType,
}

impl SongTag {
    pub fn new(metadata_string: String, tag_type: String, is_regex: SearchType) -> Self {
        Self {
            metadata: match is_regex {
                SearchType::Regex => MetadataType { regex: Regex::new(metadata_string.as_str()).unwrap(), exact: "".to_string() },
                SearchType::Contains | SearchType::Literal => MetadataType { regex: Regex::new("").unwrap(), exact: metadata_string }
            },
            tag_type,
            search_type: is_regex,
        }
    }

    fn check_tag(&self, metadata_tag: Tag) -> bool {
        match self.search_type {
            SearchType::Regex => self.is_regex_match(metadata_tag),
            SearchType::Literal => self.is_literal_match(metadata_tag),
            SearchType::Contains => self.is_contains_match(metadata_tag)
        }
    }

    fn get_info(&self, metadata_tag: Tag) -> Option<String> {
        match self.tag_type.to_lowercase().as_str() {
            "title" => metadata_tag.title().map(|e| e.to_string()),
            "artist" => metadata_tag.artist().map(|e| e.to_string()),
            "album" => metadata_tag.album().map(|e| e.to_string()),
            "albumartist" => metadata_tag.album_artist().map(|e| e.to_string()),
            "year" | "date" | "beforeyear" | "beforedate" | "afteryear" | "afterdate" => metadata_tag.year().map(|e| e.to_string()),
            "genre" => metadata_tag.genre().map(|e| e.to_string()),
            "disknumber" => metadata_tag.disc().map(|e| e.to_string()),
            _ => None
        }
    }

    fn is_regex_match(&self, metadata_tag: Tag) -> bool {
        self.get_info(metadata_tag)
            .map_or(false, |info_s| {
                let info = info_s.as_str();
                match self.tag_type.to_lowercase().as_str() {
                    "title" => self.metadata.regex.is_match(info),
                    "artist" => self.metadata.regex.is_match(info),
                    "album" => self.metadata.regex.is_match(info),
                    "albumartist" => self.metadata.regex.is_match(info),
                    "year" | "date" => self.metadata.regex.is_match(info),
                    "genre" => self.metadata.regex.is_match(info),
                    "disknumber" => self.metadata.regex.is_match(info),
                    _ => false
                }
            })
    }

    fn is_contains_match(&self, metadata_tag: Tag) -> bool {
        self.get_info(metadata_tag).map_or(false, |info| {
            let metadata = self.metadata.exact.as_str();
            match self.tag_type.to_lowercase().as_str() {
                "title" => info.contains(metadata),
                "artist" => info.contains(metadata),
                "album" => info.contains(metadata),
                "albumartist" => info.contains(metadata),
                "year" | "date" => info.contains(metadata),
                "genre" => info.contains(metadata),
                "disknumber" => info.contains(metadata),
                _ => false
            }
        })
    }

    fn is_literal_match(&self, metadata_tag: Tag) -> bool {
        self.get_info(metadata_tag).map_or(false, |info| {
            let metadata = self.metadata.exact.as_str();
            match self.tag_type.to_lowercase().as_str() {
                "title" => info == metadata,
                "artist" => info == metadata,
                "album" => info == metadata,
                "albumartist" => info == metadata,
                "beforeyear" | "beforedate" => info.parse::<i32>().unwrap().gt(&metadata.parse::<i32>().unwrap()),
                "afteryear" | "afterdate" => info.parse::<i32>().unwrap().le(&metadata.parse::<i32>().unwrap()),
                "year" | "date" => info == metadata,
                "genre" => info == metadata,
                "disknumber" => info == metadata,
                _ => false
            }
        })
    }

    pub fn filter_tag(&self, vec: &[PathBuf]) -> Option<Vec<PathBuf>> {
        Some(vec.iter()
            .map(|song| (song, Tag::read_from_path(song)))
            .filter(|(_, tag)| tag.is_ok() && self.check_tag(tag.as_ref().unwrap().to_owned()))
            .map(|(song, _)| song.to_owned())
            .collect::<Vec<PathBuf>>())
    }
}
