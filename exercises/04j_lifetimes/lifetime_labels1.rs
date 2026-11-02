// Returns the text that comes later in alphabetical order.
// TODO: Fix the compiler error by adding lifetime labels.
fn later(a: &str, b: &str) -> &str {
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
