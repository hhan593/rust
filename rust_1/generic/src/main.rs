//泛型

struct Point<T> {
    x: T,
    y: T,
}
fn main() {
    let x = Point { x: 1, y: 2 };
    let y = Point { x: "12", y: "45" };
    println!("x: {}, y: {}", x.x, x.y);
}
