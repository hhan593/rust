// fn read(y: bool) {
//     if y {
//         println!("y is true");
//     } else {
//         println!("y is false");
//     }
// }

/// 主函数演示 Rust 所有权机制
///
/// 该函数通过创建字符串变量并传递给其他函数，
/// 展示了 Rust 中所有权转移的概念
fn main() {
    // let x = true;
    // read(x);

    let m1 = String::from("hello");
    // let m2 = m1; // m1 的所有权被转移到 m2，m1 不再有效
    // println!("m1: {}", m1); // 这行会导致编译  ^^ help: if this is intentional, prefix it with an underscore: `_m2`
    let m2 = String::from("world");

    println!("m1: {}, m2: {}", m1, m2); // 这行会编译成功，因为 m1 和 m2 都是有效的字符串
    println!("m1的地址：{:p}, m2的地址 {:p},", &m1, &m2); // 这行会编译成功，因为 m1 和 m2 都是有效的字符串m2);
                                                          // greet(m1, m2); // m1 和 m2 的所有权被转移到 greet 函数中，之后 m1 和 m2 不再有效
    greet_ref(&m1, &m2); // 通过引用传递 m1 和 m2 的地址，所有权没有转移，m1 和 m2 仍然有效,因为引用释放的时候并没有指向堆内存的数据
    println!("m1: {}, m2: {}", m1, m2); // 这行会导致编译错误，因为 m1 和 m2 的所有权已经被转移到 greet 函数中，无法再使用它们 就会出现未定义行为 ，可以使用引用解决
}

// 辅助函数

/// greet - 打印两个字符串的函数
///
/// # 参数
///
/// * `g1` - 第一个字符串，类型为 String
/// * `g2` - 第二个字符串，类型为 String
///
/// 该函数接收两个 String 参数的所有权，并在函数结束时释放这两个字符串
fn greet(g1: String, g2: String) {
    println!("g1: {}, g2: {}", g1, g2);
}
// 引用解决函数

fn greet_ref(g1: &String, g2: &String) {
    println!("g1: {}, g2: {}", g1, g2);
}
// | 特性 | `greet(m1, m2)` (所有权转移) | `greet_ref(&m1, &m2)` (借用) |
// | :--- | :--- | :--- |
// | 参数类型 | `String` | `&String` (或 `&str`) |
// | 所有权 | 转移到函数内部 | 保留在调用者 (`main`) 手中 |
// | 函数结束后 | `m1`, `m2` 失效 (不可再用) | `m1`, `m2` 依然有效 |
// | 返回值 | 通常不需要返回 (除非要还回去) | 不需要返回 (只是临时看看) |
// | 性能 | 可能涉及内存分配/释放 | 极快 (只是传递指针) |
