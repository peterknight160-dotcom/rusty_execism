pub fn translate(input: &str) -> String {
    println!("input: {}", input);
    // Split the input into words, translate each word, and then join them back together.
    input
        .split_whitespace()
        .map(|word| translate_word(word))
        .collect::<Vec<String>>()
        .join(" ")
}

fn translate_word(input: &str) -> String {
    let rule1 = regex::Regex::new(r"^[aeiouAEIOU]|^(xr|yt)").unwrap();
    if rule1.is_match(input) {
        println!("rule 1 matched");
        return format!("{}ay", input);
    }

    let rule3 = regex::Regex::new(r"^([^aeiouAEIOU]*qu)(.*)").unwrap();
    if let Some(caps) = rule3.captures(input) {
        println!("caps: {:?}", caps);
        return format!("{}{}ay", &caps[2], &caps[1]);
    }
    let rule4 = regex::Regex::new(r"^([^aeiouAEIOU]+)(y.*)").unwrap();
    if let Some(caps) = rule4.captures(input) {
        println!("caps: {:?}", caps);
        return format!("{}{}ay", &caps[2], &caps[1]);
    }

    let rule2 = regex::Regex::new(r"^([^aeiouAEIOU]+)(.*)").unwrap();
    if let Some(caps) = rule2.captures(input) {
        println!("caps: {:?}", caps);
        return format!("{}{}ay", &caps[2], &caps[1]);
    }

    String::new()
}
