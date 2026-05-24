fn main() {
    //let mut i: u64 ;
    for i in 2..1000 { 
        if is_prime (i) { 
            println!("{}",i);
        }
    }
}

fn is_prime (n: u64) -> bool{
    let mut factor: u64 =2 ;
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
