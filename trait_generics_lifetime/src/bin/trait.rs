use std::fmt::Display;
use std::ops::Add; // 1. 引入 Add trait

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// 2. 添加约束：
// Y: Display -> 为了能在 println 中使用 {}
// Y: Add<Output = Y> -> 为了能让 Y + Y，且结果类型也是 Y
// 注意：因为是对 &self.x 和 &self.y 操作，所以实际上是要求 &Y 能相加
// 对于 i32 等内置类型，&i32 + &i32 是支持的，其 Output 是 i32 (即 Y)
impl<Y> Point<Y>
where
    Y: Display + Copy + Add<Output = Y>,
{
    fn add(&self) -> Y {
        // 现在编译器知道 Y 可以相加了
        // 对于 i32, &i32 + &i32 返回 i32
        let sum = self.x + self.y;

        // 如果必须用引用相加 (有些类型只实现了引用的加法)，写法如下：
        // let sum = self.x + self.y; // 如果 Y 是 Copy 的，直接值相加最方便
        // 或者如果你的类型只支持引用相加：
        // let sum = self.x + self.y; // 这里的 + 会根据 trait 自动解引用或引用

        // 修正：标准库中 i32 的实现是 Add for &i32 with &i32 returning i32
        // 但更通用的写法通常要求 Y: Copy，然后直接 self.x + self.y
        // 如果只实现了引用的加法，约束应该是：for<'a> &'a Y: Add<&'a Y, Output = Y>
        // 为了简单起见，针对数字类型，我们通常加上 Copy 约束并直接相加

        println!("add {} + {} = {}", self.x, self.y, sum);
        sum // 3. 返回计算结果
    }
}

// 上面的 impl 对于简单的数字类型，更严谨的写法其实是约束引用的加法：
/*
impl<Y> Point<Y>
where
    Y: Display,
    for<'a> &'a Y: Add<&'a Y, Output = Y>,
{
    fn add(&self) -> Y {
        let sum = &self.x + &self.y;
        println!("add {} + {} = {}", self.x, self.y, sum);
        sum
    }
}
*/
// 鉴于你是初学者，使用 Copy + 直接值相加 或者 专门针对 i32 理解会更简单。
// 下面给出一个针对当前代码最直接的修复版本（使用引用相加的约束）：

//1.6 const 泛型（常量泛型）

fn display_arr<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("{:?}", arr);
}

//使用泛型不会有任何运行时性能损失。代价是编译时间增加和二进制体积可能变大。

fn main() {
    let p = Point { x: 5, y: 10 };
    // 现在 add() 返回了一个 i32 (Y)，且 i32 实现了 Debug，所以 {:?} 可以工作
    let result = p.add();
    println!("Result is: {:?}", result);

    let arr = [1, 2, 3];
    display_arr(arr);
}
