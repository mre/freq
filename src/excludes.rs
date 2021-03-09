use regex::RegexSet;

/// Exclude configuration for freq.
/// You can ignore words based on regex patterns or fixed strings
#[derive(Clone, Debug)]
pub struct Excludes {
    pub regex: Option<RegexSet>,
}

impl Default for Excludes {
    fn default() -> Self {
        Self { regex: None }
    }
}

impl Excludes {
    pub fn excluded(&self, input: &str) -> bool {
        match &self.regex {
            Some(regex) => regex.is_match(input),
            None => false,
        }
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
}
