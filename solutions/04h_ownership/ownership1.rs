fn main() {
    // `i32` is a Copy type.
    let a = 5;
    let b = a;
    println!("a = {a}, b = {b}");

    // `String` is a move type.
    let s1 = String::from("hello");
    let s2 = s1;

    // `s1` was moved into `s2`. Only `s2` owns the text now.
    println!("The text is: {s2}");
    assert_eq!(s2, "hello");
}
