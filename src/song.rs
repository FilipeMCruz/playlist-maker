use crate::song_metadata::{IndexDetails, TagDetails};
use id3::Tag;
use std::path::PathBuf;

#[derive(Clone, PartialEq)]
pub enum Song {
    Real(PathBuf),
    Indexed(IndexDetails),
}

impl Song {
    pub fn index(&self) -> Option<IndexDetails> {
        match self {
            Song::Real(path) => {
                let Ok(tag) = Tag::read_from_path(path) else {
                    return None;
                };
                Some(
                    TagDetails {
                        path: path.to_str()?.to_string(),
                        tag,
                    }
                    .indexed(),
                )
            }
            Song::Indexed(details) => Some(details.to_owned()),
        }
    }
}
