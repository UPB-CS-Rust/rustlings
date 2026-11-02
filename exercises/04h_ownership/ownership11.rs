fn send(letter: String) {
    println!("Sending: {letter}");
}

fn main() {
    let letter = String::from("Dear Ferris, ...");
    let urgent = false;

    if urgent {
        send(letter);
    }

    // TODO: Fix the compiler error.
    println!("Letter kept in the archive: {letter}");
}
