// Returns the text that comes later in alphabetical order.
fn later<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a > b { a } else { b }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scopes() {
        let a = String::from("apple");
        let result;

        {
            let b = String::from("banana");
            result = later(&a, &b);
            // `b` is still alive here.
            assert_eq!(result, "banana");
        }
    }
}
