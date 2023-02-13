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

#[cfg(test)]
mod tests {
    use crate::tag::matcher::TagMatcher;
    use regex::Regex;

    #[test]
    fn tag_matcher_works_as_expected_1() {
        let matcher = TagMatcher::Literal(String::from("aa"));

        assert!(matcher.matches("aa"));
    }

    #[test]
    fn tag_matcher_works_as_expected_2() {
        let matcher = TagMatcher::Literal(String::from("a"));

        assert!(!matcher.matches("aa"));
    }

    #[test]
    fn tag_matcher_works_as_expected_3() {
        let matcher = TagMatcher::Contains(String::from("a"));

        assert!(matcher.matches("ad"));
    }

    #[test]
    fn tag_matcher_works_as_expected_4() {
        let matcher = TagMatcher::Contains(String::from("ad"));

        assert!(!matcher.matches("a"));
    }

    #[test]
    fn tag_matcher_works_as_expected_5() {
        let matcher = TagMatcher::Contains(String::from("ad"));

        assert!(matcher.matches("ad"));
    }

    #[test]
    fn tag_matcher_works_as_expected_6() {
        let matcher = TagMatcher::AfterDate(50);

        assert!(matcher.matches("75"));
    }

    #[test]
    fn tag_matcher_works_as_expected_7() {
        let matcher = TagMatcher::AfterDate(50);

        assert!(!matcher.matches("25"));
    }

    #[test]
    fn tag_matcher_works_as_expected_8() {
        let matcher = TagMatcher::AfterDate(50);

        assert!(!matcher.matches("50"));
    }

    #[test]
    fn tag_matcher_works_as_expected_9() {
        let matcher = TagMatcher::BeforeDate(50);

        assert!(!matcher.matches("75"));
    }

    #[test]
    fn tag_matcher_works_as_expected_10() {
        let matcher = TagMatcher::BeforeDate(50);

        assert!(matcher.matches("25"));
    }

    #[test]
    fn tag_matcher_works_as_expected_11() {
        let matcher = TagMatcher::BeforeDate(50);

        assert!(matcher.matches("50"));
    }

    #[test]
    fn tag_matcher_works_as_expected_12() {
        let regex = Regex::new("^a.*");
        let matcher = TagMatcher::Regex(regex.unwrap());

        assert!(!matcher.matches("basa"));
    }

    #[test]
    fn tag_matcher_works_as_expected_13() {
        let regex = Regex::new("a.*");
        let matcher = TagMatcher::Regex(regex.unwrap());

        assert!(matcher.matches("ades"));
    }
}
