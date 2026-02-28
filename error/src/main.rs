// ============================================================================
// Rust 错误处理知识手册 —— 完整示例代码
// ============================================================================
//
// 本文件通过可运行的代码示例，展示 Rust 错误处理的核心知识点。
// 对应文档：docs/Rust错误处理知识手册.md
//
// 运行方式：cargo run
// 查看 backtrace：RUST_BACKTRACE=1 cargo run
// ============================================================================

use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{self, ErrorKind, Read};
use std::num::ParseIntError;

fn main() {
    println!("========== Rust 错误处理知识手册 示例代码 ==========\n");

    // 按知识点依次演示
    demo_panic();
    demo_catch_unwind();
    demo_result_match();
    demo_result_nested_match();
    demo_result_unwrap_or_else();
    demo_result_common_methods();
    demo_option();
    demo_unwrap_and_expect();
    demo_question_mark_operator();
    demo_question_mark_with_option();
    demo_custom_error_type();
    demo_from_trait_conversion();
    demo_map_err();
    demo_box_dyn_error();
    demo_error_chain();

    println!("\n========== 所有示例执行完毕 ==========");
}

// ============================================================================
// 1. 不可恢复错误：panic!
// ============================================================================
// panic! 用于程序遇到不可恢复的错误时终止执行。
// 触发方式：(1) 显式调用 panic! 宏 (2) 运行时错误（如数组越界）
// 默认行为：栈展开（unwinding），沿调用栈逐层清理数据。
// 可在 Cargo.toml 中设置 panic = 'abort' 改为直接终止。

fn demo_panic() {
    println!("--- 1. panic! 不可恢复错误 ---");

    // 显式调用 panic!（此处注释掉，否则程序会终止）
    // panic!("程序崩溃了！");

    // 运行时 panic 示例：数组越界（注释掉，否则程序会终止）
    // let v = vec![1, 2, 3];
    // v[99]; // 触发 panic: index out of bounds

    println!("  panic! 用于不可恢复的错误（如违反契约、数据不一致）");
    println!("  设置 RUST_BACKTRACE=1 环境变量可查看完整调用栈\n");
}

// ============================================================================
// 2. catch_unwind：捕获 panic
// ============================================================================
// std::panic::catch_unwind 可以捕获 panic，将其转为 Result。
// 注意：不应作为通用 try/catch 机制！
// 适用场景：服务器隔离请求崩溃、FFI 边界防止 panic 传播。
// 限制：仅捕获 unwinding panic，不能捕获 abort 模式。

fn demo_catch_unwind() {
    println!("--- 2. catch_unwind 捕获 panic ---");

    // 正常执行的闭包 → Ok
    let result = std::panic::catch_unwind(|| 42);
    println!("  正常闭包结果: {:?}", result); // Ok(42)

    // panic 的闭包 → Err
    let result = std::panic::catch_unwind(|| {
        panic!("测试 panic");
    });
    println!("  panic 闭包结果: is_err = {}", result.is_err()); // true

    // 实际用途：服务器中隔离单个请求的崩溃
    let result = std::panic::catch_unwind(|| {
        let data: Vec<i32> = vec![1, 2, 3];
        data[10] // 越界 panic
    });
    match result {
        Ok(val) => println!("  请求成功: {}", val),
        Err(_) => println!("  请求处理失败，但服务器继续运行"),
    }

    println!();
}

// ============================================================================
// 3. Result 与 match：可恢复错误的基本处理
// ============================================================================
// Result<T, E> 是 Rust 处理可恢复错误的核心类型。
// enum Result<T, E> { Ok(T), Err(E) }
// 使用 match 表达式可以精确处理 Ok 和 Err 两种情况。

fn demo_result_match() {
    println!("--- 3. Result 与 match ---");

    // 打开文件返回 Result<File, io::Error>
    let file_result = File::open("hello.txt");

    // 使用 match 处理 Result
    match file_result {
        Ok(_file) => println!("  文件打开成功"),
        Err(error) => println!("  文件打开失败: {:?}", error),
    }

    println!();
}

