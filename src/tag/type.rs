use crate::tag::checker::SearchType;
use crate::tag::details::TagDetails;

#[derive(Debug, PartialEq)]
pub enum TagType {
    Path,
    Title,
    Artist,
    Album,
    AlbumArtist,
    Date,
    Genre,
    Disc,
    Track,
}

impl TagType {
    pub fn try_from(tag: &str, search: &SearchType) -> Option<Self> {
        match (tag, search) {
            ("path", _) => Some(TagType::Path),
            ("title", _) => Some(TagType::Title),
            ("artist", _) => Some(TagType::Artist),
            ("album", _) => Some(TagType::Album),
            ("albumartist", _) => Some(TagType::AlbumArtist),
            ("date" | "year", _) => Some(TagType::Date),
            ("beforedate" | "afterdate", SearchType::Literal) => Some(TagType::Date),
            ("beforeyear" | "afteryear", SearchType::Literal) => Some(TagType::Date),
            ("genre", _) => Some(TagType::Genre),
            ("discnumber" | "disc", _) => Some(TagType::Disc),
            ("track" | "tracknumber", _) => Some(TagType::Track),
            _ => None,
        }
    }

    pub fn collect<'a>(&'a self, tag: &'a TagDetails) -> Option<&str> {
        match self {
            TagType::Path => Some(tag.path.as_str()),
            TagType::Title => tag.title.as_deref(),
            TagType::Artist => tag.artist.as_deref(),
            TagType::Album => tag.album.as_deref(),
            TagType::AlbumArtist => tag.album_artist.as_deref(),
            TagType::Date => tag.year.as_deref(),
            TagType::Genre => tag.genre.as_deref(),
            TagType::Disc => tag.disc.as_deref(),
            TagType::Track => tag.track.as_deref(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tag::checker::SearchType;
    use crate::tag::r#type::TagType;

    #[test]
    fn tag_type_can_be_built_as_expected_1() {
        let tag_opt = TagType::try_from("genre", &SearchType::Literal);
        assert!(tag_opt.is_some());
        let tag = tag_opt.unwrap();

        assert_eq!(tag, TagType::Genre)
    }

    #[test]
    fn tag_type_can_be_built_as_expected_2() {
        let tag_opt = TagType::try_from("albumartist", &SearchType::Regex);
        assert!(tag_opt.is_some());
        let tag = tag_opt.unwrap();

        assert_eq!(tag, TagType::AlbumArtist)
    }

    #[test]
    fn tag_type_can_be_built_as_expected_3() {
        let tag_opt = TagType::try_from("album", &SearchType::Contains);
        assert!(tag_opt.is_some());
        let tag = tag_opt.unwrap();

        assert_eq!(tag, TagType::Album)
    }

    #[test]
    fn tag_type_can_be_built_as_expected_4() {
        let tag_opt = TagType::try_from("disc", &SearchType::Contains);
        assert!(tag_opt.is_some());
        let tag = tag_opt.unwrap();

        assert_eq!(tag, TagType::Disc)
    }

    #[test]
    fn tag_type_can_be_built_as_expected_5() {
        let tag_opt = TagType::try_from("discnumber", &SearchType::Contains);
        assert!(tag_opt.is_some());
        let tag = tag_opt.unwrap();

        assert_eq!(tag, TagType::Disc)
    }

    #[test]
    fn tag_type_can_be_built_as_expected_6() {
        let tag_opt = TagType::try_from("title", &SearchType::Contains);
        assert!(tag_opt.is_some());
        let tag = tag_opt.unwrap();

        assert_eq!(tag, TagType::Title)
    }

    #[test]
    fn tag_type_can_be_built_as_expected_7() {
        let tag_opt = TagType::try_from("track", &SearchType::Literal);
        assert!(tag_opt.is_some());
        let tag = tag_opt.unwrap();

        assert_eq!(tag, TagType::Track)
    }

    #[test]
    fn tag_type_can_be_built_as_expected_8() {
        let tag_opt = TagType::try_from("tracknumber", &SearchType::Regex);
        assert!(tag_opt.is_some());
        let tag = tag_opt.unwrap();

        assert_eq!(tag, TagType::Track)
    }

    #[test]
    fn tag_type_can_be_built_as_expected_9() {
        let tag_opt = TagType::try_from("path", &SearchType::Regex);
        assert!(tag_opt.is_some());
        let tag = tag_opt.unwrap();

        assert_eq!(tag, TagType::Path)
    }
}
