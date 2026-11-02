enum Message {
    Text(String),
    Number(i32),
}

// Returns a short description and the original message.
fn describe(message: Message) -> (&'static str, Message) {
    // TODO: Fix the compiler error by changing only the `match` line.
    let kind = match message {
        Message::Text(text) => {
            println!("Got a text: {text}");
            "text"
        }
        Message::Number(n) => {
            println!("Got a number: {n}");
            "number"
        }
    };

    (kind, message)
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text() {
        let (kind, message) = describe(Message::Text(String::from("hi")));
        assert_eq!(kind, "text");
        match message {
            Message::Text(t) => assert_eq!(t, "hi"),
            Message::Number(_) => panic!("The message changed!"),
        }
    }

    #[test]
    fn number() {
        let (kind, _) = describe(Message::Number(3));
        assert_eq!(kind, "number");
    }
}
