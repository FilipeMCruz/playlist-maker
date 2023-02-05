use std::path::Path;

pub trait ExtensionExtractor {
    fn has_extension(&self, extension: &str) -> bool;
    fn is_dir_or_has_extension(&self, extension: &str) -> bool;
}

impl ExtensionExtractor for Path {
    fn has_extension(&self, extension: &str) -> bool {
        self.is_file()
            && self
                .extension()
                .map_or(false, |e| e.eq_ignore_ascii_case(extension))
    }

    fn is_dir_or_has_extension(&self, extension: &str) -> bool {
        self.is_dir() || self.has_extension(extension)
    }
}

#[cfg(test)]
mod tests {
    use crate::extensions::path_matching::ExtensionExtractor;
    use std::path::Path;

    #[test]
    fn match_extension_for_file_can_match() {
        let path = Path::new("test-data/dummy.txt");
        assert!(path.has_extension("txt"));
    }

    #[test]
    fn match_extension_for_file_with_uppercase_extension_can_match() {
        let path = Path::new("test-data/dummy.JSON");
        assert!(path.has_extension("json"));
    }

    #[test]
    fn match_extension_for_file_with_no_extension_cant_match() {
        let path = Path::new("test-data/dummy");
        assert!(!path.has_extension("txt"));
    }

    #[test]
    fn match_extension_for_file_with_two_extensions_can_match() {
        let path = Path::new("test-data/dummy.conf.txt");
        assert!(path.has_extension("txt"));
    }

    #[test]
    fn match_extension_for_dir_doesnt_match() {
        let path = Path::new("test-data/dir.test");
        assert!(!path.has_extension("test"));
    }

    #[test]
    fn is_dir_or_match_extension_for_dir_always_matches() {
        let path = Path::new("test-data/dir.test");
        assert!(path.is_dir_or_has_extension("aaa"));
    }

    #[test]
    fn is_dir_or_match_extension_for_file_matches() {
        let path = Path::new("test-data/dummy.txt");
        assert!(path.is_dir_or_has_extension("txt"));
    }
}
