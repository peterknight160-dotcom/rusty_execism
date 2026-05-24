use std::fmt::{self, Display, Formatter}; 
use std::ops:: Add;




/// Type implementing arbitrary-precision decimal arithmetic
///
///
#[derive(Debug)]

#[derive(Clone, )]  
pub struct Decimal {
    // Sign of the number - true for positive, false for negative
    sign: bool,
    // Stored as two parts - an signed integer exponent and a vector of digits, in the range of 0.1 to just less than 1
    exponent: i32,
    digits: Vec<u32>,
}

impl Decimal {
    pub fn new(sign: bool, exponent: i32, digits: Vec<u32>) -> Self {
        Decimal {
            sign,
            exponent,
            digits,
        }
    }

    pub fn try_from(input: &str) -> Option<Decimal> {
        // Parse the input string to create a Decimal instance
        // Get first character to determine the sign,
        let mut sign = false;
        let mut exponent = 0;
        let mut digits: Vec<u32> = Vec::new();

        // Parse the digits and determine the exponent
        for c in input.chars() {
            if c == '-' {
                if digits.is_empty() {
                    sign = true; // Negative number
                } else {
                    return None; // '-' should only be at the beginning
                }
            } else if c.is_digit(10) {
                digits.push(c.to_digit(10).unwrap());
            } else if c == '.' {
                exponent = digits.len() as i32;
            } else {
                return None; // Invalid character in input
            }
        }
        digits.reverse(); // Reverse the digits to have the least significant digit at index 0
        //Remove trailing zeros from the digits vector and adjust the exponent accordingly
        while let Some(&0) = digits.last() {
            digits.pop();
            exponent -= 1;
        }
        //Remove leading zeros from the digits vector  
        while let Some(&0) = digits.first() {
            digits.remove(0);
        }

        Some(Decimal::new(sign, exponent, digits))
    }
}

pub fn add_unsigned(left: &Decimal, right: &Decimal) -> Decimal {
    // Add the absolute values of left and right, ignoring their signs
    // Return a new Decimal with the correct sign based on the original signs of left and right
    // This is a placeholder implementation and should be replaced with actual addition logic
    // Which has higher exponent, left or right?
    let mut result_exponent ;
    let mut aligned_left_digits: Vec<u32>;
    let mut aligned_right_digits: Vec<u32>;
    if left.exponent > right.exponent {
        // Align right to left
        aligned_right_digits = right.digits.clone();
        for _ in 0..(left.exponent - right.exponent) {
            aligned_right_digits.push(0); // Add trailing zeros to align    
                 
        }
        aligned_left_digits = left.digits.clone();
        result_exponent = left.exponent; // The result will have the same exponent as the larger one
      
    } else if left.exponent < right.exponent {
        // Align left to right
        aligned_left_digits = left.digits.clone();
        for _ in 0..(right.exponent - left.exponent) {
            // Add trailing zeros to align left to right
            aligned_left_digits.push(0);
     
            
        }
        aligned_right_digits = right.digits.clone();

        result_exponent = right.exponent; // The result will have the same exponent as the larger one
      
    } else {
        // Exponents are equal, no alignment needed
        aligned_left_digits = left.digits.clone();
        aligned_right_digits = right.digits.clone();
        result_exponent = left.exponent; // The result will have the same exponent as the inputs
      
    }
  // Add trailing zeros to the shorter digits vector to ensure they are the same length for addition

  if  aligned_left_digits.len() < aligned_right_digits.len() {
        for _ in 0..(aligned_right_digits.len() - aligned_left_digits.len()) {
            aligned_left_digits.insert(0, 0);
        }
    } else if aligned_right_digits.len() < aligned_left_digits.len() {
        for _ in 0..(aligned_left_digits.len() - aligned_right_digits.len()) {
            aligned_right_digits.insert(0, 0);
        }
    }
  

    let mut result_digits: Vec<u32> = Vec::new();
    let mut carry = 0;
    for (a, b) in aligned_left_digits.iter().zip(aligned_right_digits.iter()) {
       
        let sum = a + b + carry;
        result_digits.push(sum % 10); // Store the last digit of the sum
        carry = sum / 10;    // Calculate the carry for the next iteration 
    }
    if carry > 0 {
        result_digits.push(carry); // Add any remaining carry
        result_exponent += 1; // If there is a carry, the exponent increases by 1
    }

    Decimal::new(false, result_exponent, result_digits)
}

