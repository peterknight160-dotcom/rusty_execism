// This solution was largely written with the help of Github Copilot

// Helper functions to put a number into a words

//  0 ..99
fn encode_below_hundred(n: u64) -> String {
    let below_20 = [
       "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
       "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen", 
       ]; 
     
    let tens = [
        "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];

    if n < 20 {
        below_20[n as usize].to_string()
    } else {
        let ten_part = tens[(n / 10) as usize];
        let unit_part = n % 10;
        if unit_part == 0 {
            ten_part.to_string()
        } else {
            format!("{}-{}", ten_part, below_20[unit_part as usize])
        }
    }
}

// 0 ..999, built on encode_below_hundred
fn hundreds_to_words(n: u64) -> String {
    if n < 100 {
        encode_below_hundred(n)
    } else {
        let hundred_part = n / 100;
        let rest = n % 100;
        if rest == 0 {
            format!("{} hundred", encode_below_hundred(hundred_part))
        } else {
            format!(
                "{} hundred {}",
                encode_below_hundred(hundred_part),
                encode_below_hundred(rest)
            )
        }
    }
}

pub fn encode(n: u64) -> String {
    let powers_of_thousand = vec![
        ("quintillion", 1_000_000_000_000_000_000),
        ("quadrillion", 1_000_000_000_000_000),
        ("trillion", 1_000_000_000_000),
        ("billion", 1_000_000_000),
        ("million", 1_000_000),
        ("thousand", 1_000),
    ];

    if n == 0 {
        return "zero".to_string();
    }
    let mut remainder = n;
    let mut parts: Vec<String> = Vec::new(); // Explcit type annotation
    for &(name, value) in &powers_of_thousand {
        if remainder >= value {
            let count = remainder / value;
            remainder %= value;
            let part = hundreds_to_words(count);
            parts.push(format!("{} {}", part, name));
        }
    }
    if remainder > 0 {
        // Handle the last part if any
        parts.push(hundreds_to_words(remainder));
    }
    parts.join(" ")
}
