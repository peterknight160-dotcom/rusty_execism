use std::collections::HashMap;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {

    // In this method, have used a HashMap to store the partial results.
    // 
    let mut multiples = HashMap::new() ;
    for f in factors {
        if *f == 0 {
            continue ; 
        }  
    
        let mut num = *f ;
        while num < limit {
           
            let _ = multiples.insert(num, "1") ; // We are only really interested in the keys 
            num += *f ;
        }
    }
 
   let mut sum = 0; 
   for (key, _ ) in multiples.iter() {
       sum +=key ;
   }
   sum 
}
