// TODO: Change the parameter type so that the function only borrows.
fn is_admin(name: String) -> bool {
    name == "admin"
}

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn admins() {
        let user = String::from("admin");
        // TODO: Fix the calls.
        assert!(is_admin(user));
        assert!(is_admin(user));
        assert_eq!(user, "admin");
    }

    #[test]
    fn literals() {
        assert!(!is_admin("guest"));
    }
}
