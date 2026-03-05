// fn main() {
//     let r;
//     {
//         let x = 5;
//         r = &x;
//     }
//     此时x的生命周期已经结束，该块内存已经被释放，所以引用r将无法使用
//     println!("{}", r);
// }

// fn main() {
//     let x = 5;
//     let r = &x;
//     println!("{}", r);
// }

fn main() {
    // string1 是一个拥有所有权的 String，分配在堆上
    let string1 = String::from("abcd");

    // string2 是一个字符串字面量，类型为 &str，它具有 'static 生命周期（存活于整个程序运行期间）
    let string2 = "xyz";

    // 调用 longest 函数，传入两个字符串的切片（引用）
    // 注意：在没有修改 longest 函数签名之前，这里会编译报错
    let result = longest(string1.as_str(), string2);

    // 打印结果
    println!("The longest string is {}", result);
}

/**
 * 报错原因分析：
 * 这个函数违反了 Rust 的“生命周期省略规则（Lifetime Elision Rules）”。
 * 编译器看到返回类型是 &str，它不知道这个引用的有效期是跟着 x 走，还是跟着 y 走。
 * 如果没有明确标注，编译器无法保证返回的引用在 main 函数中使用时依然有效。
 */
// 修正方式：加上生命周期标注 <'a>
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // fn longest(x: &str, y: &str) -> &str {
    // Rust 建议：if 条件不需要加小括号 ()
    if x.len() > y.len() {
        x // 如果返回 x，那么返回值的生命周期必须涵盖 x 的作用域
    } else {
        y // 如果返回 y，那么返回值的生命周期必须涵盖 y 的作用域
    }
}
fn shortest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() < y.len() { x } else { y }
}
