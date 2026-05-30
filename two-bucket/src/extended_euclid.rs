fn extended_euclid( b1:  u8, b2: u8, goal: u8) -> Option<(i8, i8, u8)> {
    let mut s = 0;
    let mut old_s = 1;  
    let mut t = 1;
    let mut old_t = 0;  
    let mut r = b2 as i8;
    let mut old_r = b1 as i8;   
    let mut moves = 0;
    while r != 0 {
        moves += 1;
        let quotient = old_r / r;
        let temp_r = r;
        r = old_r - quotient * r;
        old_r = temp_r;
        let temp_s = s;
        s = old_s - quotient * s;
        old_s = temp_s;
        let temp_t = t;
        t = old_t - quotient * t;
        old_t = temp_t;
    }
    
    Some((old_s, old_t, moves))
}