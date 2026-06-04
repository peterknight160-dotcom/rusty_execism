// Implements the add trait for Decimal, allowing us to add two Decimal numbers together.

use crate::Decimal;

impl std::ops::Add for Decimal {
    type Output = Decimal;

    fn add(self, other: Decimal) -> Decimal {
        // For now, we will just return a new Decimal with the same value as self.
        // We will implement the actual addition logic later.
       // Use function usigned_add to add the two Decimal numbers together, and return the result.
       // Deal with self and other both being the same sign,  
        if self.signbit == other.signbit {
            let mut result = add_unsigned(&self, &other);
        result.signbit = self.signbit; // The result will have the same sign as self, since both numbers have the same sign
        return result;
         } 
         else {
            // If the signs are different, we need to subtract the smaller number from the larger number, and give the result the same sign as the larger number.
            let result = sub_unsigned(&self, &other);
            // If result is zero, return a Decimal with signbit false, significand [0], and exponent 0
            if result.significand == vec![0] {
                return Decimal { signbit: false, ..result };
            }
            if compare_decimals(&self, &other) == std::cmp::Ordering::Greater {
                return Decimal { signbit: self.signbit, ..result }; // self is larger, so the result has the same sign as self
            } else {
                return Decimal { signbit: other.signbit, ..result }; // other is larger, so the result has the same sign as other
            }
        }
    }
   
    
}
impl  std::ops::Sub for Decimal {
        type Output = Decimal;

    fn sub(self, other: Decimal) -> Decimal {
        // For now, we will just return a new Decimal with the same value as self.
        // We will implement the actual subtraction logic later.
        // If the sign differ, we can just add the two numbers together, and give the result the same sign as self, since we are subtracting other from self.
        if self.signbit != other.signbit {
            let mut result = add_unsigned(&self, &other);
            result.signbit = self.signbit; // The result will have the same sign as self, since we are subtracting other from self
            return result;
         } 
         else {
            // If the signs are the same, we need to subtract the smaller number from the larger number, and give the result the same sign as the larger number.
            let result = sub_unsigned(&self, &other);
            // If result is zero, return a Decimal with signbit false, significand [0], and exponent 0
            if result.significand == vec![0] {
                return Decimal { signbit: false, ..result };
            }
            if compare_decimals(&self, &other) == std::cmp::Ordering::Greater {
                return Decimal { signbit: self.signbit, ..result }; // self is larger, so the result has the same sign as self
            } else {
                return Decimal { signbit: !other.signbit, ..result }; // other is larger, so the result has the opposite sign as other
             }
         }  
       
}
}

