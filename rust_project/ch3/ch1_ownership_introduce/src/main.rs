fn main() {
    //所有权 ownership
    // Rust 中的所有权是一种独特的内存管理系统，它通过一套规则来确保内存安全和资源管理。所有权系统的核心概念包括：

    // Copy 特质

    let c1 = 1;
    let c2 = c1;
    println!("c1: {}, c2: {}", c1, c2); // c1 和 c2 都可以使用，因为 i32 实现了 Copy 特质

    // 但是String 不实现 Copy 特质，所以当 String 被赋值给变量 s2 时，s2 会成为 String 的所有者，而 s1 就会变成无效的。
    let s1 = String::from("hello world");
    let first = first_world(&s1);
    println!("first: {}", first);
    let s2 = s1; // s1 的所有权被转移给 s2
                 // println!("s1: {}, s2: {}", s1, s2); // 这段代码会导致编译错误，因为 s1 已经失去了所有权，无法再使用。
                 // 但是可以使用clone 方法来创建一个 String 的副本，
    let s3 = s2.clone();
    println!("s2: {}, s3: {}", s2, s3); // 现在 s2 和 s3 都可以使用，因为它们是独立的 String 实例。
    get_length(s3);
    let s4 = danger();
    println!("{}", s4);
    // println!("s2: {}, s3: {}", s2, s3);
}

fn get_length(str: String) -> usize {
    println!("{}", str.len());
    str.len()
}

fn danger() -> String {
    let s = String::from("hello");
    "113".to_owned(); // 返回 s 的所有权
    return s;
}

fn first_world(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