// ============================================================================
// 4. 嵌套 match：根据错误类型做不同处理
// ============================================================================
// 通过 error.kind() 区分不同的 IO 错误类型（如 NotFound、PermissionDenied）。
// 可以针对不同错误类型采取不同的恢复策略。

fn demo_result_nested_match() {
    println!("--- 4. 嵌套 match 处理不同错误类型 ---");

    let file = File::open("hello.txt");

    match file {
        Ok(_f) => println!("  文件已存在，打开成功"),
        Err(error) => match error.kind() {
            // 文件不存在 → 尝试创建
            ErrorKind::NotFound => {
                println!("  文件不存在，尝试创建...");
                match File::create("hello.txt") {
                    Ok(_fc) => println!("  文件创建成功"),
                    Err(e) => println!("  文件创建失败: {:?}", e),
                }
            }
            // 权限不足
            ErrorKind::PermissionDenied => {
                println!("  权限不足，无法打开文件");
            }
            // 其他错误
            other => println!("  其他错误: {:?}", other),
        },
    }

    // 清理创建的文件
    let _ = std::fs::remove_file("hello.txt");
    println!();
}

// ============================================================================
// 5. unwrap_or_else：用闭包简化错误处理
// ============================================================================
// unwrap_or_else 在 Err 时执行闭包，避免深层 match 嵌套。
// 比 unwrap 更灵活，比 match 更简洁。

fn demo_result_unwrap_or_else() {
    println!("--- 5. unwrap_or_else 闭包处理 ---");

    // 用 unwrap_or_else 替代嵌套 match
    let _file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("创建文件失败: {:?}", error);
            })
        } else {
            panic!("打开文件失败: {:?}", error);
        }
    });

    println!("  使用 unwrap_or_else 成功处理文件操作");

    // 清理
    let _ = std::fs::remove_file("hello.txt");
    println!();
}

// ============================================================================
// 6. Result 的常用方法
// ============================================================================
// map: 转换 Ok 中的值，Err 不变
// and_then: 链式操作（类似 flatmap），可返回新的 Result
// unwrap_or: Err 时返回默认值
// unwrap_or_else: Err 时通过闭包计算默认值
// unwrap_or_default: Err 时返回类型的 Default 值
// is_ok / is_err: 判断结果类型

fn demo_result_common_methods() {
    println!("--- 6. Result 常用方法 ---");

    let ok_val: Result<i32, String> = Ok(42);
    let err_val: Result<i32, String> = Err("出错了".to_string());

    // map: 对 Ok 值进行变换
    let doubled = ok_val.as_ref().map(|v| v * 2);
    println!("  map(Ok(42) * 2) = {:?}", doubled); // Ok(84)

    let doubled_err = err_val.as_ref().map(|v| v * 2);
    println!("  map(Err) = {:?}", doubled_err); // Err("出错了")

    // and_then: 链式操作，可以返回新的 Result
    let result = Ok::<i32, String>(42).and_then(|v| {
        if v > 0 {
            Ok(v * 10)
        } else {
            Err("必须为正数".to_string())
        }
    });
    println!("  and_then(Ok(42)) = {:?}", result); // Ok(420)

    // unwrap_or: 提供默认值
    let val = Err::<i32, &str>("error").unwrap_or(0);
    println!("  Err.unwrap_or(0) = {}", val); // 0

    // unwrap_or_else: 通过闭包计算默认值
    let val: i32 = Err::<i32, &str>("error").unwrap_or_else(|e| {
        println!("    unwrap_or_else 收到错误: {}", e);
        -1
    });
    println!("  unwrap_or_else 结果 = {}", val); // -1

    // unwrap_or_default: 使用类型的 Default 值
    let val: i32 = Err::<i32, &str>("error").unwrap_or_default();
    println!("  unwrap_or_default(i32) = {}", val); // 0

    let val: String = Err::<String, &str>("error").unwrap_or_default();
    println!("  unwrap_or_default(String) = {:?}", val); // ""

    // is_ok / is_err: 判断
    println!("  Ok(42).is_ok() = {}", Ok::<i32, &str>(42).is_ok()); // true
    println!("  Err.is_err() = {}", Err::<i32, &str>("e").is_err()); // true

    println!();
}

