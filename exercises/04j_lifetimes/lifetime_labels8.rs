// TODO: Fix the compiler error. Adding labels won't help here!
fn make_title<'a>(word: &str) -> &'a String {
    let text = String::from(word);
    &text
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn title() {
        let title = make_title("Rust");
        assert_eq!(title, "Rust");
    }
}
