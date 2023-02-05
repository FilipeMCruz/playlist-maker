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
}

impl TagDetails {
    pub fn headers() -> String {
        "\"path\";\"title\";\"artist\";\"album\";\"album_artist\";\"year\";\"genre\";\"disc\""
            .to_string()
    }

    pub fn details(&self) -> String {
        let rev = [
            self.path.as_str(),
            self.title.as_deref().unwrap_or(""),
            self.artist.as_deref().unwrap_or(""),
            self.album.as_deref().unwrap_or(""),
            self.album_artist.as_deref().unwrap_or(""),
            self.year.as_deref().unwrap_or("0"),
            self.genre.as_deref().unwrap_or(""),
            self.disc.as_deref().unwrap_or("0"),
        ]
        .join("\";\"");
        format!("\"{}\"", rev)
    }
}
