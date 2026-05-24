// Return the Hamming distance between the strings,
// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        return None;
    }
    let mut distance = 0;
    // All the hard work is done by the zip iterator - this pairs up characters into the tuple (c1, c2), which we can then compare.
    // Like the teeth of a zipper
    //  So pairing s1[i] with s2[i] is done for us!
    for (c1, c2) in s1.chars().zip(s2.chars()) {
        if c1 != c2 {
            distance += 1;
        }
    }
    Some(distance)
}