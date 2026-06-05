

use crate::Decimal;
use crate::cleanup_signficand_and_exponent;

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
            println!("Final carry after multiplication: {}", carry);
          exponent += 1; // If there's a carry after the last multiplication, we need to increase the exponent by 1 
        }
        println!("Raw multiplication result (reversed): {:?}, carry: {}, exponent: {}", result, 0, exponent);
         

        // Remove leading zeros (from reversed representation)
        while result.len() > 1 && *result.last().unwrap() == 0 {
            result.pop();
        }

        // Reverse back to normal order
        result.reverse();

        // Use the cleanup function to remove any leading zeros and adjust the exponent accordingly
        // Cleanup is in lib.rs, so we need to import it and call it here

        cleanup_signficand_and_exponent(&mut result, &mut exponent);

    

        Decimal {
            signbit: sign,
            significand: result,
            exponent,
        }
    }
}