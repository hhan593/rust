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

/**
 * 报错原因分析：
 * 这个函数违反了 Rust 的“生命周期省略规则（Lifetime Elision Rules）”。
 * 编译器看到返回类型是 &str，它不知道这个引用的有效期是跟着 x 走，还是跟着 y 走。
 * 如果没有明确标注，编译器无法保证返回的引用在 main 函数中使用时依然有效。
 */
// 修正方式：加上生命周期标注 <'a>
// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     // fn longest(x: &str, y: &str) -> &str {
//     // Rust 建议：if 条件不需要加小括号 ()
//     if x.len() > y.len() {
//         x // 如果返回 x，那么返回值的生命周期必须涵盖 x 的作用域
//     } else {
//         y // 如果返回 y，那么返回值的生命周期必须涵盖 y 的作用域
//     }
// }
// fn shortest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() < y.len() { x } else { y }
// }
//结构体中的生命周期

//结构体持有引用是，必须添加生命周期注解
// 这里的 <'a> 就像是一个“契约标记”
// 它告诉编译器：ImportantExcerpt 这个结构体的寿命，绝对不能超过它内部 part 引用的那段内存
struct ImportantExcerpt<'a> {
    part: &'a str, // 声明：part 引用的数据必须活得比这个结构体实例“至少一样久”
}

fn main() {
    // 1. 这一行在内存里开辟了一块空间存字符串，novel 是它的“房东”
    let novel = String::from("Call me Ishmael. Some years ago...");

    // 2. 声明一个变量，准备用来存以后拿到的“纸条”（引用）
    let first_sentence;

    {
        // 3. 在这个小花括号（作用域）里，我们创建了一个结构体实例 i
        // 它从 novel 那里“借”了一段话：&novel[..19]
        let i = ImportantExcerpt { part: &novel[..19] };

        // 4. 将结构体里的引用赋值给外部变量
        // 注意：这里只是复制了那张“指向内存的纸条”，并没有复制字符串本身
        first_sentence = i.part;
    } // 5. 变量 i 在这里“寿终正寝”被销毁了。
    // 但是！由于 i.part 指向的是 novel，而 novel 在花括号外面还活着，
    // 所以这个引用（纸条）依然是有效的。

    // 6. 成功打印！
    // 编译器检查：first_sentence 指向的数据源 (novel) 依然健在。
    println!("{}", first_sentence);
} // 7. 房东 novel 在这里才被销毁

/********3.6 方法中的生命周期 ******** */
impl<'a> ImportantExcerpt<'a> {
    // 情况 A：返回值是普通基础类型（如 i32）
    // 这种就像是“进屋借书，但最后只给了个数字回话”，不牵扯引用的生死。
    // 规则 1：输入参数 &self 自动获得一个隐藏的生命周期。
    fn level(&self) -> i32 {
        3
    }

    // 情况 B：返回值是一个引用（&str）
    // 规则 3 特权：如果方法里有 &self 或 &mut self，
    // 编译器会默认：返回出来的那个“借条”（&str）和 self 活得一样久。
    // 哪怕参数里还有别的引用（announcement），编译器也会优先认定返回值是“从 self 身上薅下来的”。
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
