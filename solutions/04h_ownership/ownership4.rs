// `String` isn't Copy, so `Player` can't be Copy either.
struct Player {
    name: String,
    score: u32,
}

fn main() {
    let p1 = Player {
        name: String::from("Ana"),
        score: 42,
    };

    // Copy the Copy field out before `p1` is moved.
    let old_score = p1.score;
    let p2 = p1;

    println!("{} has {} points", p2.name, p2.score);
    assert_eq!(old_score, 42);
}
