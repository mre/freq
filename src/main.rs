use std::collections::{HashMap, HashSet};
use std::io::prelude::*;
use unicode_segmentation::UnicodeSegmentation;

struct WordStat {
    word: String,
    stat: u64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use stopwords::Stopwords;
    let stop_words_spark: HashSet<_> = stopwords::Spark::stopwords(stopwords::Language::English)
        .unwrap()
        .iter()
        .collect();
    let _stop_words_nltk: HashSet<_> = stopwords::NLTK::stopwords(stopwords::Language::English)
        .unwrap()
        .iter()
        .collect();
    let stop_words_sk: HashSet<_> = stopwords::SkLearn::stopwords(stopwords::Language::English)
        .unwrap()
        .iter()
        .collect();
    let stop_words: HashSet<_> = stop_words_spark.union(&stop_words_sk).collect();
    let mut total_counter = 0u64;
    let mut word_counter = HashMap::new();
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            for word in line.unicode_words() {
                if stop_words.contains(&&word) {
                    continue;
                }
                total_counter += 1;
                word_counter
                    .entry(word.to_string())
                    .and_modify(|e| *e += 1)
                    .or_insert(1u64);
            }
        } else {
            break;
        }
    }
    let mut sorted: Vec<WordStat> = word_counter
        .into_iter()
        .map(|(k, v)| WordStat { word: k, stat: v })
        .collect();
    sorted.sort_by_cached_key(|x| x.stat);
    for f in sorted.iter() {
        println!(
            "{} - {} - {}",
            f.stat as f32 / total_counter as f32,
            f.stat,
            f.word
        );
    }
    println!("total words = {}", total_counter);

    Ok(())
}
