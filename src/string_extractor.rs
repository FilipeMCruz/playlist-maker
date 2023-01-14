use crate::query_walk::Rule;
use pest::iterators::{Pair, Pairs};

pub trait InnerStringExtractor {
    fn inner_str(self) -> Option<String>;
}

pub trait StringExtractor {
    fn next_str(&mut self) -> Option<String>;
}

pub trait RuleExtractor {
    fn inner_rule(self) -> Option<Rule>;
}

impl InnerStringExtractor for Pair<'_, Rule> {
    fn inner_str(self) -> Option<String> {
        self.into_inner().next_str()
    }
}

impl StringExtractor for Pairs<'_, Rule> {
    fn next_str(&mut self) -> Option<String> {
        self.next()?.as_str().parse().ok()
    }
}

impl RuleExtractor for Pair<'_, Rule> {
    fn inner_rule(self) -> Option<Rule> {
        Some(self.into_inner().next()?.as_rule())
    }
}
