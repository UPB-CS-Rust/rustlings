// TODO: Change the parameter so that the function only borrows the text.
fn print_twice(text: String) {
    println!("{text} {text}");
}

fn main() {
    let message = String::from("Rust!");

    for _ in 0..3 {
        // TODO: Fix the call.
        print_twice(message);
    }

    assert_eq!(message, "Rust!");
}
