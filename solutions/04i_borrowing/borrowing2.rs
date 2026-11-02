// `&str` is more flexible than `&String`.
fn greeting(name: &str) -> &'static str {
    if name == "Ferris" {
        "Hi crab!"
    } else {
        "Hello!"
    }
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn owned() {
        let name = String::from("Ferris");
        assert_eq!(greeting(&name), "Hi crab!");
    }

    #[test]
    fn literal() {
        assert_eq!(greeting("Ana"), "Hello!");
        assert_eq!(greeting("Ferris"), "Hi crab!");
    }
}
