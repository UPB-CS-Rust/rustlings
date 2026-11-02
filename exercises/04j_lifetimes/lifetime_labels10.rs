const N: usize = 4;

// Returns the alphabetically last word that comes before `limit` (compared
// with `<`), or `fallback` if no word comes before `limit`.
// TODO: Add the missing lifetime labels. Only label what is needed!
fn last_before(words: [&str; N], limit: &str, fallback: &str) -> &str {
    let mut result = fallback;
    let mut found = false;

    for word in words {
        if word < limit && (!found || word > result) {
            result = word;
            found = true;
        }
    }

    result
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    const WORDS: [&str; N] = ["banana", "cherry", "blueberry", "apple"];

    #[test]
    fn found() {
        assert_eq!(last_before(WORDS, "c", "none"), "blueberry");
        assert_eq!(last_before(WORDS, "b", "none"), "apple");
        assert_eq!(last_before(WORDS, "z", "none"), "cherry");
    }

    #[test]
    fn fallback() {
        let fallback = String::from("nothing");
        assert_eq!(last_before(WORDS, "a", &fallback), "nothing");
    }

    #[test]
    fn short_lived_limit() {
        let result;
        {
            let limit = String::from("blue");
            result = last_before(WORDS, &limit, "none");
        } // `limit` is dropped here.
        assert_eq!(result, "banana");
    }
}
