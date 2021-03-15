use regex::RegexSet;
use std::collections::HashSet;

/// Exclude configuration for freq.
/// You can ignore words based on regex patterns or fixed strings
#[derive(Clone, Debug)]
pub struct Excludes {
    pub regex: Option<RegexSet>,
    pub stopwords: HashSet<String>,
}

impl Default for Excludes {
    fn default() -> Self {
        Self {
            regex: None,
            stopwords: HashSet::new(),
        }
    }
}

impl Excludes {
    pub fn excluded(&self, input: &str) -> bool {
        let is_stopword = self.stopwords.contains(&input.to_lowercase());

        let regex_match = match &self.regex {
            Some(regex) => regex.is_match(input),
            None => false,
        };

        is_stopword || regex_match
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_exclude_regex() {
        let excludes = Excludes {
            regex: Some(RegexSet::new(&[r"bar", r"^foo$", r"^[0-9]+$", r"@example.org"]).unwrap()),
            ..Default::default()
        };

        assert_eq!(excludes.excluded(""), false);
        assert_eq!(excludes.excluded("foo"), true);
        assert_eq!(excludes.excluded("snafoo"), false);
        assert_eq!(excludes.excluded("sansibar"), true);
        assert_eq!(excludes.excluded("123"), true);
        assert_eq!(excludes.excluded("123f"), false);
        assert_eq!(excludes.excluded("mail@example.org"), true);
    }

    #[test]
    fn test_stopwords() {
        let excludes = Excludes {
            stopwords: vec![
                "bar".into(),
                "foo".into(),
                "123".into(),
                "test@example.org".into(),
            ]
            .into_iter()
            .collect(),
            ..Default::default()
        };

        assert_eq!(excludes.excluded(""), false);
        assert_eq!(excludes.excluded("foo"), true);
        assert_eq!(excludes.excluded("snafoo"), false);
        assert_eq!(excludes.excluded("bar"), true);
        assert_eq!(excludes.excluded("sansibar"), false);
        assert_eq!(excludes.excluded("123"), true);
        assert_eq!(excludes.excluded("123f"), false);
        assert_eq!(excludes.excluded("mail@example.org"), false);
        assert_eq!(excludes.excluded("test@example.org"), true);
    }
}
