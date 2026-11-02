// Return the owned `String`: the caller becomes its owner.
fn make_title(word: &str) -> String {
    String::from(word)
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn title() {
        let title = make_title("Rust");
        assert_eq!(title, "Rust");
    }
}
