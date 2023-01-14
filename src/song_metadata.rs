use id3::{Tag, TagLike};

pub trait SongMetadata {
    fn title(&self) -> Option<&str>;
    fn artist(&self) -> Option<&str>;
    fn album(&self) -> Option<&str>;
    fn album_artist(&self) -> Option<&str>;
    fn year(&self) -> Option<i32>;
    fn genre(&self) -> Option<&str>;
    fn disc(&self) -> Option<u32>;
    fn details(&self) -> String;
}

#[derive(Clone, PartialEq)]
pub struct TagDetails {
    pub tag: Tag,
    pub path: String,
}

impl TagDetails {
    pub fn indexed(&self) -> IndexDetails {
        IndexDetails {
            path: self.path.to_string(),
            title: self.tag.title().map(|e| e.to_string()),
            artist: self.tag.artist().map(|e| e.to_string()),
            album: self.tag.album().map(|e| e.to_string()),
            album_artist: self.tag.album_artist().map(|e| e.to_string()),
            year: self.tag.year(),
            genre: self.tag.genre().map(|e| e.to_string()),
            disc: self.tag.disc(),
        }
    }
}

impl SongMetadata for TagDetails {
    fn title(&self) -> Option<&str> {
        self.tag.title()
    }

    fn artist(&self) -> Option<&str> {
        self.tag.artist()
    }

    fn album(&self) -> Option<&str> {
        self.tag.album()
    }

    fn album_artist(&self) -> Option<&str> {
        self.tag.album_artist()
    }

    fn year(&self) -> Option<i32> {
        self.tag.year()
    }

    fn genre(&self) -> Option<&str> {
        self.tag.genre()
    }

    fn disc(&self) -> Option<u32> {
        self.tag.disc()
    }

    fn details(&self) -> String {
        self.indexed().details()
    }
}

#[derive(Clone, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct IndexDetails {
    pub path: String,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub album_artist: Option<String>,
    pub year: Option<i32>,
    pub genre: Option<String>,
    pub disc: Option<u32>,
}

impl IndexDetails {
    pub fn headers() -> String {
        "\"path\";\"title\";\"artist\";\"album\";\"album_artist\";\"year\";\"genre\";\"disc\""
            .to_string()
    }
}

impl SongMetadata for IndexDetails {
    fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    fn artist(&self) -> Option<&str> {
        self.artist.as_deref()
    }

    fn album(&self) -> Option<&str> {
        self.album.as_deref()
    }

    fn album_artist(&self) -> Option<&str> {
        self.album_artist.as_deref()
    }

    fn year(&self) -> Option<i32> {
        self.year
    }

    fn genre(&self) -> Option<&str> {
        self.genre.as_deref()
    }

    fn disc(&self) -> Option<u32> {
        self.disc
    }

    fn details(&self) -> String {
        let rev = [
            self.path.as_str(),
            self.title().unwrap_or(""),
            self.artist().unwrap_or(""),
            self.album().unwrap_or(""),
            self.album_artist().unwrap_or(""),
            self.year().unwrap_or(0).to_string().as_str(),
            self.genre().unwrap_or(""),
            self.disc().unwrap_or(0).to_string().as_str(),
        ]
        .join("\";\"");
        format!("\"{}\"", rev)
    }
}
