use std::path::Path;

pub trait ExtensionExtractor {
    fn match_extension(&self, extension: &str) -> bool;
    fn match_extension_or_dir(&self, extension: &str) -> bool;
}

impl ExtensionExtractor for Path {
    fn match_extension(&self, extension: &str) -> bool {
        self.is_file() && self.extension().map_or(false, |e| e.eq_ignore_ascii_case(extension))
    }

    fn match_extension_or_dir(&self, extension: &str) -> bool {
        self.is_dir() || self.match_extension(extension)
    }
}
