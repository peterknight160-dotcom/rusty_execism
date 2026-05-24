pub fn number(user_number: &str) -> Option<String> {
   // Convert to a NANP string 
    let mut nanp_string: String = user_number.chars().filter(|c| c.is_ascii_digit()).collect();
  
    if nanp_string.starts_with  ('1') {
        nanp_string.remove(0);
    }

    if nanp_string.len() !=10  {
        return None;
    } 
  
    let area_code = &nanp_string[0..1];
    let exchange_code = &nanp_string[3..4]; 
    if area_code.starts_with('0') || area_code.starts_with('1') || 
       exchange_code.starts_with('0') || exchange_code.starts_with('1') {
        return None;
    }
    
    Some(nanp_string)
}
