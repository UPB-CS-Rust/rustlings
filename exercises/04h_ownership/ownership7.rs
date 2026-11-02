fn main() {
    let first = String::from("salt");
    let second = String::from("pepper");

    let pair = (first, second);

    // TODO: Fix the compiler errors by accessing the texts through `pair`.
    println!("{first} and {second}");
    assert_eq!(first, "salt");
    assert_eq!(second, "pepper");
    assert_eq!(pair.0, "salt");
}
