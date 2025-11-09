pub fn reply(message: &str) -> &str {
    // todo!("have Bob reply to the incoming message: {message}")
    fn is_yelling(s: &str) -> bool{
        let has_letters = s.chars().any(|c| c.is_ascii_alphabetic());
    has_letters && s == s.to_uppercase()
    }

    let trimmed = message.trim();

    if trimmed.is_empty() {
        "Fine. Be that way!"
    }
    else if is_yelling(trimmed) && trimmed.ends_with('?'){
        "Calm down, I know what I'm doing!"
    }
    else if trimmed.ends_with('?'){
        "Sure."
    }
    else if is_yelling(trimmed) {
        "Whoa, chill out!"
    }
    else{
        "Whatever."
    }
}
