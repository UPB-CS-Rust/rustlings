// Returns `text`. The separator is only printed.
// The result only comes from `text`, so only `text` shares its label.
fn announce<'a>(text: &'a str, separator: &str) -> &'a str {
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
