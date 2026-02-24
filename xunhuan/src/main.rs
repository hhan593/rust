/// 主函数演示了 loop 循环和标签控制流程的使用
///
/// 这个函数展示了嵌套循环和使用标签来控制特定循环的行为，
/// 包括如何从内层循环跳出到外层循环。
///
mod test;
fn main() {
    // 输出初始信息
    println!("Hello, world!");
    let mut x = 1;
    let arr = [1, 2, 3, 4, 5];

    // 外层循环，带有标签 conut_up（注意：标签名拼写错误，应该是 count_up）
    // 'count_up: loop {
    //     println!("x = {x}");
    //     let mut y = 10;

    //     // 内层循环处理变量 y 的递减逻辑
    //     loop {
    //         println!("y = {y}");
    //         if y == 2 {
    //             break;
    //         }
    //         if x == 5 {
    //             // 当 x 等于 5 时，跳出外层循环
    //             break 'count_up;
    //         }
    //         y -= 1;
    //     }
    //     x += 1;
    // }

    while x < 10 {
        println!("x = {x}");
        x += 1;
    }
    for i in arr {
        println!("i = {i}");
    }

    test::transform_temperatures();
}
