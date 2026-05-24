pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut result : Vec<String> = Vec::new(); // Initialize an empty vector to store the resulting slices

    // This stack of methods converts the input string into a vector of characters, then
    // uses the windows method to create overlapping slices of the specified length.
    // Each slice is then collected back into a String and pushed into the result vector, using
    // the closure provided to for_each.

    digits
        .chars()
        .collect::<Vec<char>>()
        .windows(len)
        .for_each(|slice| {
            result.push(slice.iter().collect ()); // Add the collected slice to the result vector
        });
    result // Return the vector of slices
}
