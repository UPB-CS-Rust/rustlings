// Returns the text that comes later in alphabetical order.
// The result is one of the inputs, so all references get the label `'a`.
fn later<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a > b { a } else { b }
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn literals() {
        assert_eq!(later("apple", "banana"), "banana");
        assert_eq!(later("zebra", "ant"), "zebra");
    }

    #[test]
    fn strings() {
        let a = String::from("rust");
        let b = String::from("crab");
        assert_eq!(later(&a, &b), "rust");
    }
}
