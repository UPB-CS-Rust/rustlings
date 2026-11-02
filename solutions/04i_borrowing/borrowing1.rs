// `&str` accepts both `&String` and string literals.
fn is_admin(name: &str) -> bool {
    name == "admin"
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn admins() {
        let user = String::from("admin");
        assert!(is_admin(&user));
        assert!(is_admin(&user));
        assert_eq!(user, "admin");
    }

    #[test]
    fn literals() {
        assert!(!is_admin("guest"));
    }
}
