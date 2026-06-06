// For the PartialEq, PartialOrd, Eq and Ord traits 
use crate::Decimal;
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
