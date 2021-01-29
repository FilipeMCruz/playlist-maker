use std::path::PathBuf;

#[derive(PartialEq, Eq)]
pub enum OperatorType {
    OR,
    AND,
}

pub struct Operator {
    pub operator_type: OperatorType,
}

impl Operator {
    pub fn filter(&self, first: &Vec<PathBuf>, second: &Vec<PathBuf>) -> Vec<PathBuf> {
        let mut filter = Vec::new();
        if self.operator_type == OperatorType::OR {
            for song in first.iter() {
                filter.push(song.to_owned())
            }
            for song in second.iter() {
                filter.push(song.to_owned())
            }
        } else {
            for song in first.iter() {
                if second.contains(song) {
                    filter.push(song.to_owned())
                }
            }
        }
        filter
    }
}