pub fn add_unsigned(d1: &Decimal, d2: &Decimal) -> Decimal {
    // This function will add two Decimal numbers together, assuming they are both positive.
    // We will need to handle the case where one or both of the numbers are negative later.
    // Determine which decimal has the larger exponent, and align the significands accordingly.
  
    let (larger, smaller) = if d1.exponent > d2.exponent {
        (d1, d2)
    } else {
        (d2, d1)
     }; 
 

    let exponent_diff = larger.exponent - smaller.exponent;
 
      let mut aligned_smaller_significand = smaller.significand.clone();
    let mut aligned_larger_significand = larger.significand.clone();
    for _ in 0..exponent_diff {
        aligned_smaller_significand.insert(0, 0); // Shift the smaller significand to the right by adding leading zeros
    }   
    // Pad the shorter significand with trailing zeros until it has the same number of digits as the longer significand
    while aligned_smaller_significand.len() < aligned_larger_significand.len() {
        aligned_smaller_significand.push(0);
    }
    while aligned_smaller_significand.len() > aligned_larger_significand   .len() {
        aligned_larger_significand.push(0);
           }
    println!("Larger: {:?}, Smaller: {:?}, Aligned smaller significand: {:?}", larger, smaller, aligned_smaller_significand);
    // Now that the significands are aligned, we can add them together.
    let mut result_significand = Vec::new();
    let mut carry = 0;
    for (a, b) in aligned_larger_significand.iter().rev().zip(aligned_smaller_significand.iter().rev()) {
        let sum = a + b + carry;
        result_significand.push(sum % 10);
        carry = sum / 10;
    }
    if carry > 0 {
        result_significand.push(carry);
    }
    result_significand.reverse();
    Decimal {
        signbit: false,
        significand: result_significand,
        exponent: larger.exponent,
    }
}

 pub fn sub_unsigned(d1: &Decimal, d2: &Decimal) -> Decimal {
    // This function will subtract two Decimal numbers, assuming they are both positive.
    // We will need to handle the case where one or both of the numbers are negative later.
    // Determine which decimal has the larger number 
    // We can determine which number is larger by comparing their significands and exponents. 
    //The number with the larger exponent is the larger number. If the exponents are the same, we can compare the significands digit by digit to determine which is larger.
    // larger should be &mut Decimal, smaller should be &Decimal, so that we can modify larger's significand if necessary to align it with smaller's significand
     let (larger, smaller) = match compare_decimals(d1, d2) {
        std::cmp::Ordering::Greater => (d1,d2), // d1 is larger
        std::cmp::Ordering::Less => (d2, d1),    // d2 is larger
        std::cmp::Ordering::Equal => return Decimal { signbit: false, significand: vec![0], exponent: 0 }, // Both numbers are equal, so the result is zero
    };
    let exponent_diff = larger.exponent - smaller.exponent; 
    println!(" larger    {:?}, smaller: {:?}, exponent_diff: {}", larger, smaller, exponent_diff);
    let mut aligned_smaller_significand = smaller.significand.clone();
    let mut aligned_larger_significand = larger.significand.clone();
    for _ in 0..exponent_diff {
        aligned_smaller_significand.insert(0, 0); // Shift the smaller significand to the right by adding leading zeros
    }   
    // Pad the shorter significand with trailing zeros until it has the same number of digits as the longer significand
    while aligned_smaller_significand.len() < aligned_larger_significand.len() {
        aligned_smaller_significand.push(0);
    }
    while aligned_smaller_significand.len() > aligned_larger_significand   .len() {
        aligned_larger_significand.push(0);
           }
    // Now that the significands are aligned, we can subtract them. larger is larger, smaller is smaller
    let mut result_significand = Vec::new();
    println!("Aligned smaller significand: {:?}", aligned_smaller_significand);
    let mut borrow = 0; 
    for (a, b) in aligned_larger_significand.iter().rev().zip(aligned_smaller_significand.iter().rev()) {
        println!("a: {}, b: {}, borrow: {}", a, b, borrow);
        let mut diff = *a as i32 - *b as i32 - borrow;
        if diff < 0 {
            diff += 10;
            borrow = 1;
        } else {
            borrow = 0;
        }
        result_significand.push(diff as u8);
    }
    // Remove leading zeros from the result significand
    let mut result_exponent = larger.exponent; // The exponent of the result is the same as the larger number's exponent
    while result_significand.last() == Some(&0) && result_significand.len() > 1 {
        result_significand.pop();
        result_exponent -= 1;
    }
    result_significand.reverse();
    Decimal {
        signbit: false,
        significand: result_significand,
        exponent: result_exponent,
    }   
}


pub fn compare_decimals(d1: &Decimal, d2: &Decimal) -> std::cmp::Ordering {
    if d1.exponent > d2.exponent {
        return std::cmp::Ordering::Greater;
    } else if d1.exponent < d2.exponent {
        return std::cmp::Ordering::Less;
    } else {
        // Exponents are the same, compare significands
        for (a, b) in d1.significand.iter().zip(d2.significand.iter()) {
            if a > b {
                return std::cmp::Ordering::Greater;
            } else if a < b {
                return std::cmp::Ordering::Less;
            }
        }
        return std::cmp::Ordering::Equal; // Both numbers are equal
    }
}

