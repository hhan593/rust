fn main() {
    let mut s1 = String::from("hello helloworld");

    let first_word = first_word(&s1);
    println!("first_word: {}", first_word);
    // s1.clear();
    // let s2 = &s1[0..5];
    // println!("s1: {}", s1);
    // let s3 = &s2[0..3];
    // println!("s3: {}", s3);
    // println!("first_word: {}", first_word); //即使被清零，first_word 依然有效
    // println!("s2: {}", s2);
}

/// 匹配空格函数
/// 查找字符串中第一个空格的位置
///
/// # 参数
/// * `s`: &String - 需要查找的字符串引用
///
/// # 返回值
/// * usize - 返回第一个空格的索引位置，如果没有空格则返回字符串长度
fn first_word(s: &str) -> &str {
    // 将字符串转换为字节数组以便遍历
    let bytes = s.as_bytes();

    // 遍历字节数组，查找空格字符
    for (i, &item) in bytes.iter().enumerate() {
        //iter 方法返回集合中的每一个元素，而 enumerate 包装了 iter 的结果，将这些元素作为元组的一部分来返回。
        if item == b' ' {
            // return i;
            return &s[..i];
        }
    }
    // 没有找到空格，返回整个字符串的切片  rust要求返回的一定和参数类型一致，所以没有找到和找到都要返回一致的东西，这里返回字符串的切片 &s[..]，表示从字符串的开始到结尾的切片。
    &s[..]
}

//slice 是一种引用，所以它不拥有所有权。
