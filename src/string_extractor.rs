use pest::iterators::{Pair, Pairs};
use crate::query_walk::Rule;

pub trait InnerStringExtractor {
    fn inner_str(self) -> Option<String>;
}

pub trait StringExtractor {
    fn next_str(&mut self) -> Option<String>;
}

impl InnerStringExtractor for Pair<'_, Rule> {
    fn inner_str(self) -> Option<String> {
        self.into_inner().next_str()
    }
}

impl StringExtractor for Pairs<'_, Rule> {
    fn next_str(&mut self) -> Option<String> {
        self.next()?
            .as_str()
            .parse().ok()
    }
}
