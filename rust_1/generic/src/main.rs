//泛型

struct Point<T> {
    x: T,
    y: T,
}
impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
    fn get(&self) -> (&T, &T) {
        (&self.x, &self.y)
    }
}
fn main() {
    let x = Point { x: 1, y: 2 };
    let y = Point { x: "12", y: "45" };

    println!("x: {}, y: {}", x.x, x.y);
    println!("y: {}, y: {}", y.x, y.y);
    println!("{:?}", swap(2, 1));
    println!("{:?}", swap("12", "45"));
    let result = swap(0, 1);
    println!("{:?}", result);
    let point = Point::new(100, 2);
    let res = point.get();
    println!("{:?}", res);
}

fn swap<T>(a: T, b: T) -> (T, T) {
    (b, a)
}
