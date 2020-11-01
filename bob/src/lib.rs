pub fn reply(message: &str) -> &str {
    // would use lines() but the tests require a carriage return only (mac)
    let last_message = message.split('\r').last().unwrap().trim();

    if last_message.is_empty() {
        return "Fine. Be that way!";
    }

    let is_question = last_message.ends_with('?');
    let is_yell =
        last_message.to_uppercase() == last_message && last_message.to_lowercase() != last_message;

    if is_question && is_yell {
        return "Calm down, I know what I'm doing!";
    } else if is_question {
        return "Sure.";
    } else if is_yell {
        return "Whoa, chill out!";
    }

    "Whatever."
}
