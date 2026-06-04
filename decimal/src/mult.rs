

use crate::Decimal;


impl std::ops::Mul for Decimal {
    type Output = Decimal;

    fn mul(self, other: Decimal) -> Decimal {
        //println!( "self: {}, other: {}", self, other);
        let result_sign = self.signbit ^ other.signbit; // The result will be negative if one of the numbers is negative, but not both
        let mut result_exponent = self.exponent + other.exponent - 1; // The exponent of the result will be the sum of the exponents of the two numbers
        let mut result_significand = Vec::new();    
        let  left_significand: Vec<u8> = self.significand.clone().iter().rev().cloned().collect(); 
        let  right_significand: Vec<u8> = other.significand.clone().iter().rev().cloned().collect(); 
         //println!( "left_significand: {:?}, right_significand: {:?}", left_significand, right_significand);
         
        // We will multiply the significands together using long multiplication, and store the result in result_significand.
        for (i, digit) in left_significand.iter().enumerate() {
            //println!("Multiplying digit {} of left significand: {}", i, digit);
            // So starting at the right most digit in self, multiply each other significand into a temp 
            let mut stanza : Vec<u8> = Vec::new();
            let mut carry =  0;
            for digit2 in right_significand.iter() {
                let x = digit * digit2 + carry;
                stanza.push( x % 10);
                carry = x /10;
            }
            if carry > 0 {
                stanza.push(carry);
            }
            
             //println!("stanza before shifting: {:?}", stanza);
             // Then shift the temp to the left by i digits by adding i zeros to the end of the temp
            for _  in 0..i {
                stanza.insert(0, 0);
            }
              //  println!("stanza after shifting: {:?}\n", stanza);
            // Now add stanza to result_significand
            // Align lengths
            while result_significand.len()  < stanza.len() {
               result_significand.push(0);
            }
            while stanza.len() < result_significand.len() {
                stanza.insert(0, 0);
            }
            //println!("stanza: {:?}, result_significand: {:?}", stanza, result_significand);
            let mut new_result_significand = Vec::new();    
            let mut carry = 0;
            //println!("Adding stanza to result_significand: {:?} + {:?}", stanza, result_significand);
            for (a, b) in result_significand.iter().zip(stanza.iter()) {
              
                let sum = a + b + carry;
                new_result_significand.push(sum % 10);
                carry = sum / 10;
            }
            if carry > 0 {
                new_result_significand.push(carry);
            }
            //println!("new_result_significand before reversing: {:?}", new_result_significand);
            result_significand = new_result_significand;
        }
        result_significand.reverse();
        //println!("result_significand after reversing: {:?}, result_exponent: {}", result_significand, result_exponent);
            // Remove leading zeros from result_significand
            while result_significand.first() == Some(&0) && result_significand.len() > 1 {
                result_significand.remove(0);
                result_exponent -= 1; // Each leading zero removed decreases the exponent
            }   
                // Remove trailing zeros from result_significand
                while result_significand.last() == Some(&0) && result_significand.len() > 1 {
                    result_significand.pop();
                }   

            Decimal {
                signbit: result_sign,
                significand: result_significand,
                exponent: result_exponent,
            }
      
    }

}
