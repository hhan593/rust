// 知识点：所有权转移 (Move) 与 Copy 语义
// p1 是 i32 类型（标量类型），实现了 Copy 特型，传递时会发生按值复制。
// p2 是 String 类型（堆分配类型），未实现 Copy，传递时会发生所有权转移（Move）。
fn move_func(p1: i32, p2: String) {
    println!("p1 is {}", p1);
    println!("p2 is {}", p2);
} // 函数结束时，p2 离开作用域，其底层的堆内存被释放。p1 只是一个副本，离开作用域无影响。

// 知识点：Copy 语义
fn print_value(value: i32) {
    println!("value is {}", value);
}

// 知识点：不可变借用 (Immutable Borrowing)
// 参数 s 是对 String 类型的不可变引用（&String）。
// 函数只获得了数据的读取权，并没有获取所有权，因此函数结束时不会释放 s 指向的内存。
fn string_func_borrow(s: &String) {
    // (*s) 是显式解引用，获取到实际的 String 数据，然后调用其方法。
    // （在实际开发中，Rust 会自动处理引用调用方法，写成 s.to_uppercase() 即可）
    println!("s is {}", (*s).to_uppercase());
}

// 知识点：结构体与派生宏
#[derive(Debug)] // 派生 Debug 特型，让结构体可以通过 "{:?}" 格式化打印
struct Point {
    x: i32,
    y: i32,
}

// 知识点：可变借用 (Mutable Borrowing) 与解引用
// 参数 point 是一个指向 Point 的可变引用（&mut Point）。
fn modify_point(point: &mut Point) {
    (*point).x += 2; // 显式解引用：先通过 * 追踪到实际数据，再访问 x 字段修改
    point.y += 2; // 隐式解引用（语法糖）：Rust 编译器会自动为结构体字段访问加上 *
}

fn main() {
    let num = 10;
    let str = "hello world".to_string(); // str 在堆上分配内存

    // 1. Move 发生在这里
    move_func(num, str);
    // 执行到这里时，str 的所有权已经被 "移动" 给了 move_func 的内部变量 p2。

    println!("num is {}", num); // num 是 i32（Copy 类型），传递给 move_func 的只是拷贝，原变量仍然有效。

    // println!("str is {}", str);
    // 【编译错误】因为 str 的所有权已丢失（Moved），Rust 为了保证内存安全（防止二次释放），禁止继续使用失效的变量。

    // 2. 借用与 Copy 测试
    let s = "hello world".to_string();
    print_value(num); // 再次拷贝 num 的值
    print_value(num); // 多次调用没问题，因为 i32 每次传参都是复制

    string_func_borrow(&s); // 传入 s 的引用（&s）。我们只是 "借" 出去让函数看一眼。
    println!("s is {}", s); // 可以正常打印，因为我们没有交出所有权，s 仍然有效。

    // 3. 可变借用测试
    // 要创建一个可变引用，原变量自身也必须是可变的（mut）
    let mut point = Point { x: 1, y: 2 };

    modify_point(&mut point); // 传入可变引用（&mut），允许函数内部修改数据

    // point 的值已经在 modify_point 中被原地修改了
    println!("point is {:?}", point); // 输出：point is Point { x: 3, y: 4 }
}
