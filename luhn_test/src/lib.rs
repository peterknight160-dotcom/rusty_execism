/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let iter_c = code.chars().rev();
    let mut luhn: u32= 0;
    let mut two =1; 
     for c in iter_c {
        let digit: u32=  c.to_digit(10).unwrap();
        if digit == 9 {
            luhn += 10 -two ;  // If two=2, then 9*2 %9  is 8.  If two=1, then just want 9
        }
        else {
            luhn  += digit*two % 9 ;
        }
        two=3 - two ; 
    }
    if luhn %10 == 0 {
        return true
    }
    else {
        return false
    }
    
  //  todo!("Is the Luhn checksum for {code} valid?");
}
