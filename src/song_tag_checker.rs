use crate::song::Song;
use crate::song_metadata::SongMetadata;
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

pub struct SongTagChecker {
    metadata: MetadataType,
    tag_type: String,
    search_type: SearchType,
}

impl SongTagChecker {
    pub fn new(metadata_string: String, tag_type: String, search_type: SearchType) -> Option<Self> {
        let checker = Self {
            metadata: match search_type {
                SearchType::Regex => MetadataType {
                    regex: Regex::new(metadata_string.as_str()).unwrap(),
                    exact: "".to_string(),
                },
                SearchType::Contains | SearchType::Literal => MetadataType {
                    regex: Regex::new("").unwrap(),
                    exact: metadata_string,
                },
            },
            tag_type,
            search_type,
        };

        match checker.valid_type() {
            true => Some(checker),
            false => None,
        }
    }

    fn valid_type(&self) -> bool {
        !["beforeyear", "beforedate", "afteryear", "afterdate"]
            .contains(&self.tag_type.to_lowercase().as_str())
            || SearchType::Literal == self.search_type
    }

    fn is_match(&self, metadata_tag: &dyn SongMetadata) -> bool {
        match self.search_type {
            SearchType::Regex => self.is_regex_match(metadata_tag),
            SearchType::Literal => self.is_literal_match(metadata_tag),
            SearchType::Contains => self.is_contains_match(metadata_tag),
        }
    }

    fn get_info(&self, metadata_tag: &dyn SongMetadata) -> Option<String> {
        match self.tag_type.to_lowercase().as_str() {
            "title" => metadata_tag.title().map(|e| e.to_string()),
            "artist" => metadata_tag.artist().map(|e| e.to_string()),
            "album" => metadata_tag.album().map(|e| e.to_string()),
            "albumartist" => metadata_tag.album_artist().map(|e| e.to_string()),
            "year" | "date" | "beforeyear" | "beforedate" | "afteryear" | "afterdate" => {
                metadata_tag.year().map(|e| e.to_string())
            }
            "genre" => metadata_tag.genre().map(|e| e.to_string()),
            "disknumber" | "disc" => metadata_tag.disc().map(|e| e.to_string()),
            _ => None,
        }
    }

    fn is_regex_match(&self, metadata_tag: &dyn SongMetadata) -> bool {
        self.get_info(metadata_tag)
            .map_or(false, |info| self.metadata.regex.is_match(info.as_str()))
    }

    fn is_contains_match(&self, metadata_tag: &dyn SongMetadata) -> bool {
        self.get_info(metadata_tag)
            .map_or(false, |info| info.contains(self.metadata.exact.as_str()))
    }

    fn is_literal_match(&self, metadata_tag: &dyn SongMetadata) -> bool {
        self.get_info(metadata_tag).map_or(false, |info| {
            let metadata = self.metadata.exact.as_str();
            match self.tag_type.to_lowercase().as_str() {
                "beforeyear" | "beforedate" => info
                    .parse::<i32>()
                    .unwrap()
                    .le(&metadata.parse::<i32>().unwrap()),
                "afteryear" | "afterdate" => info
                    .parse::<i32>()
                    .unwrap()
                    .gt(&metadata.parse::<i32>().unwrap()),
                _ => info == metadata,
            }
        })
    }

    pub fn filter(&self, vec: &[Song]) -> Vec<Song> {
        vec.iter()
            .filter_map(|song| song.index())
            .filter(|song| self.is_match(song.metadata().unwrap().as_ref()))
            .collect::<Vec<Song>>()
    }
}
