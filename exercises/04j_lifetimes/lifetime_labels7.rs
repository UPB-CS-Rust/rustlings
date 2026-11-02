// TODO: The test doesn't compile. Give each field its own label, and make
// `second` return a reference with the label of the second field.
// Don't change the test!
struct Pair<'a> {
    first: &'a str,
    second: &'a str,
}

fn second<'a>(pair: &Pair<'a>) -> &'a str {
    pair.second
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_lifetimes() {
        let long = String::from("lives long");
        let result;

        {
            let short = String::from("short");
            let pair = Pair {
                first: &short,
                second: &long,
            };
            println!("{} / {}", pair.first, pair.second);
            result = second(&pair);
        } // `short` and `pair` are dropped here.

        assert_eq!(result, "lives long");
    }
}
