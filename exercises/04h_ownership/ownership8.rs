struct Letter {
    to: String,
    body: String,
}

fn main() {
    let template = String::from("Dear customer, thank you!");

    // TODO: Fix the compiler error. Each letter needs its own copy of the text.
    let letter1 = Letter {
        to: String::from("Ana"),
        body: template,
    };
    let letter2 = Letter {
        to: String::from("Bogdan"),
        body: template,
    };

    println!("To {}: {}", letter1.to, letter1.body);
    println!("To {}: {}", letter2.to, letter2.body);
    assert_eq!(letter1.body, letter2.body);
}
