use crate::tag::checker::SearchType;
use crate::tag::details::TagDetails;

#[derive(Debug, PartialEq)]
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
            ("discnumber" | "disc", _) => Some(TagType::Disc),
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

#[cfg(test)]
mod tests {
    use crate::tag::checker::SearchType;
    use crate::tag::r#type::TagType;

    #[test]
    fn tag_type_can_be_built_as_expected_1() {
        let tag_opt = TagType::from("genre", &SearchType::Literal);
        assert!(tag_opt.is_some());
        let tag = tag_opt.unwrap();

        assert_eq!(tag, TagType::Genre)
    }

    #[test]
    fn tag_type_can_be_built_as_expected_2() {
        let tag_opt = TagType::from("albumartist", &SearchType::Regex);
        assert!(tag_opt.is_some());
        let tag = tag_opt.unwrap();

        assert_eq!(tag, TagType::AlbumArtist)
    }

    #[test]
    fn tag_type_can_be_built_as_expected_3() {
        let tag_opt = TagType::from("album", &SearchType::Contains);
        assert!(tag_opt.is_some());
        let tag = tag_opt.unwrap();

        assert_eq!(tag, TagType::Album)
    }

    #[test]
    fn tag_type_can_be_built_as_expected_4() {
        let tag_opt = TagType::from("disc", &SearchType::Contains);
        assert!(tag_opt.is_some());
        let tag = tag_opt.unwrap();

        assert_eq!(tag, TagType::Disc)
    }

    #[test]
    fn tag_type_can_be_built_as_expected_5() {
        let tag_opt = TagType::from("discnumber", &SearchType::Contains);
        assert!(tag_opt.is_some());
        let tag = tag_opt.unwrap();

        assert_eq!(tag, TagType::Disc)
    }

    #[test]
    fn tag_type_can_be_built_as_expected_6() {
        let tag_opt = TagType::from("title", &SearchType::Contains);
        assert!(tag_opt.is_some());
        let tag = tag_opt.unwrap();

        assert_eq!(tag, TagType::Title)
    }
}
