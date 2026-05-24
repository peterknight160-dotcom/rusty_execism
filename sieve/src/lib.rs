// Use a vector, where v[n] is whether n is prime.

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut sieve = Vec::new();
    if upper_bound < 2 {
        return Vec::new();
    }
    let bound: usize = upper_bound.try_into().unwrap() ;
    sieve.push ("Zero") ;
    sieve.push ("One");
    for _ in 2..=bound {
        sieve.push( "Prime");
    }
    //println!("sieve is {:?}", sieve);
    let mut j :usize = 2;
    loop {
        if sieve[j] == "Prime" && j <= bound {
            let mut k = j *2;
            while k <= bound  {
                sieve[k] = "Compound";
                k += j;
            }
        }
        if j == 2 {
            j = 3
        } else {
            j += 2;
        }
        if j > bound  {
            break;
        }
    }
    //println!( "sieve is now  {:?}", sieve);
    let mut result: Vec<u64> = Vec::new();
    for i in 2..=bound {
        if sieve[i ] == "Prime" {
            result.push(i.try_into().unwrap());
        }
    }
    result
}
