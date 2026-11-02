// A string literal lives for the whole program.
fn default_name() -> &'static str {
    "guest"
}

// Returns `name`, or the default name if `name` is the banned name.
fn display_name<'a>(name: &'a str, banned: &str) -> &'a str {
    if name == banned { default_name() } else { name }
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names() {
        assert_eq!(default_name(), "guest");
        let name = String::from("Ana");
        assert_eq!(display_name(&name, "admin"), "Ana");
        assert_eq!(display_name("admin", "admin"), "guest");
    }
}
