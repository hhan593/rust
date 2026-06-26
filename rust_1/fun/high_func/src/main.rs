// === 知识点总结 ===
// 1. 程序入口与函数定义 (Functions)：使用 `fn` 关键字定义函数，Rust 程序从 `main` 函数开始执行。函数体最后一行无分号表示隐式返回 (Implicit Return)。
// 2. 宏 (Macros)：Rust 中以 `!` 结尾的调用是宏，如用于控制台打印的 `println!` 和用于初始化动态数组的 `vec!`。
// 3. 高阶函数与函数指针 (Higher-Order Functions & Function Pointers)：可以将函数作为参数传递给另一个函数，使用 `fn(T) -> U` 表示函数指针类型。
// 4. 变量绑定与遮蔽 (Variable Binding & Shadowing)：使用 `let` 关键字声明变量。在同一作用域内可以多次使用 `let` 声明同名变量，新变量会“遮蔽”（Shadow）旧变量。
// 5. 迭代器 (Iterators)：提供一种处理序列模式的方法。`iter()` 创建不可变引用的迭代器，`into_iter()` 创建获取所有权的迭代器。
// 6. 闭包 (Closures)：类似于匿名函数，使用 `|参数| 表达式` 的语法定义，常作为参数传递给迭代器方法。
// 7. 迭代器适配器 (Iterator Adaptors)：如 `map`（映射元素）和 `filter`（过滤元素），它们是惰性的，会返回一个新的迭代器。
// 8. 迭代器消费器 (Iterator Consumers)：如 `collect`（收集为集合）和 `fold`（折叠/归约），它们会触发迭代器的执行。
// 9. 类型推导与 Turbofish 语法 (Type Inference & Turbofish)：使用 `Vec<_>` 让编译器推导内部类型；或者在方法调用后使用 `::<T>`（Turbofish语法）显式指定类型。
// 10. 格式化输出 (Formatting)：`{}` 用于实现 Display trait 的标准输出，`{:?}` 用于实现 Debug trait 的调试输出（如数组和集合）。

fn main() { // 知识点：程序入口。解释：定义程序的入口函数 main，无参数，无返回值。
    println!("Hello, world!"); // 知识点：标准输出宏。解释：向控制台打印字符串并换行。
    let result = func_twice(mul, 5); // 知识点：高阶函数调用、变量绑定。解释：将乘法函数 mul 作为函数指针传入 func_twice，参数为 5，结果绑定到变量 result。
    println!("The result is {}", result); // 知识点：格式化输出。解释：使用 `{}` 占位符打印变量 result 的值（最终计算：5*5*5*5 = 625）。
    let res = func_twice(add, 10); // 知识点：高阶函数调用。解释：将加法函数 add 作为函数指针传入 func_twice，参数为 10，结果绑定到变量 res。
    println!("The result is {}", res); // 知识点：格式化输出。解释：打印变量 res 的值（最终计算：10+10+10 = 30）。


    //数学计算 // 知识点：代码注释。解释：单行注释，忽略不执行。

    let numbrs = vec![1, 2, 3]; // 知识点：宏、动态数组初始化。解释：使用 vec! 宏创建一个包含 1, 2, 3 的可变长度数组 (Vector)，并绑定到 numbrs。
    let total: Vec<_> = numbrs.iter().map(|x| x + x).collect(); // 知识点：迭代器、闭包、map、collect、类型推导。解释：创建不可变引用迭代器，对每个元素执行 x+x，收集结果推导为 Vec 类型。
    println!("The total is {:?}", total); // 知识点：调试格式化输出。解释：因为 Vec 默认未实现 Display，需使用 `{:?}` 打印其结构内容 [2, 4, 6]。

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9]; // 知识点：变量绑定、宏。解释：重新声明一个新 Vector 并绑定到局部变量 numbers。
    let evens = numbers // 知识点：变量绑定、链式调用。解释：开始链式调用迭代器方法，并将最终结果绑定给 evens。
        .into_iter() // 知识点：获取所有权的迭代器。解释：消费 numbers 数组，将其所有权转移进迭代器（此后上面的 numbers 不能再使用）。
        .filter(|x| x % 2 == 0) // 知识点：迭代器适配器、闭包。解释：使用 filter 筛选出偶数（对 2 取余为 0 的元素）。
        .collect::<Vec<_>>(); // 知识点：迭代器消费器、Turbofish语法。解释：消费迭代器并收集元素，显式使用 `::<Vec<_>>` 告诉编译器收集为动态数组。
    println!("Even is {:?}", evens); // 知识点：调试格式化输出。解释：打印过滤后的偶数数组 [2, 4, 6, 8]。

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9]; // 知识点：变量遮蔽 (Shadowing)。解释：重新声明一个同名变量 numbers，遮蔽了之前已被 into_iter() 消耗掉的旧 numbers。

    let sum = numbers.iter().fold(0, |acc, x| acc + x); // 知识点：迭代器、fold 归约操作、闭包。解释：从初始值 0 开始，闭包逐个处理元素 `x` 累加到累加器 `acc`，计算所有元素之和。
    println!("The sum is {}", sum); // 知识点：标准输出宏。解释：打印最终的累加结果 (45)。
} // 知识点：作用域结束。解释：main 函数结束，局部变量被丢弃，内存释放。


fn func_twice(f: fn(i32) -> i32, x: i32) -> i32 { // 知识点：高阶函数定义。解释：接受一个类型为 fn(i32)->i32 的函数指针 f，以及一个 i32 参数 x，返回 i32 结果。
    f(f(x)) // 知识点：隐式返回、函数指针调用。解释：先计算 f(x)，将结果再次传给 f 调用，最后的值作为函数返回值（无分号）。
} // 知识点：作用域结束。解释：func_twice 结束。

fn add(x: i32) -> i32 { // 知识点：函数定义。解释：定义 add 函数，接受 i32 类型参数，返回 i32 类型。
    x + 10 // 知识点：隐式返回、算术运算。解释：将参数 x 加上 10，作为返回值。
} // 知识点：作用域结束。解释：add 函数结束。

fn mul(x: i32) -> i32 { // 知识点：函数定义。解释：定义 mul 函数，接受 i32 类型参数，返回 i32 类型。
    x * x // 知识点：隐式返回、算术运算。解释：将参数 x 乘以自身，作为返回值。
} // 知识点：作用域结束。解释：mul 函数结束。