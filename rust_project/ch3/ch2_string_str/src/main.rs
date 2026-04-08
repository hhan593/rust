// --- 结构体定义 ---

// 定义结构体 Person
// 使用泛型生命周期 'a
// 含义：name 字段引用的字符串切片，其生命周期必须至少和 Person 实例一样长
// 这样可以防止出现“悬垂引用”（即 Person 还在，但 name 指向的字符串已经被释放了）
struct Person<'a> {
    name: &'a str,      // name 是一个字符串切片引用 (&str)，不拥有数据所有权
    age: i32,           // age 是一个 32 位整数
    occupation: String, // occupation 是一个 String 类型，拥有堆上数据的所有权
}

// --- 主函数 ---

fn main() {
    // =======================
    // 1. 字符串创建与所有权
    // =======================

    // 使用 String::from 将字符串字面量（&str）转换为堆上分配的 String
    // 此时 name 变量拥有这段字符串的所有权
    let name = String::from("Value C++");

    // 演示创建 String 的另外两种常见方式：
    let course = "Rust".to_string(); // 方式一：to_string()
    let course1 = "Rust".to_owned(); // 方式二：to_owned() (通常用于从借用中获取所有权)

    // =======================
    // 2. 字符串操作 (不可变性)
    // =======================

    // replace 方法不会修改原来的 `name` 变量
    // Rust 的 String 默认是不可变的。replace 会分配新内存，创建一个新的 String
    let new_name = name.replace("C++", "CPP");

    println!("{}", new_name); // 输出: Value CPP
    println!("{}", course); // 输出: Rust
    println!("{}", course1); // 输出: Rust

    // 验证原变量未被修改
    // 这证明了 Rust 的变量默认是不可变的，操作通常返回新值
    println!("{}", name); // 输出: Value C++

    // =======================
    // 3. 底层字节与转义
    // =======================

    // \x 是 Rust 的 ASCII 转义字符，后面跟两位十六进制数
    // \x52='R', \x75='u', \x73='s', \x74='t'
    let rust = "\x52\x75\x73\x74";

    println!("{rust}"); // 输出: Rust

    // =======================
    // 4. 迭代器 (Bytes vs Chars)
    // =======================

    // bytes(): 将字符串视为字节序列 (u8)。
    // {:?} 是 Debug 格式化，这里打印的是迭代器对象本身的调试信息
    println!("{:?}", rust.bytes()); // 输出: Bytes(['R', 'u', 's', 't'])

    // chars(): 将字符串视为 Unicode 字符序列。
    println!("{:?}", rust.chars()); // 输出: Chars(['R', 'u', 's', 't'])

    // 消费迭代器：遍历字符
    for c in rust.chars() {
        println!("{}", c);
        // 逐行输出: R, u, s, t
    }

    // =======================
    // 5. 结构体实例化
    // =======================

    let person = Person {
        name: "John", // 字符串字面量类型是 &'static str，满足生命周期 'a 的要求
        age: 30,
        occupation: String::from("Software Engineer"), // 拥有所有权的 String
    };

    // 手动格式化输出结构体字段
    println!("{} {} {}", person.name, person.age, person.occupation);
    // 输出: John 30 Software Engineer

    // =======================
    // 6. 函数调用与类型转换
    // =======================

    let data = "Hello, Rust!";
    let data2 = data.to_string(); // data2 是 String 类型

    // 传递 &String 给接受 &str 的函数
    // 这是一个“解引用强制多态” (Deref Coercion) 的例子
    // Rust 会自动把 &String 转换为 &str
    print_person(&data2);

    // 传递 &String 给接受 &String 的函数
    // 必须显式借用
    print_person_string(&data2);
}

// --- 辅助函数 ---

// 这个函数接受 &str
// 灵活性高：既可以接收字符串字面量，也可以接收 &String (通过自动转换)
fn print_person(data: &str) {
    println!("data is :{}", data);
}

// 这个函数只接受 &String
// 灵活性低：只能接收 String 的引用
fn print_person_string(data: &String) {
    println!("{}", data);
}
