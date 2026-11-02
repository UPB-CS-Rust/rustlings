struct Player {
    name: String,
    score: u32,
}

const N: usize = 3;

// Gives `bonus` points to every player whose score is below `limit`.
// TODO: Fix the parameter type and implement the function.
fn give_bonus(players: [Player; N], limit: u32, bonus: u32) {}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bonus() {
        let mut players = [
            Player {
                name: String::from("Ana"),
                score: 10,
            },
            Player {
                name: String::from("Bogdan"),
                score: 50,
            },
            Player {
                name: String::from("Carmen"),
                score: 20,
            },
        ];

        // TODO: Fix the call.
        give_bonus(players, 30, 5);

        assert_eq!(players[0].score, 15);
        assert_eq!(players[1].score, 50);
        assert_eq!(players[2].score, 25);
        assert_eq!(players[1].name, "Bogdan");
    }
}
