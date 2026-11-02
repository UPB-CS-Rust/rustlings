// TODO: Fix the compiler errors by adding lifetime labels.
struct Excerpt {
    text: &str,
    page: u32,
}

fn show(excerpt: &Excerpt) {
    println!("Page {}: {}", excerpt.page, excerpt.text);
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn excerpt() {
        let book = String::from("Call me Ishmael. Some years ago...");
        let excerpt = Excerpt {
            text: &book[..16],
            page: 1,
        };
        show(&excerpt);
        assert_eq!(excerpt.text, "Call me Ishmael.");
    }
}
