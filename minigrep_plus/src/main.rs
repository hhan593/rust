use std::env; // 引入环境库，用于处理命令行参数
use std::process; // 引入进程库，用于退出程序
use minigrep_plus::{run,Config};
fn main() {
    // 1. 收集命令行参数到一个字符串向量中
    // args[0] 是程序名，args[1] 是查询字符串，args[2] 是文件名

    // 2. 调用 Config 的构建函数解析参数
    // unwrap_or_else 是一个闭包处理：如果成功则返回 Config，如果失败则执行闭包逻辑
    let config =  Config::build(env::args()).unwrap_or_else(|err| {
        // eprintln! 将错误信息打印到“标准错误流 (stderr)”而不是“标准输出流 (stdout)”
        eprintln!("解析参数时出现问题: {}", err);
        // 停止程序并返回退出状态码 1（表示非正常退出）
        process::exit(1);
    });

    println!("正在搜索: {}", config.query);
    println!("目标文件: {}", config.filename);

    // 3. 执行核心逻辑
    // 使用 if let 处理 run 函数可能返回的错误
    if let Err(e) = run(config) {
        eprintln!("应用程序运行出错: {}", e);
        process::exit(1);
    }
}

// 业务逻辑函数
// Box<dyn Error> 意味着函数可以返回任何实现了 Error trait 的错误类型

//核心知识点补充：
// eprintln! vs println!：

// 在开发命令行工具时，正常的输出应该去 stdout（标准输出），而报错信息应该去 stderr（标准错误）。这样用户在重定向输出（如 cargo run > output.txt）时，错误信息依然能显示在屏幕上，而不会被写进文件。

// unwrap_or_else：

// 这是一种比 unwrap() 更优雅的错误处理方式。它允许你在遇到 Err 时定义自定义的清理逻辑或退出逻辑，而不是让程序直接崩溃（panic）。

// Box<dyn Error>：

// 这是一个“错误对象”。由于 run 函数中可能发生不同类型的错误（比如文件不存在的 IO 错误，或者未来可能添加的其他错误），使用 Box<dyn Error> 可以让函数支持返回多种不同类型的错误，只要它们都遵循 Error 协议。

// ? 运算符：

// 这是 Rust 极其好用的特性。它相当于一个简写：如果结果是 Ok，就把里面的值给变量；如果是 Err，就直接从当前函数 return 这个错误。
