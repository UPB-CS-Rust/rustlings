struct Person {
    name: String,
    age: u32,
}

// With a single reference parameter, the labels can be left out (elision).
// The full form would be: fn name_of<'a>(person: &'a Person) -> &'a str
fn name_of(person: &Person) -> &str {
    &person.name
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name() {
        let p = Person {
            name: String::from("Ana"),
            age: 30,
        };
        assert_eq!(name_of(&p), "Ana");
        assert_eq!(p.age, 30);
    }
}