// ============================================================================
// 7. Option 类型
// ============================================================================
// Option<T> 用于表示值可能存在也可能不存在。
// enum Option<T> { Some(T), None }
// 类似其他语言的 null/nil，但通过类型系统强制检查，杜绝空指针异常。
// 支持 match、if let、map、and_then、unwrap_or 等操作。
// 可通过 ok_or / ok_or_else 转换为 Result。

fn demo_option() {
    println!("--- 7. Option 类型 ---");

    let some_value: Option<i32> = Some(42);
    let no_value: Option<i32> = None;

    // match 处理
    match some_value {
        Some(v) => println!("  match Some: 值是 {}", v),
        None => println!("  match None: 没有值"),
    }

    // if let 简化（只关心 Some 的情况）
    if let Some(v) = some_value {
        println!("  if let Some: 值是 {}", v);
    }

    // map: 转换内部值
    let doubled = some_value.map(|v| v * 2);
    println!("  map(Some(42) * 2) = {:?}", doubled); // Some(84)
    println!("  map(None * 2) = {:?}", no_value.map(|v| v * 2)); // None

    // and_then (flatmap): 链式操作
    let result = some_value.and_then(|v| {
        if v > 0 { Some(v * 10) } else { None }
    });
    println!("  and_then(Some(42)) = {:?}", result); // Some(420)

    // unwrap_or: 提供默认值
    println!("  None.unwrap_or(0) = {}", no_value.unwrap_or(0)); // 0

    // filter: 条件过滤
    let filtered = some_value.filter(|v| *v > 100);
    println!("  Some(42).filter(> 100) = {:?}", filtered); // None

    // Option 转 Result（ok_or / ok_or_else）
    let result: Result<i32, &str> = some_value.ok_or("值不存在");
    println!("  Some(42).ok_or() = {:?}", result); // Ok(42)

    let result: Result<i32, &str> = no_value.ok_or("值不存在");
    println!("  None.ok_or() = {:?}", result); // Err("值不存在")

    // zip: 合并两个 Option
    let a = Some(1);
    let b = Some("hello");
    let c: Option<i32> = None;
    println!("  Some(1).zip(Some(\"hello\")) = {:?}", a.zip(b)); // Some((1, "hello"))
    println!("  None.zip(Some(\"hello\")) = {:?}", c.zip(b)); // None

    println!();
}

// ============================================================================
// 8. unwrap 和 expect
// ============================================================================
// unwrap(): Ok/Some 时返回值，Err/None 时 panic（默认错误信息）
// expect("msg"): 同 unwrap，但 panic 时使用自定义信息
// 生产代码中优先用 expect，提供有意义的上下文信息。
// 更安全的替代：unwrap_or、unwrap_or_else、unwrap_or_default。

fn demo_unwrap_and_expect() {
    println!("--- 8. unwrap 和 expect ---");

    // unwrap: 成功时返回值
    let value: Result<i32, &str> = Ok(42);
    println!("  Ok(42).unwrap() = {}", value.unwrap()); // 42

    // unwrap: 失败时 panic（此处注释掉）
    // let err: Result<i32, &str> = Err("boom");
    // err.unwrap(); // panic: called `Result::unwrap()` on an `Err` value: "boom"

    // expect: 提供自定义 panic 信息（注释掉）
    // let err: Result<i32, &str> = Err("boom");
    // err.expect("期望获取一个数值"); // panic: 期望获取一个数值: "boom"

    // Option 上的 unwrap/expect
    let name: Option<&str> = Some("Alice");
    println!("  Some(\"Alice\").unwrap() = {}", name.unwrap());

    // 推荐：当你确信不会失败时，用 expect 说明原因
    let port: u16 = "8080".parse().expect("硬编码的端口号应该总是能解析");
    println!("  expect 解析端口: {}", port);

    println!("  提示：生产代码中优先使用 expect 而非 unwrap\n");
}

