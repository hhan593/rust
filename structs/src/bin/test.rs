struct Point {
    x: i32,
    y: i32,
}
fn print_point(p: &Point) {
    println!("Point coordinates: ({}, {})", p.x, p.y);
}
fn main() {
    let mut point = Point { x: 10, y: 20 };
    // print_point(&point);

    let x = &mut point.x;

    // print_point(&point);
    // *x += 5
    println!("point.y = {}", point.y);
    //此时，point.y 的值仍然是 20，因为我们没有修改它。我们只是通过可变引用 x 修改了 point.x 的值，而 point.y 没有被修改，所以它仍然保持原来的值 20。
    //ponit 和point.x都不可以被使用，因为有一个可变引用，他们的所有权限都被 x 占用了，所以不能再使用 point 或 point.x 了，除非我们结束对 x 的使用。
}
