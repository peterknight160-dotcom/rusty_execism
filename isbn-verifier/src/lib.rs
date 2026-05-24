/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let isbn_string = String::from(isbn.replace("-", ""));
   
    if isbn_string.len() != 10 {
        return false;
    }   
    let mut sum = 0;
    for (i, c) in isbn_string.chars().enumerate() {
        if !c.is_digit(10) && !(i == 9 && c == 'X') {
            return false;
        }
        let digit = if c == 'X' { 10 } else { c.to_digit(10).unwrap() };
        sum += digit * (10 - i as u32);
    }
    sum % 11 == 0
}
