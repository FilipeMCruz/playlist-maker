use crate::song_metadata::{IndexDetails, SongMetadata, TagDetails};
use id3::Tag;
use std::path::PathBuf;

#[derive(Clone, PartialEq)]
pub enum Song {
    Real(PathBuf),
    Indexed(IndexDetails),
}

impl Song {
    pub fn path(&self) -> PathBuf {
        match self {
            Song::Real(path) => path.to_owned(),
            Song::Indexed(details) => PathBuf::from(&details.path),
        }
    }

    pub fn metadata(&self) -> Option<Box<dyn SongMetadata>> {
        match self {
            Song::Real(path) => {
                let Ok(tag) = Tag::read_from_path(path) else {
                    return None;
                };
                Some(Box::new(TagDetails {
                    path: path.to_str()?.to_string(),
                    tag,
                }))
            }
            Song::Indexed(details) => Some(Box::new(details.to_owned())),
        }
    }

    pub fn index(&self) -> Option<Song> {
        match self {
            Song::Real(path) => {
                let Ok(tag) = Tag::read_from_path(path) else {
                    return None;
                };
                Some(Song::Indexed(
                    TagDetails {
                        path: path.to_str()?.to_string(),
                        tag,
                    }
                    .indexed(),
                ))
            }
            Song::Indexed(details) => Some(Song::Indexed(details.to_owned())),
        }
    }
}
