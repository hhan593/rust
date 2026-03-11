// rust 中闭包捕获变量的方式对应三个 trait：`Fn`、`FnMut`、`FnOnce`。编译器根据闭包如何使用环境变量来自动推断。
// fn main() {
//Fn --不可变借用闭包
//     let config = vec![1, 2, 3];

//     let print_config = || {
//         println!("配置：{:?}", config);
//         println!("配置长度：{}", config.len())
//     };
//     print_config();
//     print_config();

//     println!("config 仍然存在: {:?}", config);
// }

// 2.2 FnMut — 可变借用
// fn main() {
//     let mut numbers = vec![1, 2, 3];

//     let mut add = |x| {
//         numbers.push(x);
//         println!("添加{},列表现在有{}个元素", x, numbers.len())
//     };

//     add(5);
//     add(6);

//     // --- 解决方案：在最后一次使用 add 之后，将其丢弃 ---
//     drop(add); // 显式地释放闭包，从而释放它对 numbers 的可变借用

//     // 现在，numbers 不再被借用，可以安全地访问它
//     println!("列表现在有{}个元素", numbers.len());

//     // 注意：如果在这里再次调用 add(7); 会导致编译错误，
//     // 因为 numbers 的所有权可能已经被 move 到了闭包里（取决于闭包的具体实现细节），
//     // 并且在 drop(add) 后，add 已经无效了。
// }

// FnOnce — 所有权转移 ：闭包捕获变量的所有权，只能调用一次。
fn main() {
    let s = String::from("hello");

    // 这个闭包获取 s 的所有权
    let consume = || {
        let upper = s.to_uppercase();
        println!("消费: {}", upper);
        drop(s); // s 在闭包中被消费
    };

    consume();
    // consume();  // 错误！FnOnce 只能调用一次
    // println!("{}", s);  // 错误！s 的所有权已被转移
}

// 三种 Trait 对比表
// | Trait    | 捕获方式    | 可调用次数 | 使用场景     |
// | -------- | -----      | -----      | -------- |
// | `Fn`     | 不可变借用  | 无限       | 只读取环境变量  |
// | `FnMut`  | 可变借用    | 无限       | 修改环境变量   |
// | `FnOnce` | 所有权转移  | 一次       | 消耗或转移所有权 |
