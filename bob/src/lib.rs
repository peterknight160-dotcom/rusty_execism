pub fn reply(message: &str) -> &str {
    // Is the message empty or only whitespace?
    if message.trim().is_empty() {
        return "Fine. Be that way!";
    }
    // Check for shouting just once
    let shouting = message.chars().any(|c| c.is_alphabetic()) && message.chars().all(|c| !c.is_lowercase()); 
    
    if  message.trim().ends_with('?') {
        // Is the message in all uppercase letters (shouting)?
        if shouting {
            return "Calm down, I know what I'm doing!";
        } else {
            return "Sure.";
        }
    }
    if shouting {
        return "Whoa, chill out!";
    }
    "Whatever."
    //    todo!("have Bob reply to the incoming message: {message}")
}

