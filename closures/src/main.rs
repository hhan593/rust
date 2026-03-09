// #[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
// enum ShirtColor {
//     Red,
//     Green,
//     Blue,
// }

// struct Inventory {
//     shirts: Vec<ShirtColor>,
// }

// impl Inventory {
//     // 这里的 giveaway 函数是闭包应用的核心场景
//     fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
//         // unwrap_or_else 接收一个“闭包”作为参数
//         // || self.most_stocked() 就是闭包：
//         // 如果 user_preference 是 Some，就直接用里面的值；
//         // 如果是 None，才会执行 || 后的代码去计算库存最多的颜色。
//         user_preference.unwrap_or_else(|| self.most_stocked())
//     }

//     fn most_stocked(&self) -> ShirtColor {
//         if self.shirts.is_empty() {
//             return ShirtColor::Red;
//         }

//         let mut red = 0;
//         let mut green = 0;
//         let mut blue = 0;

//         for color in &self.shirts {
//             match color {
//                 ShirtColor::Red => red += 1,
//                 ShirtColor::Green => green += 1,
//                 ShirtColor::Blue => blue += 1,
//             }
//         }

//         let max = red.max(green).max(blue);

// 返回逻辑：谁最大就给谁，优先级是 蓝 > 绿 > 红
//         if max == blue {
//             ShirtColor::Blue
//         } else if max == green {
//             ShirtColor::Green
//         } else {
//             ShirtColor::Red
//         }
//     }
// }

// fn main() {
//     let store = Inventory {
//         shirts: vec![ShirtColor::Red, ShirtColor::Green, ShirtColor::Blue],
//     };

// 情况1：用户说了他想要红色（Some）
//     let user_pref1 = Some(ShirtColor::Red);
//     let giveaway1 = store.giveaway(user_pref1);
//     println!("用户指定了 {:?}, 最终拿到: {:?}", user_pref1, giveaway1);

// 情况2：用户没主见（None），触发了闭包执行逻辑
//     let user_pref2 = None;
//     let giveaway2 = store.giveaway(user_pref2);
//     println!(
//         "用户没选 ({:?}), 自动计算库存最多的颜色: {:?}",
//         user_pref2, giveaway2
//     );
// }

// ```

// ---

// ### 2. 通俗易懂讲解：什么是闭包？

// 你可以把**闭包**想象成一个**“随叫随到的私人秘书”**，或者一个**“外卖订单”**。

// #### A. 它和普通函数的区别

// * **普通函数**：像是一道**现成的菜**。不管你吃不吃，它就在那里，参数必须提前定死。
// * **闭包**：像是**一份菜谱或订单**。你可以先把它写好（定义），放在一边。只有当你真正饿了（调用）的时候，它才会跑进厨房帮你把菜做出来。

// #### B. 为什么这里要用闭包？（性能与“懒加载”）

// 在上面的代码中：
// `user_preference.unwrap_or_else(|| self.most_stocked())`

// * 如果用户已经选了颜色（`Some`），那我们直接把颜色给他就行了。
// * **如果不使用闭包**，而是直接调用函数：
// `unwrap_or(self.most_stocked())`
// 即使系统不需要计算库存（用户已经选了颜色），程序也会强制运行一次 `most_stocked()`。如果库存里有 100 万件衣服，这种浪费就很可怕。
// * **使用了闭包 `|| ...**`：
// 程序会说：“我先记着这个逻辑，如果 `Option` 确实是 `None`，我再动手算。” 这就是程序员常说的 **Lazy Evaluation（惰性求值/懒加载）**。

// #### C. 闭包的三个核心特征

// 1. **匿名性**：它没有名字，通常长这样 `|参数| { 逻辑 }`。
// 2. **捕获环境**：这是它最厉害的地方。闭包可以“偷”走它定义位置周围的变量。比如在 `giveaway` 里，闭包能直接访问 `self`，而不需要你额外传参。
// 3. **语法灵活**：
// * 如果只有一行，大括号 `{}` 可以省略。
// * 参数类型通常可以自动推导（不用像函数那样写 `i32` ）。

// ---

// ### 总结

// 闭包就是一段**被打包好的逻辑**，它安静地躺在那里。等到你真正需要它运行的那一刻，它才会被激活。

// **你想看看闭包如何“捕获”外部变量（就像照相机拍照一样留住那一刻的变量值）的代码例子吗？**

//闭包的多种写法

fn main() {
    let sqare = |x| x * x; //c此时不知道他的类型，也没法进行推断
    println!("sqaer is {}", sqare(5)); //这是使用了5 ，rust就自己推断出了x的类型为i32

    //类型注解的闭包

    let add = |x: u32, y: u32| -> u32 { x + y };
    println!("add is {}", add(9, 3))
}
