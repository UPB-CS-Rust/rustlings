// TODO: Make this function give the ownership of `name` back to the caller.
fn greet(name: String) {
    println!("Hello, {name}!");
}

fn main() {
    let name = String::from("Ferris");

    // TODO: Fix the compiler error.
    greet(name);
    greet(name);

    assert_eq!(name, "Ferris");
}
