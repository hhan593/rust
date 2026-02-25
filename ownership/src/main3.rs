// fn get_first(vr: &Vec<i32>) -> i32 {
// <--- 注意这里的 &
//     vr[0]
// }

fn main() {
    let mut v = vec![0, 1, 2]; // v 拥有这个 Vec
    // let n = get_first(&v); // 传递的是 &v (借用)，不是 v (移动)
    // println!("{} {}", n, v[1]); // v 依然有效，可以继续使用

    let num: &i32 = &v[2]; // num 是一个引用，指向 v[2] 的值 2   此时编译器标记：v 被借用了，只要 num 还活着，v 就**不能变**！
    // 编译器大喊：“停！num 还指着 v 里面的数据呢，你要是改了 v（比如扩容移动了内存），num 就会变成‘悬垂指针’（指向无效内存）！”
    // 这就是 Rust 的借用规则：当你有一个引用指向某个数据时，你不能修改那个数据（除非你有一个可变引用），以保证内存安全。

    // 所以必须释放 num 的借用，才能修改 v。
    println!("num: {}, v: {:?}", num, v); // num 依然有效，v 也依然有效，
    v.push(3);
    let changed_num = &v[2]; // 这里 num 依然有效，因为它只是一个引用，指向 v[2] 的值 2，num 的值是 2，而不是 v[2] 的地址，所以 num + 1 不会改变 num 的值，也不会影响 v[2] 的值
    println!("changed_num: {}, v: {:?}", changed_num, v);
    // println!("num: {}, v: {:?}", num, v); // num 依然有效，v 也依然有效，但是 num 的值已经改变

    let mut s = String::from("hello");
    returm_a_string(&mut s);
    println!("s: {}", s);
}

// 关键点：函数签名 fn get_first(vr: &Vec<i32>)。
// 符号 &：表示参数 vr 是一个引用（Reference）。
// 含义：
// 当你调用 get_first(&v) 时，你并没有把 v 的所有权交给函数。
// 你只是借给了函数一个“查看权限”（不可变借用）。
// 因为所有权还在 main 函数的 v 手里，所以函数执行完后，v 不会被释放（drop），可以在后面继续使用

// 在 Rust 中，只有当所有权发生转移（Move）时，原变量才会失效并被释放。
// 传值 func(v) -> 所有权转移 -> v 失效。
// 传引用 func(&v) -> 所有权保留 -> v 依然有效
fn returm_a_string(output: &mut String) {
    output.replace_range(.., "hello_world");
}
