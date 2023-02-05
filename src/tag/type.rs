use crate::tag::checker::SearchType;
use crate::tag::details::TagDetails;

pub enum TagType {
    Title,
    Artist,
    Album,
    AlbumArtist,
    Date,
    Genre,
    Disc,
}

impl TagType {
    pub fn from(tag_type: &str, search_type: &SearchType) -> Option<Self> {
        match (tag_type, search_type) {
            ("title", _) => Some(TagType::Title),
            ("artist", _) => Some(TagType::Artist),
            ("album", _) => Some(TagType::Album),
            ("albumartist", _) => Some(TagType::AlbumArtist),
            ("date" | "year", _) => Some(TagType::Date),
            ("beforedate" | "afterdate", SearchType::Literal) => Some(TagType::Date),
            ("beforeyear" | "afteryear", SearchType::Literal) => Some(TagType::Date),
            ("genre", _) => Some(TagType::Genre),
            ("disknumber" | "disc", _) => Some(TagType::Disc),
            _ => None,
        }
    }

    pub fn collect<'a>(&'a self, metadata_tag: &'a TagDetails) -> Option<&str> {
        match self {
            TagType::Title => metadata_tag.title.as_deref(),
            TagType::Artist => metadata_tag.artist.as_deref(),
            TagType::Album => metadata_tag.album.as_deref(),
            TagType::AlbumArtist => metadata_tag.album_artist.as_deref(),
            TagType::Date => metadata_tag.year.as_deref(),
            TagType::Genre => metadata_tag.genre.as_deref(),
            TagType::Disc => metadata_tag.disc.as_deref(),
        }
    }
}
