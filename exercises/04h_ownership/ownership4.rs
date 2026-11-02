// TODO: Fix the compiler errors. `Player` can't be `Copy`. Remove the line
// below and then fix `main`.
#[derive(Clone, Copy)]
struct Player {
    name: String,
    score: u32,
}

fn main() {
    let p1 = Player {
        name: String::from("Ana"),
        score: 42,
    };

    let p2 = p1;
    let old_score = p1.score;

    println!("{} has {} points", p2.name, p2.score);
    assert_eq!(old_score, 42);
}
