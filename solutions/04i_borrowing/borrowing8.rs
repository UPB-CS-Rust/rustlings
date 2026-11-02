struct Player {
    name: String,
    score: u32,
}

fn rename(player: &mut Player, new_name: &str) {
    player.name = String::from(new_name);
}

fn add_points(player: &mut Player, points: u32) {
    player.score += points;
}

fn summary(player: &Player) -> u32 {
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

        rename(&mut player, "Ana");
        add_points(&mut player, 20);
        add_points(&mut player, 5);

        assert_eq!(summary(&player), 25);
        assert_eq!(player.name, "Ana");
    }
}
