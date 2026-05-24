pub fn encode(key: &str, s: &str) -> Option<String> {
    // convert key to a vector of u8 values representing the shift for each character
 
    // Check if the key contains only lowercase letters and return None if it contains any invalid characters
    if key.is_empty()||  key.chars().any(|c| !c.is_ascii_lowercase()) {
        return None;
    }
    let mut key_rotate = key.chars().cycle(); // create an infinite iterator that cycles through the characters of the key
 
    let encoded: String = s
        .chars()
        .map(|c| {
            let shift = key_rotate.next().unwrap() as u8 - b'a';
            (((c as u8 - b'a' + shift) % 26) + b'a') as char
        })
        .collect();
    Some(encoded)
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    // Reverse the encoding process by applying the inverse shift for each character in the encoded string
 
 
    if key.is_empty() || key.chars().any(|c| !c.is_ascii_lowercase()) {
        return None;
    }
    let mut key_rotate = key.chars().cycle(); // create an infinite iterator that cycles through the characters of the key

    let decoded: String = s
        .chars()
        .map(|c| {
            let shift = key_rotate.next().unwrap() as u8 - b'a';
            (((c as u8 - b'a' + 26 - shift) % 26) + b'a') as char
        })
        .collect();

    Some(decoded)
}

pub fn encode_random(s: &str) -> (String, String) {
    // Generate a random key of at least 100 characters, consisting of lowercase letters, and encode the input string using that key

    let key: String = (0..100)
        .map(|_| (b'a' + rand::random_range(0..26)) as char)
        .collect();
    let encoded = encode(&key, s).unwrap();
    (key, encoded)
}
