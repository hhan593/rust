// 【知识点总结】
// 1. 无限循环 (loop)：Rust 特有的关键字，用于无条件执行死循环，直到遇到 break。比 `while true` 更高效且在类型推导上有优势。
// 2. 条件循环 (while)：当给定条件为真时持续执行，常用于循环次数未知的情况。
// 3. 集合与区间遍历 (for...in)：用于遍历迭代器（如数组、Vec、区间等）。`a..b` 为左闭右开区间，`a..=b` 为全闭区间。
// 4. 控制流跳转 (break / continue)：`break` 立即终止整个循环；`continue` 跳过当前这一轮，直接进入下一次循环。
// 5. 循环标签 (Loop Labels)：使用单引号开头（如 `'outer`）标记循环，可以在多层嵌套循环中指定 `break` 或 `continue` 针对外层循环生效。
// 6. 迭代器 (Iterators) 与函数式编程：`.iter()` 产生一个借用集合元素的迭代器。结合 `.map()`（映射闭包）和 `.collect()`（收集回集合），可以极其优雅且高效地完成数据转换操作，避免手动管理状态和可变性。

//循环、break、continue、以及迭代（iterators）        // 原代码注释
fn main() {                                         // 程序的入口函数 main (知识点：程序入口)
    //loop循环                                       // 原代码注释
    // loop {                                       // Rust 的无条件死循环关键字 (知识点：无限循环 loop)
    //     println!("Enter your name: ");           // 打印提示信息 (知识点：标准输出)
    //     std::thread::sleep(std::time::Duration::from_secs(1)); // 让当前线程休眠 1 秒，防止死循环导致 CPU 占满 (知识点：标准库线程阻塞与持续时间控制)
    // }                                            // 循环结束，回到起始位置继续执行

    //while循环                                      // 原代码注释
    let mut i = 0;                                  // 声明可变变量 i，初始化为 0 (知识点：可变变量 mut)
    while i < 10 {                                  // 当 i 小于 10 时循环持续执行 (知识点：while 条件循环)
        println!("{} ", i);                         // 打印当前 i 的值 (知识点：格式化输出)
        i += 1;                                     // 将 i 的值自增 1 (知识点：复合赋值运算符)
    }                                               // while 循环块结束

    //for循环                                        // 原代码注释
    let arr1 = [1, 2, 3, 4, 5];                     // 声明一个长度固定为 5 的整型数组 arr1 (知识点：静态数组 Array 的声明)
    for element in arr1 {                           // 遍历 arr1 中的每一个元素，按值或按隐式迭代器传递 (知识点：for...in 遍历)
        println!("{}", element);                    // 打印当前取到的元素 (知识点：格式化输出)
    }                                               // for 循环结束

    //0-9,1..=10的范围是0-10                         // 原代码注释（注：1..=10 范围是 1 到 10，1..10 范围是 1 到 9）
    for i in 1..10 {                                // 遍历区间 1 到 9 (知识点：左闭右开区间操作符 ..)
        println!("{}", i);                          // 打印从 1 开始到 9 结束的数字 (知识点：范围生成器 range)
    }                                               // for 循环结束

    for ele in arr1 {                               // 再次遍历数组 arr1 (知识点：for 遍历)
        if ele == 3 {                               // 判断当前元素是否等于 3 (知识点：if 条件判断)
            break;                                  // 如果等于 3，则立刻完全跳出（终止）这个 for 循环 (知识点：break 终止循环)
        }                                           // if 代码块结束
        println!("{}", ele);                        // 因此这里只会打印 1 和 2 (知识点：流程阻断)
    }                                               // for 循环结束

    for ele in arr1 {                               // 再次遍历数组 arr1 (知识点：for 遍历)
        if ele == 3 {                               // 判断当前元素是否等于 3 (知识点：if 条件判断)
            continue;                               // 如果等于 3，跳过本轮循环剩下的代码，直接进入下一轮遍历取数字 4 (知识点：continue 跳过本轮)
        }                                           // if 代码块结束
        println!("{}", ele);                        // 因此这里会打印 1, 2, 4, 5（跳过了 3）(知识点：控制流跳转)
    }                                               // for 循环结束

    // 跳出两层循环                                    // 原代码注释
    'outer: loop {                                  // 给外层死循环打上一个名为 'outer 的生命周期标签 (知识点：循环标签 Loop Labels)
        println!("outer");                          // 打印外层循环信息
        loop {                                      // 开启一个内层死循环 (知识点：嵌套循环)
            println!("inner");                      // 打印内层循环信息
            break 'outer;                           // 绝招：不仅跳出当前内层循环，直接打破并跳出名为 'outer 的外层循环！(知识点：定向 break)
        }                                           // 内层 loop 结束
    }                                               // 外层 loop 结束，由于内部直接被 break，整个嵌套只各执行了一次打印

    //循环写法                                       // 原代码注释：接下来展示传统的命令式(Imperative)数据转换处理
    let numbers = [1, 2, 3, 4, 5];                  // 声明基础数组 numbers (知识点：数组声明)
    let mut for_numbers = Vec::new();               // 创建一个空的可变动态数组 (Vector)，用来存放结果 (知识点：动态数组 Vec 实例化)

    for &number in numbers.iter() {                 // numbers.iter() 产生对元素的引用(&i32)。for 里面用 &number 进行模式解构，直接提取出底层的 i32 值到 number 变量 (知识点：迭代器借用，模式解构取值)
        let item = number * number;                 // 计算数字的平方 (知识点：算术运算)
        for_numbers.push(item);                     // 将计算后的平方值追加存入 Vec 中 (知识点：Vec 可变方法 push)
    }                                               // for 循环结束
    println!("for_numbers: {:?}", for_numbers);     // 打印传统循环处理完后的结果集 (知识点：Debug 格式化打印)

    //迭代写法                                       // 接下来展示 Rust 更推荐的函数式、声明式(Declarative)数据处理方式
    let numbers1 = [1, 2, 3, 4, 5].to_vec();        // 声明数组并调用 to_vec() 转换为存放在堆上的动态数组 Vec (知识点：数组转 Vec)

    let iter_numbers: Vec<i32> = numbers1.iter()    // 调用 .iter() 创建不可变借用迭代器。注意：这里需显式指定变量类型 Vec<i32> 供 collect 识别目标集合 (知识点：显式类型声明，生成迭代器)
        .map(|&num| num * num)                      // .map 接收一个闭包(Closure)。|&num| 解构传入的引用拿到值，并返回平方。map 会将规则应用到流经的每个元素上并生成新的迭代器 (知识点：迭代器适配器 map，闭包/匿名函数 Closure 解构)
        .collect();                                 // .collect() 将经过处理的迭代器流水线“收集”组装成我们指定的目标集合（即 Vec<i32>） (知识点：迭代器消费方法 collect)

    println!("iter_numbers: {:?}", iter_numbers);   // 打印用迭代器函数式写法得到的结果，和上一块完全一样，但没有产生多余的状态可变性，更加安全优雅 (知识点：数据结果对比)
}                                                   // main 函数结束