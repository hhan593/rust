fn main() {
    // --- 1. 基础 if-else 语句 ---
    let age = 50;
    if age < 50 {
        println!("You are young");
    } else {
        println!("You are old");
    }

    // --- 2. if 作为表达式 (Expression) ---
    // Rust 的 if 是表达式，意味着它可以返回值，而不仅仅是控制流语句
    let scores = 70;
    if scores > 90 {
        println!("Good!!!");
    } else if scores > 60 {
        println!("You are OK!");
    } else {
        println!("Bad!!!");
    }

    // 将 if 表达式的结果赋值给变量 msg
    // 注意：if 和 else 分支返回的类型必须一致（这里都是 &str）
    let msg = if age > 50 { "old" } else { "young" };
    println!("You are {msg}");

    // --- 3. 基础 match 模式匹配 ---
    let num = 90;
    match num {
        80 => println!("80"),       // 如果 num 等于 80
        90 => println!("90"),       // 如果 num 等于 90
        _ => println!("Some else"), // 通配符，匹配其他所有情况
    }

    // --- 4. match 范围匹配 ---
    // 使用 ..= 进行闭区间范围匹配
    match num {
        25..=50 => println!("25 ... 50"),
        51..=100 => println!("51 ... 100"),
        _ => println!("Some else"),
    }

    // --- 5. match 多条件匹配 ---
    // 使用 | 表示“或”逻辑
    match num {
        25 | 50 | 75 => print!("25 or 50 or 75"),
        100 | 200 => println!("100 or 200"),
        _ => println!("Some else"),
    }

    // --- 6. match 守卫 ---
    // 使用 if 关键字在模式中添加额外的布尔条件
    match num {
        x if x < 60 => println!("bad"),   // 匹配任何值赋给 x，但前提是 x < 60
        x if x == 60 => println!("luck"), // 匹配任何值赋给 x，但前提是 x == 60
        _ => println!("Some else"),
    }

    // --- 7. match 作为表达式 ---
    // match 也是表达式，可以返回值
    let num = 60; // 重新定义 num 为 60
    let res = match num {
        x if x < 60 => "bad".to_owned(),   // 返回 String 类型
        x if x == 60 => "luck".to_owned(), // 返回 String 类型
        _ => "Some else".to_owned(),       // 返回 String 类型
    };
    println!("res value : {res}");
}
