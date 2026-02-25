struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let mut a = Point { x: 10, y: 20 };
    a.x += 1;
    let b = Point { y: 2, ..a };

    a.x += 1;
    println!("{}", b.x);
}
