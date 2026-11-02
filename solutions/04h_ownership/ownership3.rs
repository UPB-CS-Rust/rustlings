// A struct can be Copy if all its fields are Copy.
#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let mut p2 = p1;
    p2.x = 10;

    println!("p1 = ({}, {}), p2 = ({}, {})", p1.x, p1.y, p2.x, p2.y);
    assert_eq!(p1.x, 1);
    assert_eq!(p2.x, 10);
}
