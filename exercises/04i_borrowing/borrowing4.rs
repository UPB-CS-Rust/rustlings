fn shout(text: &mut String) {
    *text = String::from("HELLO");
}

fn main() {
    let mut text = String::from("hello");

    // TODO: Fix the compiler error by moving one line.
    let first = &text;
    shout(&mut text);
    println!("Before: {first}");

    println!("After: {text}");
    assert_eq!(text, "HELLO");
}