// ============================================================================
// 9. ? 操作符与错误传播
// ============================================================================
// ? 操作符是 Rust 传播错误的简洁语法糖。
// 对 Result：Ok 时提取值继续执行，Err 时立即 return Err(e)
// 对 Option：Some 时提取值，None 时立即 return None
// ? 会自动调用 From::from() 进行错误类型转换。
// 等价于 match expr { Ok(v) => v, Err(e) => return Err(From::from(e)) }

fn demo_question_mark_operator() {
    println!("--- 9. ? 操作符与错误传播 ---");

    // 调用使用 ? 的函数
    match read_username_from_file() {
        Ok(username) => println!("  读取到用户名: {}", username),
        Err(e) => println!("  读取失败（预期）: {}", e),
    }

    // 链式调用版本
    match read_username_chained() {
        Ok(username) => println!("  链式读取用户名: {}", username),
        Err(e) => println!("  链式读取失败（预期）: {}", e),
    }

    // 使用 fs::read_to_string 的极简版本
    match std::fs::read_to_string("username.txt") {
        Ok(s) => println!("  极简读取: {}", s),
        Err(e) => println!("  极简读取失败（预期）: {}", e),
    }

    println!();
}

// ? 操作符基本用法：逐步传播错误
fn read_username_from_file() -> Result<String, io::Error> {
    let mut file = File::open("username.txt")?; // 失败则提前返回 Err
    let mut username = String::new();
    file.read_to_string(&mut username)?; // 失败则提前返回 Err
    Ok(username)
}

// ? 操作符链式调用：更简洁
fn read_username_chained() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("username.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

// ============================================================================
// 10. ? 操作符用于 Option
// ============================================================================
// ? 也可用于 Option 类型：Some 时提取值，None 时提前返回 None。
// 注意：? 用于 Option 的函数返回类型必须是 Option。

fn demo_question_mark_with_option() {
    println!("--- 10. ? 用于 Option ---");

    println!(
        "  first_char(\"hello\\nworld\") = {:?}",
        first_char("hello\nworld")
    ); // Some('h')
    println!("  first_char(\"\") = {:?}", first_char("")); // None

    println!(
        "  last_char_of_first_line(\"abc\\ndef\") = {:?}",
        last_char_of_first_line("abc\ndef")
    ); // Some('c')
    println!(
        "  last_char_of_first_line(\"\") = {:?}",
        last_char_of_first_line("")
    ); // None

    println!();
}

// ? 用于 Option：None 时提前返回 None
fn first_char(text: &str) -> Option<char> {
    text.lines().next()?.chars().next()
    //                 ^ lines().next() 为 None 时直接返回 None
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

// ============================================================================
// 11. 自定义错误类型
// ============================================================================
// 自定义错误需要实现三个 trait：
//   (1) std::fmt::Debug — 通常用 #[derive(Debug)]
//   (2) std::fmt::Display — 面向用户的错误描述
//   (3) std::error::Error — 标记 trait，可选实现 source() 获取底层错误
// 通常使用枚举定义，每个变体代表一种错误类型。

#[derive(Debug)]
enum AppError {
    Io(io::Error),        // 包装 IO 错误
    Parse(ParseIntError), // 包装解析错误
    NotFound(String),     // 自定义：资源未找到
    Validation(String),   // 自定义：数据验证失败
}

// 实现 Display：面向用户的错误描述
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Io(e) => write!(f, "IO 错误: {}", e),
            AppError::Parse(e) => write!(f, "解析错误: {}", e),
            AppError::NotFound(name) => write!(f, "未找到: {}", name),
            AppError::Validation(msg) => write!(f, "验证失败: {}", msg),
        }
    }
}

