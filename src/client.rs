use anyhow::Result;
use derive_builder::Builder;
use regex::RegexSet;
use std::collections::HashSet;
use std::io::Read;

use crate::{excludes::Excludes, stats::Stats};

#[derive(Debug)]
pub struct Client {
    pub stats: Stats,
    excludes: Excludes,
}

/// A word frequency analyzer
#[derive(Builder, Debug)]
#[builder(build_fn(skip))]
#[builder(setter(into))]
#[builder(name = "ClientBuilder")]
pub struct ClientBuilderInternal {
    /// Exclude links matching this set of regular expressions
    excludes: Option<RegexSet>,

    /// Stopwords that should be ignored
    stopwords: HashSet<String>,
}

impl ClientBuilder {
    fn build_excludes(&mut self) -> Excludes {
        Excludes {
            regex: self.excludes.clone().unwrap_or_default(),
            stopwords: self.stopwords.clone().unwrap_or_default(),
        }
    }

    /// The build method instantiates the client.
    pub fn build(&mut self) -> Result<Client> {
        let excludes = self.build_excludes();
        let stats = Stats::new();
        Ok(Client { excludes, stats })
    }
}

impl Client {
    pub async fn count<T: Read>(&self, _input: T) -> Result<Stats> {
        todo!()
    }

    pub fn update<T: AsRef<str>>(&mut self, line: T) {
        for word in line.as_ref().split_whitespace() {
            if self.excludes.excluded(&word) {
                self.stats.excluded += 1;
                continue;
            }
            self.stats.total += 1;
            self.stats
                .occurrences
                .entry(word.to_string())
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }
    }
}

/// A convenience function for quick word-counting.
/// This is the most simple counter and avoids having to create a freq client manually.
/// For more complex scenarios, look into using the `ClientBuilder` instead.
pub async fn count<T: Read>(input: T) -> Result<Stats> {
    let client = ClientBuilder::default().build()?;
    Ok(client.count(input).await?)
}

/*#[cfg(test)]
mod test {
    use super::*;
    use maplit::hashmap;

    #[test]
    fn test_basic_input() {
        let input = "apple banana lychee apple";
        let mut client = ClientBuilder::default().build().unwrap();
        client.update(input);
        let expected = hashmap! {
            "apple".to_string() => 2 as usize,
            "banana".to_string() => 1 as usize,
            "lychee".to_string() => 1 as usize,
        };
        assert_eq!(client.stats.occurrences, expected);
    }
}*/
