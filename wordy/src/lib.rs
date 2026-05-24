pub fn answer(command: &str) -> Option<i32> {
    const TOKENS: [&str; 8] = [
        "What is",
        "plus",
        "minus",
        "multiplied by",
        "divided by",
        "raised to the",
        "power?",
        "?",
    ];

    let mut tokens = Vec::new();
    let mut input = command.trim();

    // --- Tokenisation ---
    while !input.is_empty() {
        if let Some(token) = TOKENS.iter().find(|t| input.starts_with(**t)) {
            tokens.push(*token);
            input = input[token.len()..].trim();
        } else {
            let end = input
                .find(|c: char| !c.is_ascii_digit() && c != '-')
                .unwrap_or(input.len());

            let num = input[..end].trim();
            num.parse::<i32>().ok()?; // validate
            tokens.push(num);

            input = input[end..]
                .trim_start_matches(|c: char| "ndrth".contains(c))
                .trim();
        }
    }

    // --- Validation ---
    if tokens.first()? != &"What is" {
        return None;
    }

    // --- Evaluation ---
    let mut iter = tokens.iter().skip(1);
    let mut result = iter.next()?.parse::<i32>().ok()?;

    while let Some(op) = iter.next() {
        // Do I get here?

        if op == &"power?" || op == &"?" {
            break;
        }
        let val = iter.next()?.parse::<i32>().ok()?;

        result = match *op {
            "plus" => result + val,
            "minus" => result - val,
            "multiplied by" => result * val,
            "divided by" => (val != 0).then(|| result / val)?,
            "raised to the" => (val >= 0).then(|| result.pow(val as u32))?,
            _ => {
                println!("Unknown operator: {}", op);
                return None;
            }
        };
    }

    Some(result)
}
