fn main() {
    let red = Color::Red;
    print_color(red);
    let my_ip = IpAddr::V4(String::from("127.0.0.1"));
    print_ip(my_ip);

    let my_house = House::Number(2020);
    let my_house = House::Street("hello".to_owned());
    let my_house = House::Unknown;
    my_house.fn_print_house();
}
//定义一个颜色枚举
enum Color {
    Red,
    Green,
    Blue,
}
fn print_color(my_color: Color) {
    match my_color {
        Color::Red => println!("red"),
        Color::Green => println!("green"),
        Color::Blue => println!("blue"),
    }
}
//定义一个枚举，里面包含一个元组
enum IpAddr {
    V4(String),
    V6(String),
}
fn print_ip(ip: IpAddr) {
    match ip {
        IpAddr::V4(ip) => println!("v4: {}", ip),
        IpAddr::V6(ip) => println!("v6: {}", ip),
    }
}
enum House {
    Number(i32),
    Street(String),
    Unknown,
}

impl House {
    fn fn_print_house(&self) {
        match self {
            House::Number(num) => println!("c {}", num),
            House::Street(street) => println!("c {}", street),
            House::Unknown => println!("c unknown"),
        }
    }
}
