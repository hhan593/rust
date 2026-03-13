// fn main() {
//     let b = Box::new(5);
//     println!("b = {}", b)
// }
//这样有个问题就是不知道这个List的空间多大
// enum List {
//     coin(i32, Box<List>),
//     Nil,
// }

//1. Box<T>：堆上分配
// Box 用于将值从栈上移到堆上。当你不知道在编译时类型的大小，或者想转移大量数据的所有权时非常有用。
fn main() {
    // 创建一个 Box，将整数 5 存储在堆上
    // 栈上保存的是指向堆内存的指针
    let b = Box::new(5);

    // Box 实现了 Deref trait，所以我们可以直接解引用获取值
    println!("b = {}", *b); // 输出: b = 5

    // --- 递归类型示例 ---
    // Rust 需要知道每个类型占用多少内存。对于递归类型（如列表），
    // 因为长度未知，编译器无法确定大小，所以必须用 Box 包装。

    enum List {
        Cons(i32, Box<List>), // 用 Box 包装，大小就固定了（指针大小）
        Nil,
    }

    use List::{Cons, Nil};

    // 构建链表: 1 -> 2 -> Nil
    let list = Cons(1, Box::new(Cons(2, Box::new(Nil))));
}
