use id3::{Tag, TagLike};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;

#[derive(Clone, PartialEq, Default, Debug, Deserialize)]
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
        String::from(
            r#""path";"track";"title";"artist";"album";"album_artist";"year";"genre";"disc""#,
        )
    }
}

impl Display for TagDetails {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
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
        write!(f, "\"{}\"", rev)
    }
}

impl TryFrom<&PathBuf> for TagDetails {
    type Error = id3::Error;

    fn try_from(value: &PathBuf) -> Result<Self, Self::Error> {
        Tag::read_from_path(value).map(|tag| {
            TagDetails {
                path: value.to_string_lossy().to_string(),
                title: tag.title().map(|e| e.to_string()),
                artist: tag.artist().map(|e| e.to_string()),
                album: tag.album().map(|e| e.to_string()),
                album_artist: tag.album_artist().map(|e| e.to_string()),
                genre: tag.genre().map(|e| e.to_string()),
                disc: tag.disc().map(|e| e.to_string()),
                track: tag.track().map(|e| e.to_string()),
                // id3 crate expects the year frame to be TYER for id3v2.4 tags instead of TDRC.
                // This if is here to ensure an uniform facade for both tag versions, at least
                // according to https://en.wikipedia.org/wiki/ID3#ID3v2_frame_specification and Kid3
                year: match tag.year() {
                    Some(year) => Some(year),
                    None => tag.date_recorded().map(|t| t.year),
                }
                .map(|e| e.to_string()),
                ..Default::default()
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::tag::details::TagDetails;
    use std::path::PathBuf;

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
            ..Default::default()
        };
        assert_eq!(
            r#""test-data/songs/1.mp3";"6";"Passionfruit";"Drake";"More Life";"Drake";"2017";"Rap";"1""#,
            info.to_string()
        );
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
            ..Default::default()
        };
        assert_eq!(
            r#""test-data/songs/1.mp3";"0";"";"";"";"";"0";"";"0""#,
            info.to_string()
        )
    }

    #[test]
    fn basic_id3_v23_tags_can_be_extracted() {
        let path = PathBuf::from("test-data/songs/id3v2.3.mp3");
        let info = TagDetails::try_from(&path).unwrap();
        assert_eq!(info.path, "test-data/songs/id3v2.3.mp3");
        assert_eq!(info.title.unwrap(), "Passionfruit");
        assert_eq!(info.artist.unwrap(), "Drake");
        assert_eq!(info.album.unwrap(), "More Life");
        assert_eq!(info.album_artist.unwrap(), "Drake");
        assert_eq!(info.year.unwrap(), "2017");
        assert_eq!(info.genre.unwrap(), "Rap");
        assert_eq!(info.track.unwrap(), "2");
        assert_eq!(info.disc.unwrap(), "1");
    }

    #[test]
    fn basic_id3_v24_tags_can_be_extracted() {
        let path = PathBuf::from("test-data/songs/id3v2.4.mp3");
        let info = TagDetails::try_from(&path).unwrap();
        assert_eq!(info.path, "test-data/songs/id3v2.4.mp3");
        assert_eq!(info.title.unwrap(), "Passionfruit");
        assert_eq!(info.artist.unwrap(), "Drake");
        assert_eq!(info.album.unwrap(), "More Life");
        assert_eq!(info.album_artist.unwrap(), "Drake");
        assert_eq!(info.year.unwrap(), "2017");
        assert_eq!(info.genre.unwrap(), "Rap");
        assert_eq!(info.track.unwrap(), "1");
        assert_eq!(info.disc.unwrap(), "1");
    }

    #[test]
    fn missing_local_song_cant_extract_info() {
        let local = TagDetails::try_from(&PathBuf::from("test-data/songs/none.mp3"));
        assert!(local.is_err());
    }
}
