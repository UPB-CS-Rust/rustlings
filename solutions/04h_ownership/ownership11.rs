// Borrowing the letter means it can still be used afterwards.
fn send(letter: &str) {
    println!("Sending: {letter}");
}

fn main() {
    let letter = String::from("Dear Ferris, ...");
    let urgent = false;

    if urgent {
        send(&letter);
    }

    println!("Letter kept in the archive: {letter}");
}
