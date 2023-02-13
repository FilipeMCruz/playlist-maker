use crate::tag::details::TagDetails;
use std::path::PathBuf;

#[derive(Clone, PartialEq)]
pub enum SongInfo {
    Local(PathBuf),
    Indexed(Box<TagDetails>),
}

impl SongInfo {
    pub fn extract_info(&self) -> Option<TagDetails> {
        match self {
            SongInfo::Local(path) => TagDetails::try_from(path).ok(),
            SongInfo::Indexed(details) => Some(*details.to_owned()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::song::info::SongInfo;
    use crate::tag::details::TagDetails;

    #[test]
    fn local_song_can_extract_info() {
        let local = SongInfo::Local(PathBuf::from("test-data/songs/id3v2.3.mp3")).extract_info();
        assert!(local.is_some());
        assert_eq!(
            r#""test-data/songs/id3v2.3.mp3";"2";"Passionfruit";"Drake";"More Life";"Drake";"2017";"Rap";"1""#,
            local.unwrap().to_string()
        );
    }

    #[test]
    fn missing_local_song_cant_extract_info() {
        let local = SongInfo::Local(PathBuf::from("test-data/songs/none.mp3")).extract_info();
        assert!(local.is_none());
    }

    #[test]
    fn indeed_song_can_extract_info() {
        let local = SongInfo::Indexed(Box::new(TagDetails {
            path: "test-data/songs/1.mp3".to_string(),
            ..Default::default()
        })).extract_info();
        assert!(local.is_some());
        assert_eq!(r#""test-data/songs/1.mp3";"0";"";"";"";"";"0";"";"0""#, local.unwrap().to_string());
    }
}
