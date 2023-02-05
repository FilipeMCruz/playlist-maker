use crate::tag::details::{TagDetails, TagDetailsMapper};
use id3::Tag;
use std::borrow::Borrow;
use std::path::PathBuf;

#[derive(Clone, PartialEq)]
pub enum SongInfo {
    Local(PathBuf),
    Indexed(TagDetails),
}

impl SongInfo {
    pub fn extract_info(&self) -> Option<TagDetails> {
        match self {
            SongInfo::Local(path) => Tag::read_from_path(path)
                .map(|tag| tag.to_details(path.to_string_lossy().borrow()))
                .ok(),
            SongInfo::Indexed(details) => Some(details.to_owned()),
        }
    }
}
