mod conflow;
use conflow::test::conflow;
fn main() {
    conflow();

    let y = {
        let x = 31;
        x + 1
    };
    println!("{}", y);
    let z = plus_one(5);
    println!("{}", z);

    println!(
        "{}",
        plus_one({
            let y = 6;
            y + 4
        })
    )
}

fn plus_one(x: i32) -> i32 {
    // return x + 2;
    x + 2 //函数体最后一个表达式的值自动作为返回值
    // x+2;加分号后变成语句，不产生值 函数实际返回 () (unit类型) 与声明的 -> i32 不匹配 → 编译错误
}
//{} 代码块的最后一个表达式的值会成为整个代码块的值,返回类型匹配 -> i32
//| 代码 | 类型 | 返回值 |
// |------|------|--------|
// | `x + 2` | 表达式 | 返回计算结果 (`i32`) |
// | `x + 2;` | 语句 | 返回 `()` (unit类型) |