pub fn sub_unsigned(a: &Decimal, b: &Decimal) -> Decimal {
    // Subtract the smaller absolute value from the larger absolute value, ignoring their signs
    let  mut left: &Decimal = a;
    let  mut right: &Decimal = b;
    let comparison = abs_compare(&a, &b);  
    if comparison == 0 {
        // If the absolute values are equal, the result is zero
        return Decimal::new(false, 0, vec![0]);
    }
    if comparison < 0 {
        // If a < b, swap them to ensure we are always subtracting the smaller from the larger
        right = a;
        left = b;
    } 
  
    let mut result_exponent = 0;
    let mut aligned_left_digits: Vec<u32>;
    let mut aligned_right_digits: Vec<u32>;
    if left.exponent > right.exponent {
        // Align right to left
        aligned_right_digits = right.digits.clone();
        for _ in 0..(left.exponent - right.exponent) {
            aligned_right_digits.push(0); // Add trailing zeros to align    
        }
        aligned_left_digits = left.digits.clone();
        result_exponent = left.exponent; // The result will have the same exponent as the larger one
    } else if left.exponent < right.exponent {
        // Align left to right
        aligned_left_digits = left.digits.clone();
        for _ in 0..(right.exponent - left.exponent) {
            // Add trailing zeros to align left to right
            aligned_left_digits.push(0);
        }
        aligned_right_digits = right.digits.clone();
        result_exponent = right.exponent; // The result will have the same exponent as the larger one
    } else {
        // Exponents are equal, no alignment needed
        aligned_left_digits = left.digits.clone();
        aligned_right_digits = right.digits.clone();
    }
    // Perform addition of aligned digits and handle carry


    let mut result_digits: Vec<u32> = Vec::new();
    let mut borrow: u32 = 0;
    for (a, b) in aligned_left_digits.iter().zip(aligned_right_digits.iter()) {
        if *a   < *b + borrow {
            // If the current digit of left is less than the current digit of right plus any borrow, we need to borrow from the next higher digit
            
            result_digits.push((*a + 10 - *b - borrow) as u32); // Add 10 to the current digit of left to perform the subtraction
            borrow = 1; // Set borrow for the next iteration
        } else {
             result_digits.push((*a - *b - borrow) as u32);
            borrow = 0; // No borrow needed for the next iteration
        }
        
    }
    

    Decimal::new(false, result_exponent, result_digits)
}

impl Add for Decimal {
    type Output = Decimal;
    fn add(self, other: Decimal) -> Decimal {
        if (self.sign != other.sign) && abs_compare(&self, &other) == 0 {
            // If the signs are different and the absolute values are equal, the result is zero
            return Decimal::new(false, 0, vec![0]);
        }

        match (self.sign, other.sign) {
            (true, true) => add_unsigned(&self, &other), // Both are negative, add their absolute values and keep the result negative
            (false, false) => add_unsigned(&self, &other), // Both are positive, add their absolute values and keep the result positive
            (true, false) => sub_unsigned(&self, &other), // Left is negative and right is positive, subtract the smaller from the larger and determine the sign based on which is larger
            (false, true) => sub_unsigned(&other, &self), // Left is positive and right is negative, subtract the smaller from the larger and determine the sign based on which is larger
        }
    }
}

impl Display for Decimal {
    // `f` is a buffer, and this method must write the formatted string into it.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let sign_str = if self.sign { "-" } else { "" };
        let mut digits_str = String::new();
        let mut leading_zeros = String::new();
        for (i, digit) in self.digits.iter().rev().enumerate() {
            if i == self.exponent as usize && self.exponent > 0 {
                digits_str.push('.'); // Insert decimal point at the correct position
            }
            digits_str.push_str(&digit.to_string());
        }
        // If the exponent is greater than the number of digits, we need to add trailing zeros
        if self.exponent > self.digits.len() as i32 {
            digits_str.push_str(&"0".repeat((self.exponent - self.digits.len() as i32) as usize));
        }

        if self.exponent <= 0 {
            // If the exponent is negative, we need to add leading zeros
            leading_zeros.push('0');
            leading_zeros.push('.'); // Add decimal point after the leading zeros
            leading_zeros.push_str(&"0".repeat((-self.exponent) as usize));
            //println!("Leading zeros for negative exponent: {}", leading_zeros);
        }

        write!(f, "{}{}{}", sign_str, leading_zeros, digits_str)
    }
}
/* impl PartialEq for Decimal {
    fn eq(&self, other: &Self) -> bool {
        self.sign == other.sign
            && self.int_part == other.int_part
            && self.frac_part == other.frac_part
    }
}
use itertools::{EitherOrBoth, Itertools};
use std::ops::Add;
use std::ops::Sub;
impl Add for Decimal {
    type Output = Decimal;
    fn add(self, other: Decimal) -> Decimal {
        match (self.sign, other.sign) {
            (true, true) => Decimal::add_unsigned(self, other),
            (false, false) => Decimal::add_unsigned(self, other),
            (true, false) => Decimal::sub_unsigned(self, other),
            (false, true) => Decimal::sub_unsigned(self, other),
        }
    }
}
 */


fn abs_compare(left: &Decimal, right: &Decimal) -> i32 {
    // Compare the absolute values of left and right
    // Return 1 if left > right, -1 if left < right, and 0 if they are equal
    if left.exponent > right.exponent {
        return 1; // Left has more digits, so it's greater
    } else if left.exponent < right.exponent {
        return -1; // Right has more digits, so it's greater
    } else {
        // Exponents are equal, compare digit by digit
        for (a, b) in left.digits.iter().zip(right.digits.iter()) {
            if a > b {
                return 1; // Left is greater
            } else if a < b {
                return -1; // Right is greater
            }
        }
        return 0; // They are equal
    }
    
    
} 
