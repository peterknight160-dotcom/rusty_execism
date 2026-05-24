pub fn rotate(input: &str, key: u8) -> String {
    // If char not in the alphabet, return it as is. Otherwise, shift it by key and wrap around the alphabet if necessary.
   input.chars()
        .map(|c| {
            match c {
                'a'..='z' => ((c as u8 - b'a' + key) % 26 + b'a') as char,
                'A'..='Z' => ((c as u8 - b'A' + key) % 26 + b'A') as char,
                _ => c,
            }
            
        })
        .collect::<String>()
}


