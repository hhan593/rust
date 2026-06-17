// 【知识点总结】
// 1. 枚举 (Enum)：允许通过列举所有可能的变体 (Variant) 来定义一个类型。
// 2. 携带数据的枚举：Rust 枚举非常灵活，其变体内部可以包含不同类型和数量的数据（如整型 i32、动态字符串 String 等）。
// 3. 模式匹配 (match)：Rust 中强大的控制流运算符，用于将值与一系列模式进行比较，并提取内部绑定的数据。必须穷尽所有可能 (Exhaustive)。
// 4. 实现方法 (impl)：在 Rust 中，不仅仅是结构体，枚举也可以拥有自己的实现块 (impl block) 并定义方法。
// 5. 变量遮蔽 (Shadowing)：在同一个作用域内，可以使用相同的变量名配合 let 重新声明变量，新变量会“遮蔽”（覆盖）掉旧变量的值和类型。

fn main() {                                             // 定义程序的入口函数 main (知识点：入口函数)
    let red = Color::Red;                               // 实例化 Color 枚举的 Red 变体，并将其绑定到不可变变量 red (知识点：基本枚举实例化)
    print_color(red);                                   // 调用自定义函数 print_color，将 red 传入进行模式匹配和打印 (知识点：函数调用)

    let my_ip = IpAddr::V4(String::from("127.0.0.1"));  // 实例化包含 String 数据的 IpAddr::V4 变体，绑定到 my_ip 变量 (知识点：携带数据的枚举实例化，String::from 分配动态字符串)
    print_ip(my_ip);                                    // 调用 print_ip 函数，传递 my_ip 进行模式匹配和数据打印 (知识点：函数调用)

    let my_house = House::Number(2020);                 // 实例化包含 i32 整型数据的 House::Number 变体，绑定到 my_house 变量 (知识点：携带数据的枚举实例化)
    let my_house = House::Street("hello".to_owned());   // 再次使用 let 声明同名变量，实例化 House::Street 变体，此时旧的 my_house 被遮蔽 (知识点：变量遮蔽 Shadowing，.to_owned() 转换为 String)
    let my_house = House::Unknown;                      // 第三次使用 let 声明同名变量，实例化无数据的 Unknown 变体，再次遮蔽 (知识点：变量遮蔽 Shadowing，无数据枚举变体)
    my_house.fn_print_house();                          // 调用为 House 枚举实例关联的方法 fn_print_house (知识点：枚举方法调用)
}                                                       // main 函数结束，局部变量离开作用域被销毁 (知识点：作用域结束，内存释放)

//定义一个颜色枚举                                        // 原代码注释：定义基础枚举
enum Color {                                            // 定义名为 Color 的枚举类型 (知识点：枚举 enum 定义)
    Red,                                                // 定义枚举变体 Red，不携带任何附加数据 (知识点：无数据枚举变体)
    Green,                                              // 定义枚举变体 Green，不携带任何附加数据 (知识点：无数据枚举变体)
    Blue,                                               // 定义枚举变体 Blue，不携带任何附加数据 (知识点：无数据枚举变体)
}                                                       // 结束 Color 枚举定义作用域 (知识点：语法与作用域)

fn print_color(my_color: Color) {                       // 定义函数 print_color，接收一个 Color 类型的参数 my_color (知识点：函数定义，枚举类型参数)
    match my_color {                                    // 使用 match 表达式对传入的 my_color 进行穷尽匹配 (知识点：match 控制流运算符)
        Color::Red => println!("red"),                  // 若匹配到 Color::Red 变体，则执行胖箭头 (=>) 后的语句打印 "red" (知识点：match 分支结构)
        Color::Green => println!("green"),              // 若匹配到 Color::Green 变体，则打印 "green" (知识点：match 模式匹配)
        Color::Blue => println!("blue"),                // 若匹配到 Color::Blue 变体，则打印 "blue" (知识点：match 穷尽性，覆盖了所有变体)
    }                                                   // match 表达式作用域结束 (知识点：match 语句块)
}                                                       // print_color 函数结束 (知识点：作用域)

//定义一个枚举，里面包含一个元组                             // 原代码注释：实际上是携带匿名参数的变体，类似元组结构体
enum IpAddr {                                           // 定义名为 IpAddr 的枚举类型 (知识点：枚举定义)
    V4(String),                                         // 定义枚举变体 V4，要求实例化时必须包裹一个 String 类型的数据 (知识点：携带单一数据的枚举变体)
    V6(String),                                         // 定义枚举变体 V6，要求实例化时必须包裹一个 String 类型的数据 (知识点：携带单一数据的枚举变体)
}                                                       // 结束 IpAddr 枚举定义作用域 (知识点：语法与作用域)

fn print_ip(ip: IpAddr) {                               // 定义 print_ip 函数，接收 IpAddr 类型参数 (知识点：函数定义)
    match ip {                                          // 使用 match 表达式匹配 ip 参数 (知识点：match 控制流)
        IpAddr::V4(ip) => println!("v4: {}", ip),       // 匹配 V4 变体，并将其包裹的 String 值提取/解构绑定到局部变量 ip 中进行打印 (知识点：模式匹配与数据解构提取)
        IpAddr::V6(ip) => println!("v6: {}", ip),       // 匹配 V6 变体，将其内部数据提取到局部变量 ip 中并打印 (知识点：模式匹配与数据解构提取)
    }                                                   // match 表达式作用域结束 (知识点：match 语句块)
}                                                       // print_ip 函数结束 (知识点：作用域)

enum House {                                            // 定义名为 House 的枚举类型，展示变体可以携带不同类型的数据 (知识点：混合类型数据变体的枚举)
    Number(i32),                                        // 变体 Number 携带一个 32 位有符号整型 (i32) 数据 (知识点：枚举变体，整型数据)
    Street(String),                                     // 变体 Street 携带一个动态字符串 (String) 数据 (知识点：枚举变体，堆字符串数据)
    Unknown,                                            // 变体 Unknown 不携带任何附加数据 (知识点：无数据枚举变体)
}                                                       // 结束 House 枚举定义作用域 (知识点：语法与作用域)

impl House {                                            // 为 House 枚举开启一个实现块 (impl block)，用于为其附带特定的行为/方法 (知识点：impl 块与枚举)
    fn fn_print_house(&self) {                          // 定义实例方法 fn_print_house，接收 &self（自身不可变引用），表示只读不修改 (知识点：实例方法定义，&self 引用)
        match self {                                    // 对传入的枚举实例引用 (&self) 进行 match 匹配 (知识点：针对引用的 match 匹配)
            House::Number(num) => println!("c {}", num), // 匹配 Number 变体，提取其内部的数字到变量 num 并打印 (知识点：模式匹配解构)
            House::Street(street) => println!("c {}", street), // 匹配 Street 变体，提取其内部的字符串到变量 street 并打印 (知识点：模式匹配解构)
            House::Unknown => println!("c unknown"),    // 匹配 Unknown 变体，由于没有数据，直接打印对应的信息 (知识点：无数据变体匹配)
        }                                               // match 表达式作用域结束 (知识点：match 语句块)
    }                                                   // 方法 fn_print_house 结束 (知识点：作用域)
}                                                       // House 的 impl 块结束 (知识点：作用域)