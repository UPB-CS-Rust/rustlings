// Returning the parameter moves the ownership back to the caller.
fn greet(name: String) -> String {
    println!("Hello, {name}!");
    name
}

fn main() {
    let name = String::from("Ferris");

    let name = greet(name);
    let name = greet(name);

    assert_eq!(name, "Ferris");
}
