// Borrowing doesn't take ownership, so the function can be called many times.
fn print_twice(text: &str) {
    println!("{text} {text}");
}

fn main() {
    let message = String::from("Rust!");

    for _ in 0..3 {
        print_twice(&message);
    }

    assert_eq!(message, "Rust!");
}
