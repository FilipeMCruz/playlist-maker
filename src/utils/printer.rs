use crate::tag::details::TagDetails;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use crate::query::processor::QueryType;

#[derive(Default)]
pub enum Output {
    File(PathBuf),
    #[default]
    Terminal,
}

#[derive(Default)]
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
            QueryType::Index => format!("{}\n{}", TagDetails::headers(), content)
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
