// Replaces the caller's text with "[censored]" if it is "secret".
// TODO: Fix the function signature and implement the function.
fn censor(text: &String) {}

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn secret() {
        let mut text = String::from("secret");
        censor(&mut text);
        assert_eq!(text, "[censored]");
    }

    #[test]
    fn not_secret() {
        let mut text = String::from("hello");
        censor(&mut text);
        assert_eq!(text, "hello");
    }
}
