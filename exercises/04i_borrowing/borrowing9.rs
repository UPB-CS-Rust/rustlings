struct Player {
    name: String,
    score: u32,
}

const N: usize = 3;

// Returns the name of the player with the highest score (the first one in
// case of a tie). The name must not be moved out of the array!
fn best_player(players: &[Player; N]) -> &str {
    // TODO: Implement this function.
    ""
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn best() {
        let players = [
            Player {
                name: String::from("Ana"),
                score: 10,
            },
            Player {
                name: String::from("Bogdan"),
                score: 30,
            },
            Player {
                name: String::from("Carmen"),
                score: 30,
            },
        ];

        assert_eq!(best_player(&players), "Bogdan");
        // The array still owns all names.
        assert_eq!(players[0].name, "Ana");
    }
}
