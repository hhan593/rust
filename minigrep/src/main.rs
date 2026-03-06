use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args); //加上分号变成语句而不是表达式
    // 程序的名字占据args的第一个元素 &args[0];
    // 所以从索引为 1 的参数开始，获取传入的参数
    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);
}
