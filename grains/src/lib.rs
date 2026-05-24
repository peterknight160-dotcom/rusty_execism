pub fn square(s: u32) -> u64 {
    if s <1 || s> 64 {
        panic! (" Dicey input ");
    }
    2_u64.pow(s-1)
    
}

pub fn total() -> u64 {  
   // Result is just the largest number that can be handled by a u64 type, so just use that const
   u64::MAX 
}
