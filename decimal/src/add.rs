use crate::{Decimal, cleanup_signficand_and_exponent};
// ordering crate::Ordering;
use std::cmp::Ordering;


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
    rhs.splice(0..0, std::iter::repeat(0).take(shift));

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
 
    match compare_decimals(d1, d2) {
        Ordering::Equal => {
            return Decimal {
                signbit: false,
                significand: vec![0],
                exponent: 0,
            }
        }
        _ => {}
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