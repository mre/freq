use anyhow::{anyhow, bail, Context, Result};
use derive_builder::Builder;
use regex::{Regex, RegexSet};
use std::{convert::TryInto, io::Read};

use crate::{excludes::Excludes, stats::Stats};

#[derive(Debug, Clone)]
pub struct Client {
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
}

impl ClientBuilder {
    fn build_excludes(&mut self) -> Excludes {
        Excludes {
            regex: self.excludes.clone().unwrap_or_default(),
        }
    }

    /// The build method instantiates the client.
    pub fn build(&mut self) -> Result<Client> {
        let excludes = self.build_excludes();

        Ok(Client { excludes })
    }
}

impl Client {
    pub async fn count<T: Read>(&self, input: T) -> Result<Stats> {
        todo!();
        // for word in input {
        //     if self.excludes.excluded(&word) {
        //         continue
        //     }
        // }
    }
}

/// A convenience function for quick word-counting.
/// This is the most simple counter and avoids having to create a freq client manually.
/// For more complex scenarios, look into using the `ClientBuilder` instead.
pub async fn count<T: Read>(input: T) -> Result<Stats> {
    let client = ClientBuilder::default().build()?;
    Ok(client.count(input).await?)
}

#[cfg(test)]
mod test {
    use super::*;
    use maplit::hashmap;

    // How much wood would a woodchuck chuck if a woodchuck could chuck wood? He
    // would chuck, he would, as much as he could, and chuck as much wood as a
    // woodchuck would if a woodchuck could chuck wood.

    #[tokio::test]
    async fn test_basic_input() {
        let input = "apple banana lychee apple";
        let stats = ClientBuilder::default()
            .build()
            .unwrap()
            .count(input)
            .await
            .unwrap();
        let expected = hashmap! {
            "apple" => 2,
            "banana" => 1,
            "lychee" => 1,
        };
        assert!(stats.occurrences, expected);
    }
}
