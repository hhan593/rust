fn main() {
    // let s = String::from("hello");
    // let s2: &String = &s;
    // let s3: &str = &s[..];

    let mut s = String::from("hello");
    for &item in s.as_bytes().iter() {
        if item == b'l' {
            s.push_str(" world");
        }
        println!("{s}");

        //         在这段代码中，存在两个相互冲突的“借用”：

        // 不可变借用（Immutable Borrow）： 当你调用 s.as_bytes().iter() 时，迭代器获取了对 s 内容的不可变引用。在整个 for 循环运行期间，这个借用都是有效的，以保证你在遍历时数据不会被篡改。

        // 可变操作（Mutable Mutation）： 在循环体内，你调用了 s.push_str(" world")。这个操作需要一个可变借用，因为它可能会改变 s 的长度，甚至导致 s 在堆内存中重新分配空间（扩容）。
    }
}
// 变量,类型,栈上存储的数据,64位系统下的总字节数
// s2,&String,指向 s 的地址,8 字节
// s3,&str,数据地址 + 数据长度,16 字节
