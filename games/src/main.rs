use ::std::io;
use rand::Rng;
use std::cmp::Ordering;

/// 主函数
///
/// 程序入口点，负责执行猜数字游戏的主要逻辑
/// 该函数不接受任何参数，也没有返回值
///
/// 功能包括：
/// - 显示游戏提示信息
/// - 读取用户输入
/// - 显示用户猜测结果
// 外部库位crate引入
fn main() {
    // 显示游戏开始提示和输入指令
    println!("guess number!");
    let guess_num: u32 = rand::rng().random_range(1..=100);

   loop{ println!("Please input your guess.");

    // 创建可变字符串变量用于存储用户输入  mut可变变量  不加mut的话是不不可变的
    let mut guess: String = String::new();

    // 从标准输入读取用户输入的一行数据
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    // 输出用户猜测的内容
    println!("you guess: {}", guess);
    match guess.cmp(&guess_num) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }}
}
