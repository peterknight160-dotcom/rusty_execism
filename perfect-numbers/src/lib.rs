#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None; // Return None for 0 as it's not a valid input for classification
    }
   
    let mut sum = 0; // Initialize sum of factors
    for i in 1..=num/2 { // Iterate from 1 to num-1 to find the factors
        if num % i == 0 { // Check if i is a factor of num
            sum += i; // Add the factor to the sum
        }
        
    }
    
    if sum < num {
        Some(Classification::Deficient) // If the sum of factors is less than num, it's classified as Deficient
    } else if sum == num {
        Some(Classification::Perfect) // If the sum of factors is equal to num, it's classified as Perfect
    } else {      
          Some(Classification::Abundant) // If the sum of factors is greater than num, it's classified as Abundant
    }
    

    //todo!("classify {num}");
}
