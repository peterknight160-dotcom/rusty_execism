use std::fmt::{Display, Formatter, Result};

pub struct Roman {
    digits: String,
}

struct Decoding<'a> {
    digits: &'a str,
    value: u32

}

const DECODE_TABLE: [Decoding; 13] = [
    Decoding { digits: "M", value: 1000 },
    Decoding { digits: "CM", value: 900 },
    Decoding { digits: "D", value: 500 },
    Decoding { digits: "CD", value: 400 },
    Decoding { digits: "C", value: 100 },
    Decoding { digits: "XC", value: 90 },
    Decoding { digits: "L", value: 50 },
    Decoding { digits: "XL", value: 40 },
    Decoding { digits: "X", value: 10 },
    Decoding { digits: "IX", value: 9 },
    Decoding { digits: "V", value: 5 },
    Decoding { digits: "IV", value: 4 },
    Decoding { digits: "I", value: 1 },
];


impl Display for Roman {
    fn fmt(&self, _f: &mut Formatter<'_>) -> Result {
        write!(_f, "{}", self.digits.as_str())
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
 
        let mut digits = String::new();
        let mut num1 = num;
        for dec in DECODE_TABLE.iter() {
            while num1 >= dec.value {
                digits.push_str(dec.digits);
                num1 -= dec.value;
            }
        }
        
        Roman { digits }
    }
}
