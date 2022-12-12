use pest::iterators::{Pair, Pairs};
use crate::query_walk::Rule;

pub trait ExtendedRulePair {
    fn inner_str(self) -> Option<String>;
}

pub trait ExtendedRulePairs {
    fn next_str(& mut self) -> Option<String>;
}

impl ExtendedRulePair for Pair<'_, Rule> {
    fn inner_str(self) -> Option<String> {
        self.into_inner()
            .next()?
            .as_str()
            .parse().ok()
    }
}

impl ExtendedRulePairs for Pairs<'_, Rule> {
    fn next_str(&mut self) -> Option<String> {
        self.next()?
            .as_str()
            .parse().ok()
    }
}
