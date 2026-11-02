struct Player {
    name: String,
    score: u32,
}

fn describe(player: &Player) -> u32 {
    println!("{} has {} points", player.name, player.score);
    player.score
}

fn main() {
    let player = Player {
        name: String::from("Bogdan"),
        score: 7,
    };

    // Borrow the field instead of moving it out of `player`.
    let name = &player.name;
    println!("The name is {name}");

    assert_eq!(player.score, 7);
    assert_eq!(describe(&player), 7);
}
