// 枚举是一种自定义数据类型，可以包含多个变体（variants），每个变体可以有不同的数据类型和数量。
// 枚举的定义使用 enum 关键字，变体之间用逗号分隔。每个变体可以是一个简单的标识符，也可以包含数据。
// 枚举的变体可以是单元变体（没有数据），元组变体（包含一个或多个匿名字段），或者结构变体（包含一个或多个命名字段）。
// 枚举的使用通常通过 match 表达式来进行模式匹配，根据不同的变体执行不同的代码逻辑。
enum Shape {
    Circle(f64),             // 圆形，包含一个 f64 类型的半径
    Rectangle(f64, f64),     // 矩形，包含两个 f64 类型的宽度和高度
    Triangle(f64, f64, f64), // 三角形，包含三个 f64 类型的边长
}

enum Color {
    Red,
    Green,
    Blue,
    Black,
}
impl Color {
    fn print_color(&self) {
        match self {
            Color::Red => println!("Red"),
            Color::Green => println!("Green"),
            Color::Blue => println!("Blue"),
            Color::Black => println!("Black"),
        }
    }
}

enum BuildingLocation {
    Number(i32),  // 包含一个 i32 类型的建筑编号
    Name(String), // 包含一个 String 类型的建筑名称
    Unknown,      // 不包含任何数据，表示未知位置
}
impl BuildingLocation {
    fn print_location(&self) {
        match self {
            BuildingLocation::Number(c) => println!("building number {}", c),
            BuildingLocation::Name(s) => println!("building name {}", s),
            BuildingLocation::Unknown => println!("unknown"),
        }
    }
}
fn main() {
    println!("enum 枚举!!!!");

    let circle = Shape::Circle(5.0);
    // let rectangle = Shape::Rectangle(4.0, 6.0);
    // let triangle = Shape::Triangle(3.0, 4.0, 5.0);
    let a = Color::Red;
    a.print_color();

    let house = BuildingLocation::Name("fast".to_string());
    let house2 = BuildingLocation::Number(1);
    let house3 = BuildingLocation::Unknown;
    house.print_location();
    house2.print_location();
    house3.print_location();
    // print_color(a);
    match circle {
        Shape::Circle(radius) => println!("Circle with radius: {}", radius),
        Shape::Rectangle(width, height) => {
            println!("Rectangle with width: {} and height: {}", width, height)
        }
        Shape::Triangle(a, b, c) => println!("Triangle with sides: {}, {}, {}", a, b, c),
    }
    let number = 5;
    match number {
        0 => println!("Zero"),
        1..=10 => println!("Between 1 and 10"),
        _ => println!("Greater than 10"),
    }

    //常见的枚举类型 Option<T> 和 Result<T, E>
}

//匹配模式
// match 表达式允许我们根据枚举的不同变体执行不同的代码逻辑。我们可以使用 match 来匹配枚举的变体，并提取其中的数据。
// 必须覆盖所有可能的变体，否则编译器会报错。我们可以使用 _ 模式来匹配所有未被显式处理的变体。
// 可以用_、..=、|等模式来匹配多个变体或忽略某些值。
pub enum Option<T> {
    Some(T), // 包含一个 T 类型的值
    None,    // 不包含任何值
}

pub enum Result<T, E> {
    Ok(T),  // 包含一个 T 类型的成功值
    Err(E), // 包含一个 E 类型的错误值
}
fn print_color(color: Color) {
    match color {
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
        _ => println!("Unknown color"),
    }
}
