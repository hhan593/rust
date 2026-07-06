// === 知识点总结 ===
// 1. PartialEq 特征 (Partial Equality)：用于为类型重载 `==` 和 `!=` 运算符。实现该特征只需提供 `eq` 方法。
// 2. 模式匹配类型一致性 (Type Match Consistency)：`match` 后面括号里的变量类型，必须与下面分支（arms）里的模式类型严格一致。不能用枚举去匹配结构体。
// 3. 匹配匹配工效学 (Match Ergonomics)：当你对引用（如 `&self.race`）进行 match 时，Rust 会自动处理引用级别的解构，允许你直接使用 `Race::White` 匹配，而不需要写成 `&Race::White`。
// 4. 自定义相等逻辑 (Custom Equality Logic)：虽然可以使用 `#[derive(PartialEq)]` 让编译器自动比较所有字段，但手动实现 `PartialEq` 可以让你自定义规则（例如这段代码中，只要 race 相同就算同一个 User，忽略了 id 和 name）。

//常见的trait Clone Copy Debug PartialEq // 知识点：代码注释。解释：说明本段代码重点演示常见的几个 Trait。
#[derive(Debug, Clone)] // 知识点：派生宏。解释：让编译器自动为 Race 生成调试打印 (Debug) 和深度拷贝 (Clone) 的实现代码。
enum Race { // 知识点：枚举定义。解释：定义一个枚举类型 Race，用于表示种族。
    White, // 知识点：枚举成员。解释：白色人种变体。
    Yellow, // 知识点：枚举成员。解释：黄色人种变体。
    Black, // 知识点：枚举成员。解释：黑色人种变体。
} // 知识点：作用域结束。解释：Race 枚举定义结束。

#[derive(
    Debug,
    Clone
)] //可以直接在这个给user实现Debug这个特质但是 // 知识点：派生宏。解释：自动为 User 实现 Debug 和 Clone。因为内部字段 String 支持 Clone，所以整体可以 Clone。
struct User { // 知识点：结构体定义。解释：定义一个名为 User 的结构体。
    id: u32, // 知识点：结构体字段。解释：无符号 32 位整数类型的 ID。
    name: String, //String没有实现Copy // 知识点：结构体字段。解释：String 存放在堆上，未实现 Copy 特征，因此 User 整体也无法派生 Copy 特征。
    race: Race, // 知识点：结构体字段。解释：嵌套使用上面定义的 Race 枚举类型。
} // 知识点：作用域结束。解释：User 结构体定义结束。

impl PartialEq for User { // 知识点：特征实现。解释：手动为 User 结构体实现 PartialEq，以支持 `==` 运算。
    fn eq(&self, other: &Self) -> bool { // 知识点：重写特征方法。解释：接收两个 User 实例的不可变引用，返回一个布尔值表示是否相等。
        match (&self.race, &other.race) { // 知识点：模式匹配、字段访问。【修复点】：原代码是 `match (self, other)` 类型不符。此处提取出两者的 race 字段的引用进行匹配。
            (Race::White, Race::White) => true, // 知识点：多变量模式匹配。解释：如果两者的 race 都是 White，返回 true。
            (Race::Yellow, Race::Yellow) => true, // 知识点：多变量模式匹配。解释：如果两者的 race 都是 Yellow，返回 true。
            (Race::Black, Race::Black) => true, // 知识点：多变量模式匹配。解释：如果两者的 race 都是 Black，返回 true。
            _ => false, // 知识点：通配符匹配 (catch-all)。解释：除了上述种族相同的情况外（即种族不同时），一律返回 false。
        } // 知识点：隐式返回。解释：match 表达式产生的值作为 eq 方法的返回值。
    } // 知识点：作用域结束。解释：eq 方法结束。
} // 知识点：作用域结束。解释：PartialEq 实现结束。

fn main() { // 知识点：程序入口。解释：主函数。
    let user = User { // 知识点：结构体实例化。解释：创建一个 User 实例绑定到 user 变量。
        id: 1, // 知识点：字段初始化。解释：id 为 1。
        name: String::from("zhangsan"), // 知识点：字段初始化。解释：分配堆内存创建 String。
        race: Race::White, // 知识点：字段初始化。解释：设置 race 为 Race::White。
    }; // 知识点：语句结束。解释：user 实例创建完毕。

    let user2 = user.clone(); // 知识点：Clone 方法调用。解释：深度克隆 user 产生一个完全独立的新实例 user2（因为有 String 字段，这里会重新分配堆内存）。

    println!("user2: {:?}", user2); // 知识点：格式化输出。解释：使用 Debug 特征打印 user2 的内部结构。
    println!("user:{:?}", user); // 知识点：格式化输出。解释：打印 user1。注意 user 在 clone 后依然存活，没有发生所有权转移。
    println!("user == user2: {}", user == user2); // 知识点：运算符重载调用。解释：这里使用 `==` 会自动触发我们上方手写的 `eq` 方法，由于两者 race 都是 White，因此输出 true。
} // 知识点：作用域结束。解释：main 函数执行完毕。