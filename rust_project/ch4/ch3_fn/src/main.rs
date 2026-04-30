// 定义一个加法函数，接收两个 i32 类型的参数，返回一个 i32 类型的值
fn add(x: i32, y: i32) -> i32 {
    return x + y; // 使用 return 关键字显式返回 x 和 y 的和
}


// 1. 为什么 change_i32(x) 没有改变 x 的值？（Copy 语义）
// 在 Rust 中，像 i32、f64、bool 这种大小固定且存储在栈上的基础类型，默认都实现了 Copy 特质。当你把 x 传入 change_i32 时，Rust 并没有转移 x 的所有权，而是复制了一份全新的数据交给函数。
// 函数内部修改的仅仅是那份副本，main 函数里的原变量 x 毫发无损。
// 2. 为什么 modify_i32(&mut x) 改变了 x 的值？（可变借用）
// 这次你传入的是 &mut x，也就是 x 在内存中的真实地址。函数通过解引用操作符 *，直接找到了 main 函数里那个 x 的内存位置，并修改了它的值。
// 这就好比你把家里的钥匙（引用）给了朋友，朋友可以直接进你家（内存）搬东西，你回家时自然会看到变化。
// 3. 为什么结构体 s 被传入函数后，还能继续访问？（自定义类型的 Copy）
// 在 Rust 中，普通的自定义结构体默认是不实现 Copy 的。如果去掉代码最上面的 #[derive(Copy, Clone)]，你的代码会在 print_point(s) 处报错，提示 s 的所有权已经被“移动（move）”进函数里了，后面无法再使用。
// 但因为你加上了 #[derive(Copy, Clone)]，并且 Point 里的字段（i32）也都支持 Copy，Rust 就会像对待 i32 一样对待 Point。
// 每次调用 print_point(s) 时，Rust 都会在底层悄悄地复制一份全新的 Point 传进去。所以 main 函数里的 s 依然完好无损地保留着所有权，你可以无限次地使用它。
// 定义一个试图修改 i32 的函数
// 注意：这里的 mut x 表示在函数内部，x 是一个可变的变量
fn change_i32(mut x: i32) {
    x = 100 + 1; // 修改的是函数内部这个局部变量 x 的值，不会影响外部
}

// 定义一个通过可变引用修改 i32 的函数
// 参数 x 是一个指向 i32 的可变引用（&mut i32），它持有外部变量的内存地址
fn modify_i32(x: &mut i32) {
    *x = 100 + 1; // 使用解引用操作符 *，直接修改引用指向的内存地址中的值
}

// 定义一个结构体 Point，包含两个 i32 字段
// #[derive(Copy, Clone)] 是关键！它让 Point 自动实现了 Copy 特质
// 这意味着 Point 在传递时会发生“复制”而不是“所有权移动”
#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}<websource>source_group_web_1</websource>

fn main() {
    let a = 1; // 声明不可变变量 a
    let b = 2; // 声明不可变变量 b
    let c = add(a, b); // 调用 add 函数，i32 类型发生复制传参，计算结果赋值给 c
    println!("c: {c}"); // 打印 c 的值，输出 3

    let mut x = 10; // 声明一个可变的变量 x，初始值为 10
    change_i32(x); // 调用函数。因为 i32 实现了 Copy，这里传进去的是 x 的一个副本
    println!("x: {x}"); // 打印 x，结果依然是 10<websource>source_group_web_2</websource>。因为函数里改的是副本，不影响原变量

    modify_i32(&mut x); // 调用函数，传入 x 的可变引用（地址）
    println!("x: {x}"); // 打印 x，结果变成了 101。因为函数通过地址直接修改了原变量

    // 声明一个 Point 结构体实例 s
    let s = Point { x: 1, y: 2 };
    print_point(s); // 调用函数。因为 Point 实现了 Copy，这里传进去的是 s 的一个副本
    // 原变量 s 的所有权并没有转移，依然属于 main 函数，所以后面还能继续使用！
    println!("s.x: {}", s.x); // 打印 s.x，正常输出 1
    print_point(s); // 再次调用，依然传的是 s 的副本，完全合法
}

// 接收一个 Point 类型的参数（按值传递）
fn print_point(point: Point) {
    // 打印 point 的 x 和 y 坐标
    println!("point: ({}, {})", point.x, point.y);
} // 函数结束，参数 point（也就是传入的那个副本）在这里被销毁