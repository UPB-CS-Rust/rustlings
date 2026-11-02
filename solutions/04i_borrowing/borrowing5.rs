fn main() {
    let s = String::from("borrowed");

    let r = &s;
    println!("Reference: {r}"); // Last use of the borrow.
    let moved = s; // Now `s` can be moved.

    assert_eq!(moved, "borrowed");
}
