struct Quote<'a> {
    text: &'a str,
}

// Returns a quote of `a` if `first` is true, otherwise of `b`.
// TODO: Fix the compiler error by adding lifetime labels.
fn pick(a: &str, b: &str, first: bool) -> Quote {
    if first { Quote { text: a } } else { Quote { text: b } }
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn picking() {
        let x = String::from("To be");
        let y = String::from("or not to be");
        assert_eq!(pick(&x, &y, true).text, "To be");
        assert_eq!(pick(&x, &y, false).text, "or not to be");
    }
}