// 实现 Error trait：可选提供 source() 获取底层错误链
impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AppError::Io(e) => Some(e),    // 有底层错误源
            AppError::Parse(e) => Some(e), // 有底层错误源
            AppError::NotFound(_) => None, // 无底层错误
            AppError::Validation(_) => None,
        }
    }
}

fn demo_custom_error_type() {
    println!("--- 11. 自定义错误类型 ---");

    let errors: Vec<AppError> = vec![
        AppError::NotFound("config.yaml".to_string()),
        AppError::Validation("年龄不能为负数".to_string()),
    ];

    for err in &errors {
        // Display trait: 用户友好的输出
        println!("  Display: {}", err);
        // Debug trait: 开发者友好的输出
        println!("  Debug:   {:?}", err);
        // Error::source(): 获取底层错误（这两个变体没有底层错误）
        println!("  Source:  {:?}", err.source());
    }

    println!();
}

// ============================================================================
// 12. From trait 与错误自动转换
// ============================================================================
// 实现 From<SourceError> for TargetError 后，
// ? 操作符会自动调用 From::from() 进行错误类型转换。
// 这是 Rust 错误传播的核心机制：
//   match expr { Ok(v) => v, Err(e) => return Err(From::from(e)) }

// 实现 From<io::Error>：允许 ? 自动将 io::Error 转换为 AppError
impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError::Io(error)
    }
}

// 实现 From<ParseIntError>：允许 ? 自动将 ParseIntError 转换为 AppError
impl From<ParseIntError> for AppError {
    fn from(error: ParseIntError) -> Self {
        AppError::Parse(error)
    }
}

// 现在可以在返回 Result<T, AppError> 的函数中对不同错误类型使用 ?
fn process_file(path: &str) -> Result<i32, AppError> {
    let content = std::fs::read_to_string(path)?; // io::Error → AppError::Io（自动转换）
    let number: i32 = content.trim().parse()?; // ParseIntError → AppError::Parse（自动转换）
    Ok(number)
}

fn demo_from_trait_conversion() {
    println!("--- 12. From trait 错误自动转换 ---");

    // 场景 1：文件不存在 → io::Error 自动转为 AppError::Io
    match process_file("nonexistent.txt") {
        Ok(num) => println!("  解析结果: {}", num),
        Err(e) => println!("  错误（IO → AppError）: {}", e),
    }

    // 场景 2：文件内容不是数字 → ParseIntError 自动转为 AppError::Parse
    std::fs::write("test_parse.txt", "not_a_number").unwrap();
    match process_file("test_parse.txt") {
        Ok(num) => println!("  解析结果: {}", num),
        Err(e) => println!("  错误（Parse → AppError）: {}", e),
    }

    // 场景 3：成功路径
    std::fs::write("test_parse.txt", "12345").unwrap();
    match process_file("test_parse.txt") {
        Ok(num) => println!("  解析成功: {}", num),
        Err(e) => println!("  错误: {}", e),
    }

    // 清理
    let _ = std::fs::remove_file("test_parse.txt");
    println!();
}

// ============================================================================
// 13. map_err：手动转换错误类型
// ============================================================================
// 当不想实现 From trait，或者同一错误类型需要映射到不同变体时，
// 使用 map_err 手动将错误从一种类型转换为另一种。
// 适合需要添加上下文信息的场景。

fn open_config(path: &str) -> Result<String, AppError> {
    // map_err: 手动将 io::Error 转换为更具语义的 AppError::NotFound
    std::fs::read_to_string(path)
        .map_err(|e| AppError::NotFound(format!("配置文件 '{}': {}", path, e)))
}

fn parse_port(s: &str) -> Result<u16, AppError> {
    // map_err: 添加上下文信息
    s.parse::<u16>()
        .map_err(|e| AppError::Validation(format!("端口号 '{}' 无效: {}", s, e)))
}

