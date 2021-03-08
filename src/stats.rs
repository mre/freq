use serde::Serialize;
use std::{
    collections::HashMap,
    fmt::{self, Display},
};

pub struct WordStat {
    pub word: String,
    pub stat: usize,
}
/// The freq statistics
#[derive(Debug, Serialize)]
pub struct Stats {
    pub total: usize,
    pub excluded: usize,
    pub occurrences: HashMap<String, usize>,
}

impl Stats {
    pub fn new() -> Self {
        Stats {
            total: 0,
            excluded: 0,
            occurrences: HashMap::new(),
        }
    }
}

impl Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut sorted: Vec<WordStat> = self
            .occurrences
            .iter()
            .map(|(k, v)| WordStat {
                word: k.to_string(),
                stat: *v,
            })
            .collect();
        sorted.sort_by_cached_key(|x| x.stat);
        for word in sorted.iter() {
            writeln!(
                f,
                "{:.3} - {} - {}",
                word.stat as f32 / self.total as f32,
                word.stat,
                word.word
            )?;
        }
        writeln!(f)
    }
}
