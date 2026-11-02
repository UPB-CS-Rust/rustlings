fn main() {
    let names = [
        String::from("Ana"),
        String::from("Bogdan"),
        String::from("Carmen"),
    ];

    // Iterating over `&names` borrows the elements.
    for name in &names {
        println!("Hello, {name}!");
    }

    assert_eq!(names[2], "Carmen");
}
