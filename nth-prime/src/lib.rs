

pub fn nth(n: u32) -> u32 {
    let mut target: u32 = n;
    let mut prime: u32 = 1;
    if target == 0 {
        return 2;
    }
    while target > 0 {
        prime +=2 ;
        if is_prime (prime) {
            target -= 1;
        }

    }   
    prime 
}

pub fn is_prime (n: u32) -> bool{
    let mut factor: u32 =2 ;
    if n% factor ==0 {
        return false
    } 
    factor = 3;
    loop {
    if factor * factor > n {
                return true
        }
        if n%factor == 0 { 
            return false
        } else {
         
              factor +=2;
           
        }
         
    }
}
