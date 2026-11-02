fn main() {
    let first = String::from("salt");
    let second = String::from("pepper");

    let pair = (first, second);

    // The texts were moved into the tuple.
    println!("{} and {}", pair.0, pair.1);
    assert_eq!(pair.0, "salt");
    assert_eq!(pair.1, "pepper");
}
