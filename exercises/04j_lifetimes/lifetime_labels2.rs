// Returns `text`. The separator is only printed.
// TODO: The test doesn't compile. Fix the labels so that the result is only
// tied to `text`. Don't change the test!
fn announce<'a>(text: &'a str, separator: &'a str) -> &'a str {
    println!("{separator} {text} {separator}");
    text
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn result_outlives_separator() {
        let text = String::from("Breaking news");
        let result;

        {
            let separator = String::from("***");
            result = announce(&text, &separator);
        } // `separator` is dropped here.

        assert_eq!(result, "Breaking news");
    }
}
