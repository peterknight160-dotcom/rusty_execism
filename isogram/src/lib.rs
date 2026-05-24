use std::collections::HashSet ;

pub fn check(candidate: &str) -> bool {
        
    let mut seen = HashSet::new();    
    for c in candidate.chars() {
        if c.is_alphabetic() {
            let lower_c = c.to_ascii_lowercase();
            if seen.contains(&lower_c) {
                return false; // Duplicate found
            }
            seen.insert(lower_c);
        }
    }
    true // No duplicates found
   
}
