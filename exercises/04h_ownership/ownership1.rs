fn main() {
    // `i32` is a Copy type.
    let a = 5;
    let b = a;
    println!("a = {a}, b = {b}");

    // `String` is a move type.
    let s1 = String::from("hello");
    let s2 = s1;

    // TODO: Fix the compiler error without changing the lines above.
    println!("The text is: {s1}");
    assert_eq!(s2, "hello");
}
