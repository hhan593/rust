// Rust 中的循环主要有三种：loop（无限循环）、while（条件循环）和 for（迭代循环）
//
// 跳出或控制循环的关键字：
// break：用于立即终止循环，彻底跳出循环体
// continue：用于跳过当前迭代（循环体中剩下的代码不执行了），直接进入下一次迭代

fn main() {
    // --- loop 循环 ---
    // loop 是 Rust 独有的无限循环语法。它会一直执行，直到遇到 break 或程序被强制终止（如 Ctrl+C）
    // (这里被注释掉了，否则会一直打印)
    // loop {
    //     println!("loop");
    //     std::thread::sleep(std::time::Duration::from_secs(1)); // 让程序暂停1秒，避免刷屏太快
    // }

    // --- while 循环 ---
    let mut count = 0; // 声明一个可变的变量 count，初始值为 0

    // 只要条件 count < 5 成立（为 true），就会一直执行大括号里的代码
    while count < 5 {
        println!("count: {}", count); // 打印当前 count 的值
        count += 1; // 每次循环结束前，将 count 的值加 1。如果不加这行，count 永远为 0，会变成死循环
    }
    // 知识点：while 适合用于“循环次数不确定，但结束条件很明确”的场景。

    // --- for 循环 (遍历范围 Range) ---
    // 0..5 在 Rust 中叫“范围（Range）”，它的规则是“包头不包尾”（包含0，不包含5）
    for i in 0..5 {
        println!("i: {}", i); // 依次打印 0, 1, 2, 3, 4
    }

    // --- for 循环 (遍历数组) ---
    let arr = ; // 定义一个包含5个整数的数组
    // 使用 for 循环直接遍历数组中的每一个元素，Rust 会自动处理索引，非常安全（不会越界）
    for i in arr {
        println!(" for 循环 i: {}", i); // 依次打印数组里的 1, 2, 3, 4, 5
    }<websource>source_group_web_1</websource>

    // 再次演示范围遍历，0..10 会打印 0 到 9
    for i in 0..10 {
        println!(" for 循环 i: {}", i);
    }

    // --- break 的实战演示 ---
    // 定义一个包含 1 到 10 的数组
    let arr = ;
    
    for element in arr {
        if element == 5 {
            break; // 当元素等于 5 时，立即终止整个 for 循环！后面的 6,7,8...都不会再遍历了
        }
        // 因为遇到 5 就 break 了，所以这行打印代码只会执行到 4
        println!("break for 循环 element: {}", element); // 输出：1, 2, 3, 4
    }<websource>source_group_web_2</websource>

    // --- continue 的实战演示 ---
    for element in arr {
        if element == 5 {
            continue; // 当元素等于 5 时，跳过本次循环剩下的代码（即跳过下面的 println），直接进入下一轮（去处理 6）
        }
        // 所以这里会打印除了 5 以外的所有数字 (1,2,3,4,6,7,8,9,10)
        println!("continue for 循环 element: {}", element);
    }

    // --- 循环标签 (Loop Labels) ---
    // 在嵌套循环（循环里套循环）中，默认的 break 只能跳出最内层的循环。
    // 如果想直接跳出外层循环，可以给外层循环打上标签（以单引号 ' 开头）
    'outer: loop { // 给外层 loop 打上标签，名字叫 'outer
        println!("outer loop");
        loop { // 这是一个内层 loop
            println!("inner loop");
            // 这里指定跳出 'outer 标签对应的循环，也就是说，直接结束整个外层的大循环！
            break 'outer; 
        }
    }
    // 知识点：如果没有 'outer，break 只会跳出内层 loop，外层 loop 会继续执行，变成死循环。

    // --- 循环的常规写法（命令式编程） ---
    let numbers = ; // 定义原始数组
    let mut for_numbers = Vec::new(); // 创建一个空的、可变的向量（动态数组），用来存放结果
    
    // 遍历 numbers 数组<websource>source_group_web_3</websource>。&number 表示我们借用数组里的值（因为数组是 Copy 的，这里也可以直接写 number）
    for &number in numbers.iter() {
        let item = number * number; // 计算当前数字的平方
        for_numbers.push(item);     // 将计算好的平方值推入（添加）到 for_numbers 向量中
    }
    println!("for : {:?}", for_numbers); // 打印结果向量:<websource>source_group_web_4</websource>

    // --- 迭代的写法（函数式编程 / 迭代器） ---
    // 这是 Rust 中更地道、更高效的“零成本抽象”写法
    let numbers = .to_vec(); // 将数组转为向量 (Vec)
    
    // 1. .iter() 获取这个向量的迭代器
    // 2. .map() 是一个高阶函数，它对迭代器里的每个元素执行闭包操作（|&number| number * number 就是计算平方）
    let iter_numbers = numbers.iter().map(|&number| number * number);
    
    // 3. .collect::<Vec<_>>() 将迭代器处理完的结果，收集并组装成一个新的向量
    println!("iter : {:?}", iter_numbers.collect::<Vec<_>>()); // 打印结果: 
}
```<websource>source_group_web_5</websource>

//  重点知识总结

// 1. **`for` 循环是 Rust 中最常用的循环**：
//    它基于“迭代器（Iterator）”工作。相比于其他语言用 `for (i=0; i<arr.length; i++)` 这种写法，Rust 的 `for element in arr` 不仅代码更简洁，而且彻底杜绝了“差一错误（Off-by-one error）”和数组越界的风险<websource>source_group_web_6</websource>。

// 2. **`loop` 的独特之处**：
//    除了做无限循环，`loop` 还可以**返回值**！你可以把 `loop` 当作一个表达式，在 `break` 后面带上一个值，这个值就会作为整个 `loop` 的结果返回给变量<websource>source_group_web_7</websource>。例如：
//    ```rust
//    let result = loop {
//        counter += 1;
//        if counter == 10 {
//            break counter * 2; // 循环结束时，返回 20
//        }
//    };