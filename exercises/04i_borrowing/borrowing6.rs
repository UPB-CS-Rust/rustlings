fn main() {
    let names = [
        String::from("Ana"),
        String::from("Bogdan"),
        String::from("Carmen"),
    ];

    // TODO: Fix the compiler error by changing only the loop line.
    for name in names {
        println!("Hello, {name}!");
    }

    assert_eq!(names[2], "Carmen");
}
