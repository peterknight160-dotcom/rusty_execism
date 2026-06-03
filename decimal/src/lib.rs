use core::fmt;

/// Type implementing arbitrary-precision decimal arithmetic
//use std::fmt::Display;


// add.rs contains the implementation of addition for Decimal
mod add;
#[derive(Debug)]
pub struct Decimal {
    // Use exponent and significand to represent the decimal value
    // value = significand * 10^exponent
    signbit : bool,    //True if negative, false if positive
    significand: Vec<u8>, 
    exponent: i32,
}

impl Decimal {
    pub fn try_from(input: &str) -> Option<Decimal> {
        // Parse the input string to extract the sign, significand, and exponent
        let mut chars = input.chars().peekable();
        let mut signbit = false;
        let mut significand = Vec::new();
        let mut exponent = 0;

        // Handle optional sign
        if let Some(&c) = chars.peek() {
            if c == '-' {
                signbit = true;
                chars.next();
            } else if c == '+' {
                chars.next();
            }
        }

        // Parse digits and decimal point
        let mut decimal_point_seen = false;
        for c in chars {
            if c == '0' && significand.is_empty() &&  decimal_point_seen {
                exponent -= 1;
                continue;
            }
            if c.is_digit(10) {
                significand.push(c.to_digit(10).unwrap() as u8);
                if ! decimal_point_seen {
                    exponent += 1; // Each digit before the decimal point increases the exponent
                }
            } else if c == '.' {
                if decimal_point_seen {
                    return None; // Invalid input: multiple decimal points
                }
                decimal_point_seen = true;
            } else {
                return None; // Invalid character
            }
        }

        // Remove leading zeros from significand
        while significand.first() == Some(&0) && significand.len() > 1 {
            significand.remove(0);
            exponent -= 1; // Each leading zero removed decreases the exponent
        }
        // Strip trailing zeros from significand 
        while significand.last() == Some(&0) && significand.len() > 1 {
            significand.pop();
        
        }

    

        Some(Decimal { signbit, significand, exponent })    
    }
    // Need to implement std:fmt::Display for Decimal to print it
    
}


impl fmt::Display for Decimal {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut result = String::new();
            let mut exponent = self.exponent;
            if self.signbit {
                result.push('-');
            }
            for &digit in &self.significand {
                result.push((digit + b'0') as char);
                exponent -= 1;
                if exponent == 0 {
                    result.push('.');
                }
            }
            println! ("result: {}, exponent: {}", result, self.exponent);
            if self.exponent < 1 {
                for _ in 0..(-exponent) {
                    // If the exponent is negative, we need to add leading zeros to the front of the result string
                    result.insert(0, '0');
                }
                // Add a decimal point after the first digit if there are more than one digit in the significand
        
                    result.insert(1, '.');
               
            }
             
            write!(f, "{}", result)
        }
    }