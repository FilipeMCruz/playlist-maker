use regex::Regex;

pub enum TagMatcher {
    Regex(Regex),
    Contains(String),
    Literal(String),
    AfterDate(i32),
    BeforeDate(i32),
}

impl TagMatcher {
    pub fn matches(&self, info: &str) -> bool {
        match self {
            TagMatcher::Regex(regex) => regex.is_match(info),
            TagMatcher::Contains(string) => info.contains(string),
            TagMatcher::Literal(metadata) => info == metadata,
            TagMatcher::AfterDate(date) => info.parse::<i32>().unwrap().gt(date),
            TagMatcher::BeforeDate(date) => info.parse::<i32>().unwrap().le(date),
        }
    }
}
