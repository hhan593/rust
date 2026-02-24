fn main() {
    // 1. 创建 m1 (Box<String>)
    let m1 = Box::new(String::from("hello"));
    println!("m1: {}, m1的地址：{:p}", m1, &m1);

    // 2. 创建 m2 (Box<i32>) - 注意：这和 m1 无关，是新创建的
    let m2: Box<i32> = Box::new(-42);

    // 3. 隐式解引用调用方法
    // Box 实现了 Deref<Target=i32>，所以 m2.abs() 会自动解引用成 (*m2).abs()
    let m2_abs = m2.abs();

    // 4. 显式解引用调用关联函数
    // *m2 得到 i32 值，然后调用 i32::abs
    let m2_abs1 = i32::abs(*m2);

    // 5. 打印 m1 和 m2
    // 因为 i32 实现了 Copy trait，*m2 只是复制了值，m2 依然有效
    // String 没有实现 Copy，但这里我们没动 m1，所以 m1 也有效
    println!("m1: {}, m2: {}", m1, m2);

    println!("m2的绝对值 (隐式): {}", m2_abs);
    println!("m2的绝对值 (显式): {}", m2_abs1);

    // 6. 正确比较并打印结果
    // 使用 == 运算符比较，返回 bool
    let are_equal = m2_abs == m2_abs1;
    println!("m2的绝对值相等吗？ {}", are_equal);

    // 7. 正确使用 assert_eq! (单独一行，用于测试)
    // 如果不相等，程序会在这里崩溃并报错；如果相等，程序继续运行
    assert_eq!(m2_abs, m2_abs1); // 使用 assert_eq! 测试 m2_abs 和 m2_abs1 是否相等
    println!("✅ assert_eq! 检查通过！");
}
