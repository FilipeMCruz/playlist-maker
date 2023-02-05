pub trait TagDetailsMapper {
    fn to_details(&self, path: &str) -> TagDetails;
}

#[derive(Clone, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TagDetails {
    pub path: String,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub album_artist: Option<String>,
    pub year: Option<String>,
    pub genre: Option<String>,
    pub disc: Option<String>,
    pub track: Option<String>,
}

impl TagDetails {
    pub fn headers() -> String {
        String::from(r#""path";"track";"title";"artist";"album";"album_artist";"year";"genre";"disc""#)
    }

    pub fn details(&self) -> String {
        let rev = [
            self.path.as_str(),
            self.track.as_deref().unwrap_or("0"),
            self.title.as_deref().unwrap_or(""),
            self.artist.as_deref().unwrap_or(""),
            self.album.as_deref().unwrap_or(""),
            self.album_artist.as_deref().unwrap_or(""),
            self.year.as_deref().unwrap_or("0"),
            self.genre.as_deref().unwrap_or(""),
            self.disc.as_deref().unwrap_or("0"),
        ]
            .join(r#"";""#);
        format!("\"{}\"", rev)
    }
}


#[cfg(test)]
mod tests {
    use crate::tag::details::TagDetails;

    #[test]
    fn tag_details_prints_headers_correctly() {
        assert_eq!("\"path\";\"track\";\"title\";\"artist\";\"album\";\"album_artist\";\"year\";\"genre\";\"disc\"", TagDetails::headers());
    }

    #[test]
    fn tag_details_prints_info_correctly_when_present() {
        let info = TagDetails {
            path: "test-data/songs/1.mp3".to_string(),
            title: Some(String::from("Passionfruit")),
            artist: Some(String::from("Drake")),
            album: Some(String::from("More Life")),
            album_artist: Some(String::from("Drake")),
            year: Some(String::from("2017")),
            genre: Some(String::from("Rap")),
            disc: Some(String::from("1")),
            track: Some(String::from("6")),
        };
        assert_eq!(r#""test-data/songs/1.mp3";"6";"Passionfruit";"Drake";"More Life";"Drake";"2017";"Rap";"1""#, info.details());
        assert_eq!(info.path, "test-data/songs/1.mp3");
        assert_eq!(info.title.unwrap(), "Passionfruit");
        assert_eq!(info.artist.unwrap(), "Drake");
        assert_eq!(info.album.unwrap(), "More Life");
        assert_eq!(info.album_artist.unwrap(), "Drake");
        assert_eq!(info.year.unwrap(), "2017");
        assert_eq!(info.genre.unwrap(), "Rap");
        assert_eq!(info.disc.unwrap(), "1");
        assert_eq!(info.track.unwrap(), "6");
    }

    #[test]
    fn tag_details_prints_info_correctly_when_missing() {
        let info = TagDetails {
            path: "test-data/songs/1.mp3".to_string(),
            title: None,
            artist: None,
            album: None,
            album_artist: None,
            year: None,
            genre: None,
            disc: None,
            track: None
        };
        assert_eq!(r#""test-data/songs/1.mp3";"0";"";"";"";"";"0";"";"0""#, info.details())
    }
}
