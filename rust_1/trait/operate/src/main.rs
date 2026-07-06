// === 知识点总结 ===
// 1. 运算符重载 (Operator Overloading)：使用 `std::ops::Add` 特征可以重载 `+` 运算符，使得自定义结构体可以使用加号。
// 2. 关联类型 (Associated Types)：在 `Add` 特征内部有 `type Output;`，用于指定加法运算结果的具体类型。
// 3. 泛型约束 (Trait Bounds)：`T: Add<Output=T> + Copy` 要求类型 T 必须同时支持加法运算，并且实现了 Copy 特征。
// 4. Copy 特征 (Copy Trait)：Rust 的标记特征。实现了 Copy 的类型在赋值或传参时会按位复制，而不是转移所有权。数字类型自带 Copy，而 String 没有。增加 Copy 约束可以有效将 String 排除在外。
// 5. 派生宏 (Derive Macro)：`#[derive(Debug)]` 让编译器自动为结构体生成格式化调试输出的代码。

use std::ops::Add;
// 知识点：模块引入。解释：从标准库引入 Add 特征，用于后续的加号运算符重载。

#[derive(Debug)] // 知识点：派生宏。解释：自动为 Point 结构体实现 Debug 特征，以便能够使用 `{:?}` 打印。
struct Point<T> { // 知识点：泛型结构体定义。解释：定义一个包含泛型 T 的二维坐标点。
    x: T, // 知识点：结构体字段。解释：坐标的 x 值，类型为 T。
    y: T, // 知识点：结构体字段。解释：坐标的 y 值，类型为 T。
} // 知识点：作用域结束。解释：结构体定义结束。

impl<T> Add for Point<T> // 知识点：泛型特征实现。解释：为泛型结构体 Point<T> 实现 Add 特征。
where // 知识点：where 子句。解释：用于分离泛型约束，提高可读性。
    T: Add<Output=T> + Copy, // 知识点：多重泛型约束。解释：要求 T 必须支持 T+T=T 的加法，并且必须是 Copy 类型（这一步显式排除了 String）。
{ // 知识点：特征实现体开始。
    type Output = Point<T>; // 知识点：关联类型定义。解释：明确告诉编译器，两个 Point 相加后，返回的类型依然是 Point<T>。

    fn add(self, rhs: Self) -> Self::Output { // 知识点：特征方法重写。解释：实现 add 方法，接收两个 Point 的实例（self 和 rhs），返回一个全新的 Point。
        Point { // 知识点：结构体实例化与隐式返回。解释：创建一个新的 Point 实例作为计算结果返回。
            x: self.x + rhs.x, // 知识点：字段运算。解释：将两个点的 x 坐标相加。因为 T 约束了 Add，所以这里能用 `+`。
            y: self.y + rhs.y, // 知识点：字段运算。解释：将两个点的 y 坐标相加。
        } // 知识点：实例构造结束。
    } // 知识点：作用域结束。解释：add 方法结束。
} // 知识点：作用域结束。解释：Add 特征实现结束。

fn main() { // 知识点：程序入口。解释：主函数开始。
    let c1 = Point { x: 1, y: 2 }; // 知识点：结构体实例化。解释：创建一个 i32 类型的 Point。
    let c2 = Point { x: 1, y: 2 }; // 知识点：结构体实例化。解释：再创建一个 i32 类型的 Point。

    let c3 = c1 + c2; // 知识点：运算符重载调用。解释：使用 `+` 触发我们自定义的 add 方法。i32 满足 Add 和 Copy 约束，编译通过。
    println!("{:?}", c3); // 知识点：调试格式化输出。解释：打印相加后的结果 `Point { x: 2, y: 4 }`。

    // === 下面这部分代码现在不仅会因为 Add 报错，还会因为没有实现 Copy 而报错 ===
    /* let s1 = Point { // 知识点：注释代码。解释：如果取消注释，编译器会提示 String 未实现 Copy，也未实现 Add<String>。
        x: String::from("hello"),
        y: String::from("world"),
    };
    let s2 = Point {
        x: String::from("1111"),
        y: String::from("123"),
    };
    let s = s1 + s2;
    println!("{:?}", s);
    */
} // 知识点：作用域结束。解释：main 函数结束。