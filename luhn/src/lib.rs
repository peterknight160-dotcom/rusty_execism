/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let iter_c = code.chars().rev();
    let mut luhn: u32= 0;
    let mut two =1; 
    let mut count_of_digits = 0;
     for c in iter_c { 
        // println! ( " c is {} luhn  is {} two is {}",c, luhn, two);
       
        let digit =  c.to_digit(10);
        
        if digit.is_none() {
            // Handle non space characters
            if c.is_whitespace () {
                continue
            } 
            else
            { 
                return false
            }
            
        }
        if digit.unwrap () == 9 {
          
            luhn += 9 ;
        }
        else {
            luhn  += digit.unwrap() *two % 9 ;
        }
        two=3 - two ; 
        count_of_digits += 1;
    }

    if count_of_digits == 1 // Always invalid
    { 
        return false 
    }
    return luhn %10 == 0 
      
    
    
  //  todo!("Is the Luhn checksum for {code} valid?");
}