pub fn factors(n: u64) -> Vec<u64> {
    let mut result: Vec<u64> = Vec::new();
    let mut factor: u64 = 2;
    let mut x: u64 = n ;
    while x >= factor/2 {

    
        while x%factor == 0 {
            result.push ( factor);
            x = x / factor
        }

        if factor == 2 {
            factor +=1;
        } 
        else {
            factor +=2;
        }
    }    
    result 
    //todo!("This should calculate the prime factors of {n}")
}
