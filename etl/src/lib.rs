use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {

    // Transform the input BTreeMap by iterating over each (score, letters) pair
    // and inserting each letter with its corresponding score into a new BTreeMap.
    // Result has btree to be letter, score. Note the input has score, letters(array)
    // Code generated for me by GitHub Copilot
    let mut result = BTreeMap::new();
    for (score, letters) in h {
        for &letter in letters {
            result.insert(letter.to_ascii_lowercase(), *score);
        }
    }
    result
    // Return the transformed BTreeMap
}
