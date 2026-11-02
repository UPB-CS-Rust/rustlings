fn shout(text: &mut String) {
    *text = String::from("HELLO");
}

fn main() {
    let mut text = String::from("hello");

    let first = &text;
    println!("Before: {first}"); // Last use of the shared reference.
    shout(&mut text);

    println!("After: {text}");
    assert_eq!(text, "HELLO");
}
