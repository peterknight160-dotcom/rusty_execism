use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    // Count words in `words` and return a map of the counts.
    // Words are separated by whitespace or punctuation, and the count is case insensitive.
    // Apostrophes are allowed in words, but other punctuation is not.
    // The function returns a HashMap where the keys are words and the values are their counts.

    // An apostrophe is allowed in a word, not if it's a quote mark.
    words
        .split(|c: char| !c.is_alphanumeric() && c != '\'')
        .map(|w| w.trim_matches('\'').to_lowercase())
        .filter(|w| !w.is_empty())
        .fold(HashMap::new(), |mut acc, word| {
            *acc.entry(word).or_insert(0) += 1;
            acc
        })
}
