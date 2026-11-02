struct Player {
    name: String,
    score: u32,
}

fn log_score(score: &mut u32, name: &str) {
    *score += 10;
    println!("{name} now has {score} points");
}

fn main() {
    let mut player = Player {
        name: String::from("Ferris"),
        score: 5,
    };

    // Borrowing two different fields at the same time is allowed.
    log_score(&mut player.score, &player.name);

    assert_eq!(player.score, 15);
}
