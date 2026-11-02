fn main() {
    let s = String::from("borrowed");

    // TODO: Fix the compiler error by reordering the lines (don't remove any).
    let r = &s;
    let moved = s;
    println!("Reference: {r}");

    assert_eq!(moved, "borrowed");
}
