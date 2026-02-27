fn main() {
    println!("Hello, world!");

    let mut s = String::new();

    let data = "initial contents"; //这个data为什么是&str类型的，因为它是一个字符串字面值，字符串字面值在Rust中是&str类型的。

    let s = data.to_string(); // s:String类型 to_string()方法是将&str类型转换为String类型的方法。

    let s = String::from("initial contents"); // 另一种创建String类型的方法，使用String::from()函数。

    //给String添加内容字符串可以使用push_str()和push方法。

    let mut strs = String::from("2020520200");

    //     push_str()：接收的是 字符串切片 (&str)。就像是往本子里贴一段话，长度不限。
    // push()：接收的是 单个字符 (char)。在 Rust 中，char 必须用 单引号 ' 包裹。就像是往本子里写一个字母。
    // 深入理解：&str 与 char 的区别
    // char (单引号 'w')：它是 Rust 的原生类型，固定占据 4 个字节。它代表一个 Unicode 标量值，意味着它可以是 'a'，也可以是 '心' 或者 '😊'。
    // &str (双引号 "w")：它是一个指向 UTF-8 编码数据的窗口。即使双引号里只有一个字母，它也是一个字符串序列，底层可能占据 1 到 4 个字节不等，取决于字符的 Unicode 编码。

    let mut strs = String::from("2020520200");

    let s1 = "hello";
    let s = String::from("world");
    strs.push_str(s1); // ✅ 正常，添加了一串字符

    let s2 = 'w'; // ✅ 注意：这里改成了单引号，类型是 char
    strs.push(s2); // ✅ 正常，添加了一个字符
    strs.push('c');
    strs.push('h');

    println!("{}", strs); // 输出: 2020520200hellowch

    let s3 = strs + &s;
    // 注意：这里 strs 被移动了，不能再使用 strs 了,为什么呢，这时候使用的strs，而第二个参数使用的是s的引用（&s）
    // 所以s没有被移动，可以继续使用s，而strs被移动了，所以不能再使用strs了。
    println!("{}", s3); // 输出: 2020520200hellowchworl
    let s4 = format!("{}-{}-{}", s3, s1, s2); // format!宏可以将多个字符串拼接成一个新的字符串，且不会移动任何参数。
    println!("{}", s4); // 输出: 2020520200hellowchworl-
}
