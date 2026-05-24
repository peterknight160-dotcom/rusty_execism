use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palindrome {
    factors: HashSet<(u64, u64)>,
    value: u64,
}

impl Palindrome {
    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn into_factors(self) -> HashSet<(u64, u64)> {
        self.factors
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut smallest: Palindrome = Palindrome {
        value: u64::MAX,
        factors: HashSet::new(),
    };
    let mut largest: Palindrome = Palindrome {
        value: 0,
        factors : HashSet::new(),
    };

    for i in min..=max {
        for j in i..=max {
            let product = i * j;
            if is_palindrome(product) {
                // Check if it's the smallest or largest palindrome and update accordingly
                if  product < smallest.value {
                    smallest = Palindrome {
                        value: product,
                        factors: HashSet::new(),
                    };
                    smallest.factors.insert((i, j));
                }
                if  product > largest.value {
                    largest = Palindrome {
                        value: product,
                        factors: HashSet::new(),
                    };
                    largest.factors.insert((i, j));
                }
                // If the product is equal to the current smallest or largest palindrome, add the factors to the respective HashSet
                if product == smallest.value {
                    smallest.factors.insert((i, j));
                }
                if product == largest.value {
                    largest.factors.insert((i, j));
                }
            }
        }
    }

    if smallest.value == u64::MAX || largest.value == 0 {
        None
    } else {
        Some((smallest, largest))
    }
}

pub fn is_palindrome(num: u64) -> bool {
    let s = num.to_string();
    s.chars().eq(s.chars().rev())
}