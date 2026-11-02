// Each reference has its own label.
struct Pair<'a, 'b> {
    first: &'a str,
    second: &'b str,
}

// The result is only tied to the second field.
fn second<'b>(pair: &Pair<'_, 'b>) -> &'b str {
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
