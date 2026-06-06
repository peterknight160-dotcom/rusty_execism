use core::fmt;


#[derive(Debug,Clone,PartialEq, Eq)]
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
            if c.is_ascii_digit() {
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
        if signbit && significand.iter().all(|&d| d == 0) {
            signbit = false; // If the number is zero, it should not be negative
        }
        

        cleanup_signficand_and_exponent(&mut significand, &mut exponent);
      
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
            // If the exponent is negative, we need to add leading zeros and a decimal point
            if exponent <= 0 {
                result.push('0');
                result.push('.');
               while exponent < 0 {
                   result.push('0');
                   exponent += 1;
               }
            }
        
            // Now we can add the digits of the significand, and add a decimal point if we reach the position of the decimal point
                for &digit in &self.significand {
                    result.push((digit + b'0') as char);
                    exponent -= 1;
                    // is digit is the last digit of the significand and the exponent is 0, we need to avoid adding a decimal point
                    if exponent == 0 && digit != *self.significand.last().unwrap() {
                        result.push('.');
                    }
                } 
                // If we have processed all digits but still have an exponent greater than 0, we need to add trailing zeros
            while exponent > 0 {
                result.push('0');
                exponent -= 1;
            }
             // If we have processed all digits but still have an exponent less than 0, we need to add leading zeros and a decimal point       
           /*  
            for &digit in &self.significand {
                result.push((digit + b'0') as char);
                exponent -= 1;
                if exponent == 0 {
                    result.push('.');
                }
            }
            // If we have processed all digits but still have an exponent greater than 0, we need to add trailing zeros
            while exponent > 0 {
                result.push('0');
                exponent -= 1;
            }
             // If we have processed all digits but still have an exponent less than 0, we need to add leading zeros and a decimal point
            while exponent < 0 {
                result.insert(0, '0');
                exponent += 1;
            }
            
             */
            write!(f, "{}", result) 
        }
    }


    pub fn cleanup_signficand_and_exponent (signficand : &mut Vec<u8>, exponent: &mut i32)   {
        
        // Remove leading zeros from significand
        while signficand.first() == Some(&0) && signficand.len() > 1 {
            signficand.remove(0);
            *exponent -= 1; // Each leading zero removed decreases the exponent
        }
        // Strip trailing zeros from significand 
        while signficand.last() == Some(&0) && !signficand.is_empty() {
            signficand.pop();
        
        }
          if signficand.is_empty() {
          
            signficand.push(0); // If there are no digits, the number is zero
            *exponent = 0; // Set exponent to 0 for zero
        }
  
    }

    use std::cmp::Ordering;
   
use std::cmp::PartialOrd;


impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Decimal) -> Option<Ordering> {
        // Compare two Decimals based on their sign, exponent, and significand
        if self.signbit != other.signbit {
            return if self.signbit { Some(Ordering::Less) } else { Some(Ordering::Greater) };
        }
        if self.exponent != other.exponent {
            // Reverse sign for negative numbers
            return if (self.exponent < other.exponent) ^ self.signbit { Some(Ordering::Less) } else { Some(Ordering::Greater) };
        }
      
        // Compare significands but reverse sign for negative numbers
        let self_significand = self.significand.iter().rev();
        let other_significand = other.significand.iter().rev();
        // Pad the shorter significand with zeros for comparison
        let max_len = std::cmp::max(self.significand.len(), other.significand.len());
        let self_significand = self_significand.chain(std::iter::repeat(&0)).take(max_len);
        let other_significand = other_significand.chain(std::iter::repeat(&0)).take(max_len);
        for (a, b) in self_significand.zip(other_significand) {
            if a != b {
                return if (a < b) ^ self.signbit { Some(Ordering::Less) } else { Some(Ordering::Greater) };
            }
        }
        Some(Ordering::Equal)
    }
}

impl Ord for Decimal {
    fn cmp(&self, other: &Decimal) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}


impl std::ops::Add for Decimal {
    type Output = Decimal;

    fn add(self, other: Decimal) -> Decimal {
        match self.signbit == other.signbit {
            true => {
                let mut result = add_unsigned(&self, &other);
                result.signbit = self.signbit;
                result
            }
            false => signed_sub(self, other),
        }
    }
}

impl std::ops::Sub for Decimal {
    type Output = Decimal;

    fn sub(self, other: Decimal) -> Decimal {
        match self.signbit == other.signbit {
            true => signed_sub(self, other),
            false => {
                let mut result = add_unsigned(&self, &other);
                result.signbit = self.signbit;
                result
            }
        }
    }
}

fn signed_sub(a: Decimal, b: Decimal) -> Decimal {

    // Function sub_unsigned assumes that abs(a) >= abs(b), so we need to compare the two decimals first and then call sub_unsigned with the larger one first
    let (larger, smaller) = match compare_decimals(&a, &b) {
        Ordering::Greater => (a.clone(), b.clone()),
        Ordering::Less => (b.clone(), a.clone()),
        Ordering::Equal => {
            return Decimal {
                signbit: false,
                significand: vec![0],
                exponent: 0,
            }
        }
    };
  
    let  result = sub_unsigned(&larger, &smaller);

    if result.significand == [0] {
        return Decimal { signbit: false, ..result };
    }

    match compare_decimals(&a, &b) {
        Ordering::Greater => Decimal { signbit: a.signbit, ..result },
        Ordering::Less => Decimal { signbit: !a.signbit, ..result },
        Ordering::Equal => Decimal { signbit: false, ..result },
    }
}

