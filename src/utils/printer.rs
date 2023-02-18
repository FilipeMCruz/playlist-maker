use crate::query::processor::QueryType;
use crate::tag::details::TagDetails;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[derive(Default, PartialEq, Debug)]
pub enum Output {
    File(PathBuf),
    #[default]
    Terminal,
}

#[derive(Default, PartialEq, Debug)]
pub struct Printer {
    pub output: Output,
    pub print_type: QueryType,
}

impl Printer {
    fn format(&self, info: &[TagDetails]) -> String {
        let content = info
            .iter()
            .map(|tag| match self.print_type {
                QueryType::Play => tag.path.clone(),
                QueryType::Index => tag.to_string(),
            })
            .collect::<Vec<String>>()
            .join("\n");
        match self.print_type {
            QueryType::Play => content,
            QueryType::Index => format!("{}\n{}", TagDetails::headers(), content),
        }
    }

    pub fn print(&self, info: &[TagDetails]) -> Option<()> {
        match &self.output {
            Output::Terminal => println!("{}", self.format(info)),
            Output::File(out) => {
                let mut file = File::create(out).ok()?;
                writeln!(file, "{}", self.format(info)).ok()?;
            }
        }
        Some(())
    }
}

#[cfg(test)]
mod tests {
    use crate::query::processor::QueryType;
    use crate::tag::details::TagDetails;
    use crate::utils::printer::Printer;

    #[test]
    fn ensure_fn_format_works_as_expected_1() {
        let printer = Printer {
            print_type: QueryType::Play,
            ..Default::default()
        };
        let output = printer.format(default_songs().as_slice());

        assert_eq!(
            "test-data/songs/1.mp3\ntest-data/songs/2.mp3\ntest-data/songs/3.mp3",
            output
        )
    }

    #[test]
    fn ensure_fn_format_works_as_expected_2() {
        let printer = Printer {
            print_type: QueryType::Index,
            ..Default::default()
        };
        let output = printer.format(default_songs().as_slice());

        assert_eq!(
            r#""path","track","title","artist","album","album_artist","year","genre","disc"
"test-data/songs/1.mp3","1","","","Black","","","",""
"test-data/songs/2.mp3","","","","Blue","Surf","","",""
"test-data/songs/3.mp3","","","Cap","","","","","""#,
            output
        )
    }

    fn default_songs() -> Vec<TagDetails> {
        let info1 = TagDetails {
            path: "test-data/songs/1.mp3".to_string(),
            album: Some("Black".to_string()),
            track: Some("1".to_string()),
            ..Default::default()
        };
        let info2 = TagDetails {
            path: "test-data/songs/2.mp3".to_string(),
            album: Some("Blue".to_string()),
            album_artist: Some("Surf".to_string()),
            ..Default::default()
        };
        let info3 = TagDetails {
            path: "test-data/songs/3.mp3".to_string(),
            artist: Some("Cap".to_string()),
            ..Default::default()
        };
        vec![info1, info2, info3]
    }
}
