use crate::tag::details::TagDetails;

#[derive(Clone)]
pub struct Playlist {
    pub name: String,
    pub songs: Vec<String>,
}

impl Playlist {
    pub fn filter(&self, vec: &[TagDetails]) -> Vec<TagDetails> {
        return vec
            .iter()
            .filter(|song| self.songs.contains(&song.path))
            .map(|song| song.to_owned())
            .collect::<Vec<TagDetails>>();
    }
}

#[cfg(test)]
mod tests {
    use crate::playlist::Playlist;
    use crate::tag::details::TagDetails;

    #[test]
    fn empty_playlist_removes_all_tag_details() {
        let filtered = &Playlist {
            name: "test".to_string(),
            songs: vec![],
        }
        .filter(default_songs().as_slice());

        assert_eq!(0, filtered.len())
    }

    #[test]
    fn basic_playlist_removes_expected_tag_details_1() {
        let filtered = &Playlist {
            name: "test".to_string(),
            songs: vec!["test-data/songs/1.mp3".to_string()],
        }
        .filter(default_songs().as_slice());

        assert_eq!(1, filtered.len())
    }

    #[test]
    fn basic_playlist_removes_expected_tag_details_2() {
        let filtered = &Playlist {
            name: "test".to_string(),
            songs: vec![
                "test-data/songs/3.mp3".to_string(),
                "test-data/songs/4.mp3".to_string(),
            ],
        }
        .filter(default_songs().as_slice());

        assert_eq!(1, filtered.len())
    }

    #[test]
    fn basic_playlist_removes_expected_tag_details_3() {
        let filtered = &Playlist {
            name: "test".to_string(),
            songs: vec![
                "test-data/songs/3.mp3".to_string(),
                "test-data/songs/4.mp3".to_string(),
                "test-data/songs/2.mp3".to_string(),
            ],
        }
        .filter(default_songs().as_slice());

        assert_eq!(2, filtered.len())
    }

    fn default_songs() -> Vec<TagDetails> {
        let info1 = TagDetails {
            path: "test-data/songs/1.mp3".to_string(),
            ..Default::default()
        };
        let info2 = TagDetails {
            path: "test-data/songs/2.mp3".to_string(),
            ..Default::default()
        };
        let info3 = TagDetails {
            path: "test-data/songs/3.mp3".to_string(),
            ..Default::default()
        };
        vec![info1, info2, info3]
    }
}
