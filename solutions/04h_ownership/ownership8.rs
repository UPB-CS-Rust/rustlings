struct Letter {
    to: String,
    body: String,
}

fn main() {
    let template = String::from("Dear customer, thank you!");

    let letter1 = Letter {
        to: String::from("Ana"),
        // A deep copy with its own memory.
        body: template.clone(),
    };
    let letter2 = Letter {
        to: String::from("Bogdan"),
        // The last use can take the original.
        body: template,
    };

    println!("To {}: {}", letter1.to, letter1.body);
    println!("To {}: {}", letter2.to, letter2.body);
    assert_eq!(letter1.body, letter2.body);
}
