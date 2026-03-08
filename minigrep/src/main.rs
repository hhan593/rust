use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args); //加上分号变成语句而不是表达式
    // 程序的名字占据args的第一个元素 &args[0];
    // 所以从索引为 1 的参数开始，获取传入的参数
    let query = &args[1];
    // let filename = &args[2];

    // println!("Searching for {}", query);
    // println!("In file {}", filename);
    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query}");
    println!("In file {file_path}");

    println!("In file {file_path}");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

//提取命令行参数函数

struct Config {
    query: String,
    file_path: String,
}
fn parge_params(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();

    Config { query, file_path }
}
// main 函数中的责任应该被限制为：

// 使用参数值调用命令行解析逻辑
// 设置任何其他的配置
// 调用 lib.rs 中的 run 函数
// 如果 run 返回错误，则进行错误处理

// main函数处理程序运行，lib.rs处理业务逻辑
