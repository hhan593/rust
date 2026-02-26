// 硬币匹配示例程序
// 演示 Rust 中 match 表达式的使用

// 主函数入口，演示硬币匹配功能
// 创建一个硬币实例并计算其面值，然后输出结果
fn main() {
    // 创建一个Penny硬币实例
    let coin = Coin::Quarter(UsState::Alabama);
    // 调用match_coin函数计算硬币面值
    let value = match_coin(coin);
    // 输出硬币的面值
    // println!("The value of the coin is {} cents.", value);

    let num = Some(4);
    let count = plus_one(num);
    // println!("count: {:?}", count);
    let none = plus_one(None);
    // println!("none: {:?}", none);

    // 通配符和占位符 通配模式和 _ 占位符
    let dice_doll = 9;

    // 使用match表达式匹配骰子点数，判断是否中奖,没中奖继续下一轮
    // match dice_doll {
    //     3 | 7 => println!("You win a free ride!"),
    //     _ => println!("Sorry, try again next time."),
    // }
    //等价于
    // match dice_doll {
    //     3 => println!("You win a free ride!"),
    //     7 => println!("You win a free ride!"),
    //     _=> println!("Sorry, try again next time."),

    //无论扔出的是什么都会输出，如果是3或7会输出中奖信息，否则输出未中奖信息
    // match dice_doll {
    //     3 | 7 => win(dice_doll),
    //     other => lose(dice_doll),
    // }
    //如果扔出来的点数是3或7，调用sorry函数输出惩罚信息；如果是其他点数，无事发生
    match dice_doll {
        3 | 7 => sorry(dice_doll),
        _ => (), // 这里使用了空元组()作为占位符，表示不执行任何操作
    }
}

/// 处理掷骰子失败情况的函数
///
/// 当骰子点数不是 3 或 7 时的处理函数
///
/// # 参数
/// * `dice_doll` - 掷出的骰子点数
fn sorry(dice_doll: u8) {
    println!("Sorry, you rolled a {} and lose!", dice_doll);
}

/// 处理掷骰子获胜情况的函数
///
/// 当骰子点数为 3 或 7 时的处理函数
///
/// # 参数
/// * `dice_doll` - 掷出的骰子点数
fn win(dice_doll: u8) {
    println!("You win a free ride! You rolled a {}.", dice_doll);
}

/// 处理掷骰子失败情况的函数（其他情况）
///
/// 当骰子点数不是 3 或 7 时的处理函数
///
/// # 参数
/// * `dice_doll` - 掷出的骰子点数
fn lose(dice_doll: u8) {
    println!("Sorry, try again next time. You rolled a {}.", dice_doll);
}

#[derive(Debug)]
// 美国州枚举定义
// 用于表示美国的不同州，特别用于25美分硬币的州标识
enum UsState {
    Alabama,    // 阿拉巴马州
    Alaska,     // 阿拉斯加州
    California, // 加利福尼亚州
}

#[derive(Debug)]
// 定义硬币枚举，包括不同面额的硬币
// 每种硬币对应不同的面值，Quarter还包含美国州的信息
enum Coin {
    Penny,            // 1美分硬币
    Nickel,           // 5美分硬币
    Dime,             // 10美分硬币
    Quarter(UsState), // 25美分纪念币，包含发行该硬币的美国州信息
}

/// 计算硬币面值的函数
///
/// 根据传入的硬币类型计算其对应的美分数值
/// 使用模式匹配(match)来处理不同的硬币类型
///
/// # 参数
/// * `coin` - 表示硬币类型的Coin枚举值
///
/// # 返回值
/// * `u8` - 硬币的面值，以美分为单位
fn match_coin(coin: Coin) -> u8 {
    // 使用match表达式对硬币类型进行模式匹配
    match coin {
        Coin::Penny => 1,  // 匹配1美分硬币，返回1
        Coin::Nickel => 5, // 匹配5美分硬币，返回5
        Coin::Dime => 10,  // 匹配10美分硬币，返回10
        Coin::Quarter(state) => {
            // 匹配25美分纪念币，提取其中的州信息
            // 输出该纪念币来自哪个州
            println!("State quarter from {state:?}!");
            // 返回25美分的面值
            25
        }
    }
}

/// 对Option<i32>值加一的函数
///
/// 如果输入是Some(i)，则返回Some(i+1)，否则返回None
///
/// # 参数
/// * `x` - 要加一的Option<i32>值
///
/// # 返回值
/// * `Option<i32>` - 加一后的结果，如果原值为None则返回None
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// 总结：Rust的匹配是穷尽的，必须覆盖所有的可能情况。 如果没有覆盖所有情况，编译器会报错，提示缺少匹配分支。
