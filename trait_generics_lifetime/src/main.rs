#[derive(Debug)]
//#[derive(Debug)] 只能加在结构体（struct）、枚举（enum）或联合体（union）的定义上面，不能加在函数（fn）上面
struct Point<T, U> {
    X: T,
    Y: U,
}
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let largest = latest(&number_list);
    println!("The largest number is {largest}");

    let integer = Point { X: 5, Y: '1' };
    println!("{:?}", integer.X);
    println!("{:?}", integer.Y);
}
//抽象出lastest函数

fn latest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for number in list {
        if number > largest {
            largest = number;
        }
    }

    largest
}
