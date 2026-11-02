struct Person {
    name: String,
    age: u32,
}

// TODO: Fix the compiler error. The result borrows from `person`.
fn name_of<'a, 'b>(person: &'a Person) -> &'b str {
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