//
// ✅ Shared helper: align significands
//
fn align_significands(d1: &Decimal, d2: &Decimal) -> (Vec<u8>, Vec<u8>, i32) {
    let (larger, smaller) = if d1.exponent >= d2.exponent {
        (d1, d2)
    } else {
        (d2, d1)
    };

    let mut lhs = larger.significand.clone();
    let mut rhs = smaller.significand.clone();

    // Shift smaller (prefix zeros)
    let shift = (larger.exponent - smaller.exponent) as usize;
    rhs.splice(0..0, std::iter::repeat_n(0, shift));

    // Pad to equal length (trailing zeros)
    let max_len = lhs.len().max(rhs.len());
    lhs.resize(max_len, 0);
    rhs.resize(max_len, 0);

    (lhs, rhs, larger.exponent)
}

//
// ✅ Unsigned addition
//
pub fn add_unsigned(d1: &Decimal, d2: &Decimal) -> Decimal {
    let (lhs, rhs, exponent) = align_significands(d1, d2);

    let mut result = Vec::with_capacity(lhs.len() + 1);
    let mut carry = 0;

    for (&a, &b) in lhs.iter().rev().zip(rhs.iter().rev()) {
        let sum = a + b + carry;
        result.push(sum % 10);
        carry = sum / 10;
    }
 let mut exp = exponent;
    if carry > 0 {
        result.push(carry);
        exp  += 1; // If there's a carry after the last addition, we need to increase the exponent by 1
    }

    result.reverse();

    cleanup_signficand_and_exponent(&mut result, &mut exp);

    Decimal {
        signbit: false,
        significand: result,
        exponent: exp,
    }
}

//
// ✅ Unsigned subtraction
//
pub fn sub_unsigned(d1: &Decimal, d2: &Decimal) -> Decimal {

    if (d1.exponent, &d1.significand) == (d2.exponent, &d2.significand) {
        return Decimal {
            signbit: false,
            significand: vec![0],
            exponent: 0,
        }
    }
 
   

    let (lhs, rhs, exponent) = align_significands(d1, d2);
   

    let mut result = Vec::with_capacity(lhs.len());
    let mut borrow = 0;

    for (&a, &b) in lhs.iter().rev().zip(rhs.iter().rev()) {
        let mut diff = a as i32 - b as i32 - borrow;
    
        borrow = 0; // Reset borrow for the next iteration 
        while diff < 0 {
            diff += 10;
            borrow = 1; // We need to borrow for the next iteration
        }

        result.push(diff as u8);
    }
  
    result.reverse();

    let mut exp = exponent;
    cleanup_signficand_and_exponent(&mut result, &mut exp);

    Decimal {
        signbit: false,
        significand: result,
        exponent: exp,
    }
}

//
// ✅ Decimal comparison
//
pub fn compare_decimals(d1: &Decimal, d2: &Decimal) -> Ordering {
    match d1.exponent.cmp(&d2.exponent) {
        Ordering::Equal => {
            for (a, b) in d1.significand.iter().zip(&d2.significand) {
                match a.cmp(b) {
                    Ordering::Equal => continue,
                    ord => return ord,
                }
            }
            Ordering::Equal
        }
        ord => ord,
    }
}

impl std::ops::Mul for Decimal {
    type Output = Decimal;

    fn mul(self, other: Decimal) -> Decimal {
        let sign = self.signbit ^ other.signbit;
        let mut exponent = self.exponent + other.exponent - 1;

        // Work with reversed digits (LSB first)
        let lhs: Vec<u8> = self.significand.iter().rev().copied().collect();
        let rhs: Vec<u8> = other.significand.iter().rev().copied().collect();

        // Result buffer (max possible length)
        let mut result = vec![0u8; lhs.len() + rhs.len()];
                let  carry = 0; 
        // Long multiplication (more efficient accumulation)
        for (i, &a) in lhs.iter().enumerate() {
            let mut carry = 0; 

            for (j, &b) in rhs.iter().enumerate() {
                let idx = i + j;
                let tmp = result[idx] + a * b + carry;

                result[idx] = tmp % 10;
                carry = tmp / 10;
            }
            if carry > 0 {
                result[i + rhs.len()] += carry;
               
            }
            // Is this the last iteration of the outer loop? If so, we need to add any remaining carry to the next position in the result.
             if i == lhs.len() - 1 && carry > 0 {
             exponent += 1; // If there's a carry after the last multiplication, we need to increase the exponent by 1 
             }
        }
        if carry > 0 {
           
          exponent += 1; // If there's a carry after the last multiplication, we need to increase the exponent by 1 
        }

        // Remove leading zeros (from reversed representation)
        while result.len() > 1 && *result.last().unwrap() == 0 {
            result.pop();
        }

        // Reverse back to normal order
        result.reverse();

        cleanup_signficand_and_exponent(&mut result, &mut exponent);

        Decimal {
            signbit: sign,
            significand: result,
            exponent,
        }
    }
}