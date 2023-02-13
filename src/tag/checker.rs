use crate::tag::details::TagDetails;
use crate::tag::matcher::TagMatcher;
use crate::tag::r#type::TagType;
use regex::Regex;

#[derive(PartialEq)]
pub enum SearchType {
    Regex,
    Contains,
    Literal,
}

pub struct TagChecker {
    matcher: TagMatcher,
    tag: TagType,
}

impl TagChecker {
    pub fn try_from(exp: String, tag: String, search_type: SearchType) -> Option<Self> {
        let tag_type = tag.to_lowercase();

        match TagType::try_from(tag_type.as_str(), &search_type) {
            Some(tag) => Some(Self {
                matcher: match search_type {
                    SearchType::Regex => TagMatcher::Regex(Regex::new(exp.as_str()).ok()?),
                    SearchType::Contains => TagMatcher::Contains(exp),
                    SearchType::Literal => match tag_type.as_str() {
                        "beforeyear" | "beforedate" => {
                            TagMatcher::BeforeDate(exp.parse::<i32>().ok()?)
                        }
                        "afteryear" | "afterdate" => {
                            TagMatcher::AfterDate(exp.parse::<i32>().ok()?)
                        }
                        _ => TagMatcher::Literal(exp),
                    },
                },
                tag,
            }),
            None => None,
        }
    }

    pub fn filter(&self, vec: &[TagDetails]) -> Vec<TagDetails> {
        vec.iter()
            .filter(|info| {
                self.tag
                    .collect(info)
                    .map_or(false, |info| self.matcher.matches(info))
            })
            .map(|song| song.to_owned())
            .collect::<Vec<TagDetails>>()
    }
}

#[cfg(test)]
mod tests {
    use crate::tag::checker::{SearchType, TagChecker};

    #[test]
    fn search_type_can_be_compared() {
        assert!(SearchType::Literal == SearchType::Literal);
        assert!(SearchType::Regex == SearchType::Regex);
        assert!(SearchType::Contains == SearchType::Contains);

        assert!(SearchType::Regex != SearchType::Literal);
        assert!(SearchType::Literal != SearchType::Contains);
        assert!(SearchType::Contains != SearchType::Regex);
    }

    #[test]
    fn song_tag_checker_is_valid_1() {
        let checker = TagChecker::try_from(
            String::from("drake"),
            String::from("artist"),
            SearchType::Literal,
        );
        assert!(checker.is_some());
    }

    #[test]
    fn song_tag_checker_is_valid_2() {
        let checker = TagChecker::try_from(
            String::from("drake"),
            String::from("albumartist"),
            SearchType::Literal,
        );
        assert!(checker.is_some());
    }

    #[test]
    fn song_tag_checker_is_valid_3() {
        let checker = TagChecker::try_from(
            String::from("Damage"),
            String::from("album"),
            SearchType::Contains,
        );
        assert!(checker.is_some());
    }

    #[test]
    fn song_tag_checker_is_valid_4() {
        let checker = TagChecker::try_from(
            String::from("1980"),
            String::from("year"),
            SearchType::Literal,
        );
        assert!(checker.is_some());
    }

    #[test]
    fn song_tag_checker_is_valid_5() {
        let checker = TagChecker::try_from(
            String::from("1980"),
            String::from("afterdate"),
            SearchType::Literal,
        );
        assert!(checker.is_some());
    }

    #[test]
    fn song_tag_checker_is_valid_6() {
        let checker = TagChecker::try_from(
            String::from("1980"),
            String::from("beforeyear"),
            SearchType::Literal,
        );
        assert!(checker.is_some());
    }

    #[test]
    fn song_tag_checker_is_valid_7() {
        let checker = TagChecker::try_from(
            String::from("a.*"),
            String::from("album"),
            SearchType::Regex,
        );
        assert!(checker.is_some());
    }

    #[test]
    fn song_tag_checker_is_not_valid_1() {
        let checker = TagChecker::try_from(
            String::from("1980"),
            String::from("afterdate"),
            SearchType::Contains,
        );
        assert!(checker.is_none());
    }

    #[test]
    fn song_tag_checker_is_not_valid_2() {
        let checker = TagChecker::try_from(
            String::from("1980"),
            String::from("afterdate"),
            SearchType::Regex,
        );
        assert!(checker.is_none());
    }

    #[test]
    fn song_tag_checker_is_not_valid_3() {
        let checker = TagChecker::try_from(
            String::from("1980"),
            String::from("beforedate"),
            SearchType::Regex,
        );
        assert!(checker.is_none());
    }

    #[test]
    fn song_tag_checker_is_not_valid_4() {
        let checker = TagChecker::try_from(
            String::from("a|*"),
            String::from("album"),
            SearchType::Regex,
        );
        assert!(checker.is_none());
    }

    #[test]
    fn song_tag_checker_is_not_valid_5() {
        let checker = TagChecker::try_from(
            String::from("aa"),
            String::from("beforedate"),
            SearchType::Literal,
        );
        assert!(checker.is_none());
    }

    #[test]
    fn song_tag_checker_is_not_valid_6() {
        let checker = TagChecker::try_from(
            String::from("aa"),
            String::from("afterdate"),
            SearchType::Literal,
        );
        assert!(checker.is_none());
    }
}
