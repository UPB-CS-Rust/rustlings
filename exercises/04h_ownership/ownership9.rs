fn main() {
    let numbers = [1, 2, 3];
    let first_number = numbers[0]; // Fine: `i32` is Copy.

    let names = [
        String::from("Ana"),
        String::from("Bogdan"),
        String::from("Carmen"),
    ];

    // TODO: Fix the compiler error.
    let first_name = names[0];

    println!("{first_number} {first_name}");
    assert_eq!(first_name, "Ana");
    assert_eq!(names[1], "Bogdan");
}
