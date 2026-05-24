pub fn encode(source: &str) -> String {
    let mut encoding = String::from("");
    let mut char2: Option<char> = None;
    let mut occurrences = 1;
    for char1 in source.chars() {
        if char2.is_none() {
            char2 = Some(char1);
            continue;
        } // With one character, can't make any decisions

        if char2.unwrap() != char1 {
            if occurrences > 1 {
                encoding.push_str(&occurrences.to_string());
                occurrences = 1;
            }
            encoding.push(char2.unwrap());
        } else {
            occurrences += 1;
        }
        char2 = Some(char1);
    }
    if char2.is_some() {
        if occurrences > 1 {
            encoding.push_str(&occurrences.to_string());
        }
        encoding.push(char2.unwrap());
    }
    encoding
}

pub fn decode(source: &str) -> String {
    let mut decoded = String::new();
    let mut num = String::new();
    for char1 in source.chars() {
        
        if char1.is_numeric() {
            num.push(char1);
            continue;
        } else {
            if num.is_empty() {
                decoded.push(char1);
            } else {
                let c = num.parse::<u32> () ;
                if ! c.is_ok()  {
                    panic!( "Value is duff ");
                }
                for _i in 0..c.unwrap()  {
                    decoded.push(char1);    
                }
                num.clear();
                
            }
        }
    }
    decoded
}