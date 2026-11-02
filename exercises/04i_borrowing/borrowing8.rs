struct Player {
    name: String,
    score: u32,
}

// TODO: Fix the parameter types of the three functions. Use the most
// restrictive kind of reference that works for each function.

fn rename(player: Player, new_name: &str) {
    player.name = String::from(new_name);
}

fn add_points(player: Player, points: u32) {
    player.score += points;
}

fn summary(player: Player) -> u32 {
    println!("{}: {}", player.name, player.score);
    player.score
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game() {
        let mut player = Player {
            name: String::from("anonymous"),
            score: 0,
        };

        // TODO: Fix the calls.
        rename(player, "Ana");
        add_points(player, 20);
        add_points(player, 5);

        assert_eq!(summary(player), 25);
        assert_eq!(player.name, "Ana");
    }
}