fn demo_map_err() {
    println!("--- 13. map_err 手动错误转换 ---");

    // map_err 将 io::Error 转为语义化的 NotFound
    match open_config("app.toml") {
        Ok(content) => println!("  配置内容: {}", content),
        Err(e) => println!("  {}", e),
    }

    // map_err 将 ParseIntError 转为带上下文的 Validation
    match parse_port("abc") {
        Ok(port) => println!("  端口: {}", port),
        Err(e) => println!("  {}", e),
    }

    match parse_port("8080") {
        Ok(port) => println!("  解析端口成功: {}", port),
        Err(e) => println!("  {}", e),
    }

    println!();
}

// ============================================================================
// 14. Box<dyn Error>：快速原型方案
// ============================================================================
// 所有实现了 Error trait 的类型都可以转换为 Box<dyn Error>。
// 优点：无需定义自定义错误类型，所有错误都能用 ? 传播。
// 缺点：类型被擦除，调用者无法 match 具体错误类型。
// 适合：原型开发、简单脚本、main 函数。
// 注意：main() 可以返回 Result<(), Box<dyn Error>>。

fn do_something() -> Result<i32, Box<dyn Error>> {
    // io::Error 和 ParseIntError 都能自动转换为 Box<dyn Error>
    let content = std::fs::read_to_string("number.txt")?; // io::Error → Box<dyn Error>
    let num: i32 = content.trim().parse()?; // ParseIntError → Box<dyn Error>
    Ok(num * 2)
}

fn demo_box_dyn_error() {
    println!("--- 14. Box<dyn Error> 快速原型 ---");

    // 文件不存在时
    match do_something() {
        Ok(val) => println!("  结果: {}", val),
        Err(e) => println!("  Box<dyn Error> 错误: {}", e),
    }

    // 文件内容不是数字时
    std::fs::write("number.txt", "hello").unwrap();
    match do_something() {
        Ok(val) => println!("  结果: {}", val),
        Err(e) => println!("  Box<dyn Error> 错误: {}", e),
    }

    // 成功路径
    std::fs::write("number.txt", "21").unwrap();
    match do_something() {
        Ok(val) => println!("  Box<dyn Error> 成功: {} (21 * 2)", val), // 42
        Err(e) => println!("  错误: {}", e),
    }

    let _ = std::fs::remove_file("number.txt");

    // 提示：main 函数也可以返回 Result
    // fn main() -> Result<(), Box<dyn Error>> { ... Ok(()) }
    println!("  提示：main() 可返回 Result<(), Box<dyn Error>>\n");
}

// ============================================================================
// 15. 错误链：遍历 source() 获取完整错误链路
// ============================================================================
// Error::source() 返回底层错误，可以递归遍历获取完整错误链。
// 这对日志记录和调试非常有用——了解错误的根本原因。

fn print_error_chain(err: &dyn Error) {
    println!("  错误: {}", err);
    let mut source = err.source();
    let mut depth = 1;
    while let Some(cause) = source {
        println!("  {}原因 {}: {}", "  ".repeat(depth), depth, cause);
        source = cause.source();
        depth += 1;
    }
}

fn demo_error_chain() {
    println!("--- 15. 错误链：source() 追踪根因 ---");

    // IO 错误链：AppError::Io 包装了 io::Error
    match process_file("nonexistent_file.txt") {
        Ok(_) => {}
        Err(ref e) => {
            println!("  [IO 错误链]");
            print_error_chain(e);
        }
    }

    // 解析错误链：AppError::Parse 包装了 ParseIntError
    std::fs::write("bad_number.txt", "xyz").unwrap();
    match process_file("bad_number.txt") {
        Ok(_) => {}
        Err(ref e) => {
            println!("  [Parse 错误链]");
            print_error_chain(e);
        }
    }

    let _ = std::fs::remove_file("bad_number.txt");
    println!();
}
