struct Player {
    name: String,
    score: u32,
}

// TODO: This function only needs the player's name. Change its second
// parameter and its body accordingly.
fn log_score(score: &mut u32, player: &Player) {
    *score += 10;
    println!("{} now has {} points", player.name, score);
}

fn main() {
    let mut player = Player {
        name: String::from("Ferris"),
        score: 5,
    };

    // TODO: Fix the call.
    log_score(&mut player.score, &player);

    assert_eq!(player.score, 15);
}
