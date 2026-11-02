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

    // TODO: Fix the compiler error by changing only this line.
    let name = player.name;
    println!("The name is {name}");

    // Using a Copy field is fine even after the move above.
    assert_eq!(player.score, 7);

    // But using the whole player isn't.
    assert_eq!(describe(&player), 7);
}
