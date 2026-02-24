// 导入标准库的输入输出模块
use std::io;

// 主函数，程序的入口点
fn main() {
    // 打印游戏欢迎信息
    println!("Guess the number!");

    // 提示用户输入猜测的数字
    println!("Please input your guess.");

    // 创建一个可变的空字符串用于存储用户的输入
    let mut guess = String::new(); // mut表示变量是可变的,如果没有mut则表示不可变变量

    // 从标准输入读取一行文本，并存储到guess变量中
    // expect方法用于处理可能的错误，如果读取失败则会打印错误信息并终止程序
    io::stdin()
        .read_line(&mut guess)
        // 读取返回的值是有两种
        // ok 成功
        // Error 失败
        .expect("阅读数据失败");

    // 打印用户输入的内容
    println!("You guessed: {}", guess);
}

// 这些注释解释了代码的主要功能、变量用途和操作流程，帮助理解程序的执行逻辑。
