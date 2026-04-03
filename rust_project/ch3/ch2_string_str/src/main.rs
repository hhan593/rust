// 定义结构体 Person
// 使用泛型生命周期 'a，表示 name 字段引用的字符串切片，其生命周期至少和 Person 实例一样长
struct Person<'a> {
    name: &'a str,      // name 是一个字符串切片引用 (&str)，不拥有数据所有权
    age: i32,           // age 是一个 32 位整数
    occupation: String, // occupation 是一个 String 类型，拥有堆上数据的所有权
}

fn main() {
    // --- 字符串操作部分 ---

    // 1. 创建 String 类型
    // 使用 String::from 将字符串字面量（&str）转换为堆上分配的 String
    // 这样创建的变量拥有所有权，可以被修改（除非声明为不可变）
    let name = String::from("Value C++");

    // 演示创建 String 的三种常见方式：
    // - String::from: 显式转换
    // - to_string(): 通用 trait 方法
    // - to_owned(): 通用 trait 方法（通常用于从借用中获取所有权）
    let course = "Rust".to_string(); // 方式一
    let course1 = "Rust".to_owned(); // 方式二

    // 2. 字符串操作 (不可变性演示)
    // replace 方法并不会修改原来的 `name` 变量
    // 而是根据规则创建了一个**新的**字符串值，并将其绑定到 `new_name` 上
    // 原来的 `name` 依然保持不变（Rust 的默认不可变性原则）
    let new_name = name.replace("C++", "CPP");

    // 打印结果
    // new_name 是修改后的新值
    println!("{}", new_name); // 输出: Value CPP

    // course 和 course1 是我们创建的两个 Rust 字符串
    println!("{}", course); // 输出: Rust
    println!("{}", course1); // 输出: Rust

    // 关键点：这里的 name 依然是 "Value C++"
    // 证明了上面的 replace 操作没有“消耗”或改变原变量
    println!("{}", name); // 输出: Value C++

    // 3. ASCII 十六进制字面量
    // \x 是 Rust 的转义字符，后面跟两位十六进制数代表一个 ASCII 字符
    // \x52 = 'R', \x75 = 'u', \x73 = 's', \x74 = 't'
    let rust = "\x52\x75\x73\x74";

    // 打印字符串本身
    println!("{rust}"); // 输出: Rust

    // 4. 底层字节与字符分析
    // bytes(): 返回一个迭代器，将字符串视为字节序列 (u8)
    // {:?} 是 Debug 格式化，打印迭代器的调试信息（如 "Bytes(...)")
    println!("{:?}", rust.bytes());

    // chars(): 返回一个迭代器，将字符串视为 Unicode 字符序列
    // {:?} 打印迭代器的调试信息
    println!("{:?}", rust.chars());

    // 5. 迭代器消费
    // 使用 for 循环遍历 chars() 迭代器
    // 将 "Rust" 拆分成单个字符并逐行打印
    for c in rust.chars() {
        println!("{}", c);
        // 输出:
        // R
        // u
        // s
        // t
    }

    // --- 结构体部分 ---

    // 6. 实例化结构体 Person
    // name: "John" 是字符串字面量 (&str)，其生命周期是 'static
    // age: 30 是整数
    // occupation: String::from(...) 创建了一个拥有所有权的 String
    let person = Person {
        name: "John",
        age: 30,
        occupation: String::from("Software Engineer"),
    };

    // 7. 打印结构体字段
    // 直接访问结构体的字段进行打印
    // 注意：这里没有使用 {:?} (Debug trait)，而是手动格式化输出
    println!("{} {} {}", person.name, person.age, person.occupation);
    // 输出: John 30 Software Engineer
    let data = "Hello, Rust!";
    let data2 = data.to_string();

    print_person(&data2); // 传递 &String，函数接受 &str

    print_person_string(&data2); //但是这个函数只能接受&String
}
//函数

//这个函数可以传递&String 和 &str
fn print_person(data: &str) {
    // println!("{} {} {}", person.name, person.age, person.occupation);
    println!("data is :{}", data);
}

//这个函数可以传递&String
fn print_person_string(data: &String) {
    println!("{}", data);
}
