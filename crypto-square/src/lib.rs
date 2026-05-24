pub fn encrypt(input: &str) -> String {
    // Remove all non-alphanumeric characters and convert to lowercase
    let cleaned: String = input.chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    let len = cleaned.len();
    let cols = (len as f64).sqrt().ceil() as usize;
    let mut square = vec![String::new(); cols];
    if len == 0 {
        return String::new();
    }
    let mut max_i = 0;
   for (i, c) in cleaned.chars().enumerate() {
        // If square [i] doesn't exist, create it
        if square[i % cols].is_empty() {
            square[i % cols] = String::new();

        }               
        square[i % cols].push(c); 
        max_i    = i;
    }
    // Pad the last chunk with spaces if necessary
    for i in (max_i + 1)..(cols * ((max_i / cols) + 1)) {
        square[i % cols].push(' ');
    }
    square.join(" ")
    
}
