// 主函数演示了 Rust 中的条件语句（if/else）和模式匹配（match）
fn main() {
    let source = 70;
    if source > 90 {
        println!("Good!!!");
    } else if source > 60 {
        println!("You are OK!");
    } else {
        println!("Bad!!!");
    }

    let mut scores = 70;
    let msg = if scores > 50 { "old" } else { "young" };
    println!("You are {msg}");
    change(&mut scores);
    println!("scores = {scores}");

    // 使用 match 模式匹配特定值
    match scores {
        80 => println!("80"),
        90 => println!("90"),
        _ => println!("Some else"),
    }
    // 使用 match 模式匹配,匹配范围

    // 使用 match 模式匹配数值范围
    match scores {
        25..=50 => println!("25 ... 50"),
        51..=100 => println!("51 ... 100"),
        _ => println!("Some else"),
    }

    // 使用 match 模式匹配,匹配多个条件

    // 使用 match 模式匹配多个可能的值
    match scores {
        25 | 50 | 75 => print!("25 or 50 or 75"),
        100 | 200 => println!("100 or 200"),
        _ => println!("Some else"),
    }
}

/// 修改传入的整数值
///
/// # 参数
///
/// * `source` - 需要被修改的可变整数引用
///
/// # 返回值
///
/// 此函数没有返回值
fn change(source: &mut i32) {
    *source = 80;
}
