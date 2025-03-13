// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
#[derive(Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });
    let y_clone=y.clone();
    match y_clone {
        Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    y; // Fix without deleting this line.
}
