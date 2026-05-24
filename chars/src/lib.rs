/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let iter_c = code.chars().rev();
    let mut luhn: u32= 0;
    let mut two =1; 
     for c in iter_c { 
     //   println! ( " c is {} luhn  is {} two is {}",c, luhn, two);
       
        let digit =  c.to_digit(10);
        if digit.is_none() {
            continue;
        }
        if digit.unwrap () == 9 {
            luhn += 10 -two ;  // If two=2, then 9*2 %9  is 8.  If two=1, then just want 9
        }
        else {
            luhn  += digit.unwrap() *two % 9 ;
        }
        two=3 - two ; 
    }

    println! ( "Final luhn is {}", luhn);
    if luhn %10 == 0 {
        return true
    }
    else {
        return false
    }
    
  //  todo!("Is the Luhn checksum for {code} valid?");
}